use crate::config::{LoaderConfig, LoadPattern};
use crate::ws_client::run_ws_client;
use log::info;
use std::{fs, sync::{Arc, atomic::{AtomicU64, Ordering}}};
use tokio::time::{sleep, Duration};

pub async fn run_loader(config: LoaderConfig) {
    let raw_json = fs::read_to_string(&config.server.json_path)
        .expect("Failed to read JSON file");
    let shared_json = Arc::new(raw_json);
    let global_sent = Arc::new(AtomicU64::new(0));
    let start_time = std::time::Instant::now();

    match config.load.pattern {
        LoadPattern::Steady => run_steady(config, shared_json, Arc::clone(&global_sent)).await,
        LoadPattern::Burst => run_burst(config, shared_json, Arc::clone(&global_sent)).await,
        LoadPattern::RampUp => run_ramp_up(config, shared_json, Arc::clone(&global_sent)).await,
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let sent = global_sent.load(Ordering::Relaxed);
    let rps = if elapsed > 0.0 { sent as f64 / elapsed } else { 0.0 };

    info!(
        "📊 Summary: sent {} messages in {:.2} seconds ({:.2} msg/sec)",
        sent, elapsed, rps
    );
}

async fn run_steady(config: LoaderConfig, json: Arc<String>, global_sent: Arc<AtomicU64>) {
    let mut handles = Vec::new();
    for client_id in 0..config.load.clients {
        let json = Arc::clone(&json);
        let addr = config.server.connect_addr.clone();
        let interval = config.load.interval_ms;
        let global_sent = Arc::clone(&global_sent);

        info!("🚀 Spawning steady WS client {client_id}");

        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, client_id, config.load.duration_secs, global_sent).await;
        });

        handles.push(handle);
        sleep(Duration::from_millis(10)).await;
    }

    sleep(Duration::from_secs(config.load.duration_secs)).await;
    info!("✅ Steady load complete");
}

async fn run_burst(config: LoaderConfig, json: Arc<String>, global_sent: Arc<AtomicU64>) {
    let addr = config.server.connect_addr.clone();
    let interval = config.load.interval_ms;
    let clients = config.load.clients;
    let mut handles = Vec::new();

    info!("🚀 Burst mode: sending {clients} messages at once");

    for client_id in 0..clients {
        let json = Arc::clone(&json);
        let addr = addr.clone();
        let global_sent = Arc::clone(&global_sent);

        info!("🚀 Spawning burst WS client {client_id}");

        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, client_id, config.load.duration_secs, global_sent).await;
        });

        handles.push(handle);
    }

    sleep(Duration::from_secs(config.load.duration_secs)).await;
    info!("✅ Burst load complete");
}

async fn run_ramp_up(config: LoaderConfig, json: Arc<String>, global_sent: Arc<AtomicU64>) {
    let addr = config.server.connect_addr.clone();
    let base_interval = config.load.interval_ms;
    let clients = config.load.clients;
    let duration = config.load.duration_secs;
    let delay_per_client = duration * 1000 / clients as u64;

    info!("🚀 Ramp-up mode: adding {clients} clients every {delay_per_client}ms");

    let mut handles = Vec::new();
    for client_id in 0..clients {
        let json = Arc::clone(&json);
        let addr = addr.clone();
        let interval = base_interval / 2 + client_id as u64;
        let global_sent = Arc::clone(&global_sent);

        info!("🚀 Spawning ramp-up WS client {client_id} (interval {interval}ms)");

        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, client_id, config.load.duration_secs, global_sent).await;
        });

        handles.push(handle);
        sleep(Duration::from_millis(delay_per_client)).await;
    }

    sleep(Duration::from_secs(duration)).await;
    info!("✅ Ramp-up load complete");
}
