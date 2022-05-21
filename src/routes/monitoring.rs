use axum::response::Html;
use axum::response::IntoResponse;
use opentelemetry_prometheus::Encoder;
use opentelemetry_prometheus::TextEncoder;

use crate::PROMETHEUS;

pub async fn health_check() -> impl IntoResponse {
    Html("<p>200 OK</p>")
}

pub async fn metrics() -> impl IntoResponse {
    let mut output = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = PROMETHEUS.registry().gather();
    encoder.encode(&metric_families, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}
