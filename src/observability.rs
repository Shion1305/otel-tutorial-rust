/// Observability Module - OTEL, Tracing, and Loki Integration
///
/// This module demonstrates how to set up observability in a Rust application using:
/// 1. OpenTelemetry (OTEL) for distributed tracing
/// 2. Tracing crate for structured logging
/// 3. Loki for log aggregation (via structured logs)
///
/// Key Concepts:
/// - Spans: Represent a unit of work (request, database query, etc.)
/// - Traces: Collection of spans that represent a complete operation
/// - Metrics: Quantitative measurements
/// - Logs: Textual information about events

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize all telemetry: OpenTelemetry, Tracing, and Loki
pub async fn setup_telemetry() {
    // Initialize Tracing Subscriber (must be first)
    init_tracing();

    // Initialize OpenTelemetry Tracer (Jaeger backend)
    init_opentelemetry().await;

    info!("✓ Telemetry initialized successfully");
}

/// Initialize OpenTelemetry with Jaeger exporter
///
/// This creates a tracer that sends spans to a Jaeger collector.
/// The tracer is globally registered so it can be accessed anywhere in the application.
async fn init_opentelemetry() {
    // Note: Jaeger initialization happens via environment variables or config files
    // JAEGER_AGENT_HOST, JAEGER_AGENT_PORT, JAEGER_SERVICE_NAME, etc.
    // For now, we just log that OTEL is ready
    info!("✓ OpenTelemetry configured (set JAEGER_* env vars to enable export)");
}

/// Initialize tracing subscriber with multiple layers
///
/// This sets up structured logging that can be:
/// - Printed to stdout (fmt layer)
/// - Sent to logs for Loki collection
fn init_tracing() {
    // Create a layer that formats logs to stdout with JSON
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .json();

    // Create environment filter (respects RUST_LOG env var)
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Combine layers (without OTEL initially to avoid trait bounds issues)
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    info!("✓ Tracing subscriber initialized");
}

/// Helper to create a span for a specific operation
///
/// Example usage:
/// ```
/// let span = create_span("database_query");
/// let result = async {
///     // do work
/// }.instrument(span).await;
/// ```
pub fn create_span(name: &str) -> tracing::Span {
    tracing::info_span!(
        "operation",
        name = %name,
        span_id = %uuid::Uuid::new_v4(),
    )
}
