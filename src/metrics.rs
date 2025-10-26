use std::time::Duration;

use actix_web::{HttpResponse, Responder};
use once_cell::sync::Lazy;
use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge_vec, Encoder, HistogramVec,
    IntCounterVec, IntGaugeVec, TextEncoder,
};

static HTTP_REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests processed",
        &["method", "endpoint", "status"]
    )
    .expect("failed to register http_requests_total counter")
});

static HTTP_REQUEST_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request latency in seconds",
        &["method", "endpoint", "status"],
        vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
    )
    .expect("failed to register http_request_duration_seconds histogram")
});

static HTTP_REQUESTS_IN_FLIGHT: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        "http_requests_in_flight",
        "In-flight HTTP requests",
        &["method", "endpoint"]
    )
    .expect("failed to register http_requests_in_flight gauge")
});

/// Record that a request has started so we can capture concurrent request counts.
pub fn track_request_start(method: &str, endpoint: &str) {
    HTTP_REQUESTS_IN_FLIGHT
        .with_label_values(&[method, endpoint])
        .inc();
}

/// Record request completion metrics (counter, latency histogram, in-flight gauge).
pub fn track_request_result(method: &str, endpoint: &str, status: u16, duration: Duration) {
    let status_label = status.to_string();

    HTTP_REQUESTS_IN_FLIGHT
        .with_label_values(&[method, endpoint])
        .dec();

    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method, endpoint, &status_label])
        .inc();

    HTTP_REQUEST_DURATION_SECONDS
        .with_label_values(&[method, endpoint, &status_label])
        .observe(duration.as_secs_f64());
}

/// Expose Prometheus metrics via `/metrics`.
pub async fn metrics_handler() -> impl Responder {
    let metric_families = prometheus::gather();
    let mut buffer = Vec::with_capacity(8192);
    let encoder = TextEncoder::new();

    if let Err(err) = encoder.encode(&metric_families, &mut buffer) {
        return HttpResponse::InternalServerError().body(format!("failed to encode metrics: {err}"));
    }

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}
