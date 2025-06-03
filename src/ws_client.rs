use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::types::request::{EventType, GuardianTopic, WsServerRequest};

pub async fn run_ws_client(
    addr: &str,
    json_payload: &str,
    interval_ms: u64,
    client_id: usize,
) {
    // Connect to WebSocket server
    let (ws_stream, _) = match connect_async(addr).await {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("[client {client_id}] Connection failed: {err}");
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    // Pre-parse JSON payload once
    let base_data: Value = match serde_json::from_str(json_payload) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[client {client_id}] Invalid JSON payload: {e}");
            return;
        }
    };

    loop {
        // Generate fresh UUID and request on each send
        let uuid = uuid::Uuid::new_v4();
        let request = WsServerRequest {
            event_type: EventType::Request,
            id: uuid.to_string(),
            topic: GuardianTopic::VaultOpen,
            data: base_data.clone(), // Clone base template for each send
        };

        let message = match serde_json::to_string(&request) {
            Ok(s) => Message::text(s),
            Err(e) => {
                eprintln!("[client {client_id}] Serialization error: {e}");
                break;
            }
        };

        // Send the message
        if let Err(e) = write.send(message).await {
            eprintln!("[client {client_id}] Failed to send: {e}");
            break;
        }

        println!("[client {client_id}] Sent message with ID: {uuid}");

        // Await and print one response
        match read.next().await {
            Some(Ok(reply)) => {
                println!("[client {client_id}] Received: {:?}", reply);
            }
            Some(Err(e)) => {
                eprintln!("[client {client_id}] Error receiving: {e}");
                break;
            }
            None => {
                eprintln!("[client {client_id}] Connection closed");
                break;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    }
}
