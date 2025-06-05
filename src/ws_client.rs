use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;
use log::{info, error, debug};
use metrics::{counter, histogram};
use tokio::time::{Duration, sleep};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use crate::types::request::{EventType, GuardianTopic, WsServerRequest};

pub async fn run_ws_client(
    addr: &str,
    json_payload: &str,
    interval_ms: u64,
    client_id: usize,
    duration_secs: u64,
    global_sent: Arc<AtomicU64>,
) {
    let (ws_stream, _) = match connect_async(addr).await {
        Ok(ok) => ok,
        Err(err) => {
            error!("Client {}: Connection failed: {}", client_id, err);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    let base_data: Value = match serde_json::from_str(json_payload) {
        Ok(v) => v,
        Err(e) => {
            error!("Client {}: Invalid JSON payload: {}", client_id, e);
            return;
        }
    };

    let start_time = tokio::time::Instant::now();
    let max_duration = Duration::from_secs(duration_secs);

    loop {
        if start_time.elapsed() >= max_duration {
            info!("Client {}: Duration expired, stopping send loop", client_id);
            break;
        }

        let uuid = Uuid::new_v4();
        let request = WsServerRequest {
            event_type: EventType::Request,
            id: uuid.to_string(),
            topic: GuardianTopic::VaultOpen,
            data: base_data.clone(),
        };

        let message = match serde_json::to_string(&request) {
            Ok(s) => Message::text(s),
            Err(e) => {
                error!("Client {}: Serialization error: {}", client_id, e);
                counter!("messages_failed_total", 1, "client" => client_id.to_string());
                break;
            }
        };

        let send_start = std::time::Instant::now();
        counter!("messages_sent_total", 1, "client" => client_id.to_string());
        global_sent.fetch_add(1, Ordering::Relaxed);

        if let Err(e) = write.send(message).await {
            error!("Client {}: Failed to send: {}", client_id, e);
            counter!("messages_failed_total", 1, "client" => client_id.to_string());
            break;
        }

        info!("Client {}: Sent message with ID {}", client_id, uuid);

        match read.next().await {
            Some(Ok(reply)) => {
                counter!("messages_received_total", 1, "client" => client_id.to_string());
                histogram!(
                    "latency_ms",
                    send_start.elapsed().as_millis() as f64,
                    "client" => client_id.to_string()
                );
                debug!("Client {}: Received: {:?}", client_id, reply);
            }
            Some(Err(e)) => {
                error!("Client {}: Receive error: {}", client_id, e);
                counter!("messages_failed_total", 1, "client" => client_id.to_string());
                break;
            }
            None => {
                error!("Client {}: Connection closed", client_id);
                break;
            }
        }

        sleep(Duration::from_millis(interval_ms)).await;
    }

    info!("Client {}: Finished sending. Remaining in passive mode.", client_id);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(_) => {
                counter!("messages_received_total", 1, "client" => client_id.to_string());
            }
            Err(e) => {
                error!("Client {}: Passive receive error: {}", client_id, e);
                break;
            }
        }
    }

    info!("Client {}: Fully done", client_id);
}
