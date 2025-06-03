mod config;
mod loader;
mod types;
mod ws_client;

use config::LoaderConfig;

#[tokio::main]
async fn main() {
    let config = LoaderConfig::from_file("loader.toml");

    println!("Loaded config: {:?}", config);
    loader::run_loader(config).await;
}
