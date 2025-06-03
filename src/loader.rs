use crate::config::{LoaderConfig, LoadPattern};
use crate::ws_client::run_ws_client;
use std::fs;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn run_loader(config: LoaderConfig) {
    // Read message payload from file
    let raw_json = fs::read_to_string(&config.server.json_path)
        .expect("Failed to read JSON file");
    let shared_json = Arc::new(raw_json);

    // Spawn clients based on load pattern
    match config.load.pattern {
        LoadPattern::Steady => run_steady(config, shared_json).await,
        LoadPattern::Burst => run_burst(config, shared_json).await,
        LoadPattern::RampUp => run_ramp_up(config, shared_json).await,
    }
}

async fn run_steady(config: LoaderConfig, json: Arc<String>) {
    let mut handles = Vec::new();
    for i in 0..config.load.clients {
        let json = Arc::clone(&json);
        let addr = config.server.connect_addr.clone();
        let interval = config.load.interval_ms;

        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, i).await;
        });

        handles.push(handle);
        sleep(Duration::from_millis(10)).await;
    }

    sleep(Duration::from_secs(config.load.duration_secs)).await;
    println!("Steady load complete");
}

async fn run_burst(config: LoaderConfig, json: Arc<String>) {
    let addr = config.server.connect_addr.clone();
    let interval = config.load.interval_ms;
    let clients = config.load.clients;

    println!("Burst mode: sending {} messages at once", clients);

    let mut handles = Vec::new();
    for i in 0..clients {
        let json = Arc::clone(&json);
        let addr = addr.clone();
        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, i).await;
        });
        handles.push(handle);
    }

    sleep(Duration::from_secs(config.load.duration_secs)).await;
    println!("Burst complete");
}

async fn run_ramp_up(config: LoaderConfig, json: Arc<String>) {
    let addr = config.server.connect_addr.clone();
    let interval = config.load.interval_ms;
    let clients = config.load.clients;
    let duration = config.load.duration_secs;

    let delay_per_client = duration * 1000 / clients as u64;

    println!("Ramp-up mode: adding clients every {delay_per_client}ms");

    let mut handles = Vec::new();
    for i in 0..clients {
        let json = Arc::clone(&json);
        let addr = addr.clone();
        let interval = interval / 2 + i as u64; // simulate increasing pressure
        let handle = tokio::spawn(async move {
            run_ws_client(&addr, &json, interval, i).await;
        });

        handles.push(handle);
        sleep(Duration::from_millis(delay_per_client)).await;
    }

    sleep(Duration::from_secs(duration)).await;
    println!("📈 Ramp-up complete");
}
