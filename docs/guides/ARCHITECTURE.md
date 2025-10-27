# OpenTelemetry with Rust: Complete Tutorial

A comprehensive tutorial demonstrating how to implement observability in a Rust application using OpenTelemetry (OTEL), distributed tracing with Jaeger, log aggregation with Loki, and visualization with Grafana.

## Architecture Overview

This tutorial implements a complete observability stack:

```
┌─────────────────────────┐
│  Rust Application       │
│  (with OTEL Tracing)    │
└────────┬────────────────┘
         │
         ├──→ Jaeger (Port 6831)    ← Distributed Traces
         │
         ├──→ Stdout Logs           ← Structured JSON Logs
         │    (JSON Format)
         │
         └──→ Promtail              ← Log Collection
              └──→ Loki (Port 3100)
                  └──→ Grafana (Port 3000)
                      ├── Loki (Logs)
                      ├── Prometheus (Metrics)
                      └── Jaeger (Traces)
```

## Key Concepts

### 1. **Spans** (Distributed Tracing)
- **What**: Represent a unit of work (request, database query, function call, etc.)
- **Where**: Jaeger backend receives and stores them
- **Example**: `#[tracing::instrument]` macro creates a span automatically

```rust
#[tracing::instrument]
async fn get_user(id: String) -> Result<User> {
    info!(user_id = %id, "Fetching user");
    // Each async call gets its own span
}
```

### 2. **Traces** (Distributed Tracing)
- **What**: A complete request's journey through the system
- **Composed of**: Multiple related spans
- **Benefit**: See exact timeline of what happened across services
- **View**: Jaeger UI at http://localhost:16686

### 3. **Logs** (Structured Logging)
- **What**: Textual records of events
- **Format**: JSON with structured fields
- **Storage**: Loki aggregates and indexes them
- **Query**: Use LogQL in Grafana

### 4. **Metrics** (Performance Data)
- **What**: Quantitative measurements (latency, throughput, errors)
- **Collection**: Prometheus scrapes targets
- **Visualization**: Grafana graphs and alerts

## Project Structure

```
otel-tutorial-rust/
├── src/
│   ├── main.rs                 # Entry point
│   ├── observability.rs        # OTEL/Tracing setup
│   ├── handlers.rs             # API endpoints with instrumentation
│   └── custom_middleware.rs    # Request ID middleware
├── config/
│   ├── loki-config.yml         # Loki configuration
│   ├── promtail-config.yml     # Log collector config
│   ├── prometheus.yml          # Metrics scraper config
│   └── grafana/
│       ├── datasources/        # Grafana data source configs
│       └── dashboards/         # Dashboard definitions
├── docker-compose.yml          # Full observability stack
└── Cargo.toml                  # Rust dependencies
```

## Quick Start

### 1. Start the Observability Stack

```bash
docker-compose up -d
```

This starts:
- **Loki** (Port 3100) - Log aggregation
- **Promtail** (no exposed port) - Log collector
- **Jaeger** (Port 6831 UDP, 16686 UI) - Distributed tracing
- **Prometheus** (Port 9090) - Metrics
- **Grafana** (Port 3000) - Visualization

### 2. Build and Run the Application

```bash
# Build the application
cargo build --release

# Run with default log level
./target/release/otel-tutorial

# Or with debug logging
RUST_LOG=debug ./target/release/otel-tutorial
```

The application starts on `http://127.0.0.1:8080`

### 3. Make Requests to Generate Observability Data

```bash
# Health check
curl http://localhost:8080/api/health

# List users
curl http://localhost:8080/api/users

# Create user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'

# Get user by ID
curl http://localhost:8080/api/users/{user_id}

# Compute fibonacci (CPU-intensive operation)
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 20}'
```

## Viewing Your Data

### Logs (Loki + Grafana)
1. Open Grafana: http://localhost:3000 (admin/admin)
2. Go to **Explore** → Select **Loki**
3. Run a query: `{job="rust-app"} | json`
4. See all logs from your application

### Traces (Jaeger)
1. Open Jaeger: http://localhost:16686
2. Select **otel-tutorial** from the service dropdown
3. Click **Find Traces**
4. Each request is a complete trace with all its spans

### Metrics (Prometheus + Grafana)
1. Open Grafana: http://localhost:3000
2. Go to **Explore** → Select **Prometheus**
3. Query examples:
   - `sum(rate(http_requests_total[1m]))` - Request rate
   - `histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))` - P95 latency

## Understanding the Code

### Setting Up Tracing (`src/observability.rs`)

```rust
pub async fn setup_telemetry() {
    // Initialize structured logging with JSON output
    init_tracing();

    // Initialize OTEL with Jaeger backend
    init_opentelemetry().await;
}
```

**What happens:**
1. `tracing-subscriber` creates a registry with layers
2. JSON formatter outputs structured logs to stdout
3. OTEL SDK connects to Jaeger (via env vars)
4. All logs and traces can now be collected

### Instrumenting Endpoints (`src/handlers.rs`)

```rust
#[tracing::instrument(skip(req))]
pub async fn create_user(
    req: web::Json<CreateUserRequest>
) -> ActixResult<HttpResponse> {
    info!("Creating new user: {}", req.name);
    // ... implementation ...
    info!(user_id = %user.id, "User created successfully");
    Ok(HttpResponse::Created().json(user))
}
```

**What happens:**
1. `#[tracing::instrument]` macro automatically creates a span
2. Function name becomes the span name
3. `info!()` logs create events within that span
4. Return value and errors are recorded
5. Span duration is automatically measured

### Request ID Middleware (`src/custom_middleware.rs`)

```rust
pub struct RequestIdMiddleware;

// For each request:
let request_id = Uuid::new_v4().to_string();
let span = tracing::info_span!(
    "http_request",
    request_id = %request_id,
    method = %method,
    path = %path,
);
```

**What happens:**
1. Unique ID generated for each request
2. Span created with request metadata
3. All logs within that request are linked
4. Easy to trace a request across multiple services

## Advanced Topics

### 1. Connecting Traces to Logs

In Grafana, configure the Jaeger datasource to link traces to logs:
- Go to **Data Sources** → **Jaeger**
- Enable "Traces to Logs"
- Map `trace_id` field to Loki

Then in Jaeger trace view, click "Logs" to see associated logs.

### 2. Custom Spans

Create spans for specific operations:

```rust
async fn process_payment(user_id: String) {
    let span = tracing::info_span!("payment_processing", user_id = %user_id);

    async {
        info!("Starting payment");
        // ... payment logic ...
        info!("Payment completed");
    }
    .instrument(span)
    .await
}
```

### 3. Adding Metrics

To add metrics (requests/errors), install `prometheus` crate:

```toml
[dependencies]
prometheus = "0.13"
```

Then track metrics alongside spans.

### 4. Multi-Service Tracing

In distributed systems, pass the trace context:

```rust
// In Service A: Create request header
let trace_header = span.context().to_w3c_trace_context();
let request = client
    .get(url)
    .header("traceparent", trace_header)
    .send()
    .await;

// In Service B: Extract trace from header
let span = tracing::info_span!(
    "request",
    trace_id = %trace_header.trace_id,
);
```

## Environment Variables

```bash
# Logging level
RUST_LOG=info  # or debug, warn, error

# Jaeger configuration
JAEGER_AGENT_HOST=localhost
JAEGER_AGENT_PORT=6831
JAEGER_SERVICE_NAME=otel-tutorial
JAEGER_SAMPLER_TYPE=const
JAEGER_SAMPLER_PARAM=1

# Other options
JAEGER_LOG_TRACES=true
JAEGER_TAGS=env=dev,service=api
```

## Troubleshooting

### Logs not appearing in Loki
1. Check if Promtail is running: `docker logs promtail`
2. Verify Loki is receiving logs: `curl http://localhost:3100/api/prom/label/job/values`
3. Check container name matches the scrape config

### Traces not appearing in Jaeger
1. Verify Jaeger is running: `curl http://localhost:16686/api/services`
2. Check JAEGER_AGENT_HOST and JAEGER_AGENT_PORT are correct
3. Ensure Rust app can reach UDP port 6831 to Jaeger

### Grafana datasources not connecting
1. Reload Grafana: `docker restart grafana`
2. Check network: `docker network inspect observability`
3. Verify datasources.yml is being mounted correctly

## Learning Resources

- **OpenTelemetry**: https://opentelemetry.io/docs/
- **Jaeger Tracing**: https://www.jaegertracing.io/docs/
- **Loki**: https://grafana.com/docs/loki/
- **Tracing Crate**: https://docs.rs/tracing/latest/tracing/
- **Grafana**: https://grafana.com/docs/grafana/latest/

## Next Steps

1. **Add Metrics**: Instrument performance-critical paths with metrics
2. **Multiple Services**: Set up another service and trace across them
3. **Alerting**: Configure Grafana alerts based on log patterns
4. **Custom Dashboards**: Build service-specific dashboards
5. **Sampling**: Implement sampling for high-throughput services

## Summary

This tutorial showed you:
- ✅ How to add OpenTelemetry to a Rust application
- ✅ How to create and use spans for tracing
- ✅ How Loki collects and stores logs
- ✅ How Grafana visualizes all observability data
- ✅ How to correlate traces, logs, and metrics

You now have a production-ready observability foundation!
