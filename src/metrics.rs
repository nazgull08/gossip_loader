use metrics_exporter_prometheus::PrometheusBuilder;
use std::{net::SocketAddr, sync::Once};
use log::info;

static INIT: Once = Once::new();

/// Initialize Prometheus exporter on given address (from config)
pub fn init_metrics(addr: SocketAddr) {
    INIT.call_once(|| {
        info!("Starting Prometheus exporter on http://{}/metrics", addr);

        PrometheusBuilder::new()
            .with_http_listener(addr)
            .install()                         // ← поднимает HTTP-сервер
            .expect("failed to install Prometheus exporter");
    });
}
