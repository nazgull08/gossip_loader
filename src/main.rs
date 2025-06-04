mod config;
mod loader;
mod metrics;
mod types;
mod ws_client;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = config::LoaderConfig::from_file("loader.toml");
    let metrics_addr: SocketAddr = config
        .server
        .metrics_addr
        .parse()
        .expect("Invalid metrics_addr in config");

    metrics::init_metrics(metrics_addr);

    log::info!("Loaded config: {:?}", config);
    loader::run_loader(config).await;

    tokio::signal::ctrl_c().await?;
    Ok(())
}
