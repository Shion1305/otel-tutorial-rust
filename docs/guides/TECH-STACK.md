# Technology Stack Explained

## Overview: How OTEL + Loki + Grafana Works Together

This is a complete observability system. Here's how each component works and how they interact.

## The 3 Pillars of Observability

### 1. **Traces** (Distributed Tracing)
**Purpose**: Understand the complete journey of a request through your system

**In this project:**
- **Tool**: OpenTelemetry + Jaeger
- **What it tracks**: API requests, database queries, function calls
- **What you see**: Timeline of what happened and how long each step took
- **Use case**: "Why did request X take 5 seconds?"

**How it works:**
```
Request arrives
    ↓
[Span: http_request] ----[23 ms total]----
    ├─ [Span: auth_check] ------[2 ms]
    ├─ [Span: db_query] -------[15 ms]
    └─ [Span: serialize] ------[6 ms]
    ↓
Response sent
```

Each span records:
- Start/end time
- Operation name
- Attributes (user_id, endpoint, etc.)
- Events (structured log messages)
- Status (success/error)

**In Rust**: The `#[tracing::instrument]` macro creates spans:
```rust
#[tracing::instrument]
async fn get_user(id: String) -> Result<User> {
    info!("Fetching user");  // Creates an event in the span
    // ... your code ...
}
```

---

### 2. **Logs** (Structured Logging)
**Purpose**: Detailed record of what happened in the system

**In this project:**
- **Tool**: Tracing (Rust) → Promtail → Loki
- **What it captures**: Application events, errors, state changes
- **What you see**: Searchable, indexed logs across services
- **Use case**: "What error message did service X log at 3:45 PM?"

**How it works:**
```
Application
    ↓
info!("User created", user_id="123")  [structured]
    ↓
stdout output
    [JSON format]
    ↓
Promtail (collector)
    [reads logs from containers]
    ↓
Loki (indexer)
    [stores with labels: job, container, level]
    ↓
Grafana
    [query and visualize]
```

**Key difference from traditional logs:**
- Traditional: `"User 123 created at 2024-10-27 10:30:45"`
- Structured: `{timestamp: "...", level: "INFO", user_id: "123", action: "created"}`

Structured logs let you search by any field!

---

### 3. **Metrics** (Performance Data)
**Purpose**: Quantitative measurements of system health

**In this project:**
- **Tool**: Prometheus + Grafana
- **What it tracks**: Request count, error rate, response time, resource usage
- **What you see**: Graphs and trends over time
- **Use case**: "What's our p95 latency? How many requests failed today?"

**How it works:**
```
Application (exports metrics endpoint)
    ↓
Prometheus (scraper)
    [every 15 seconds: GET http://app:8080/metrics]
    ↓
Time-series database
    [stores: metric_name{labels} = value]
    ↓
Grafana
    [graphs, aggregations, alerts]
```

Example metric:
```
http_requests_total{endpoint="/api/users", method="POST", status="201"} = 42
request_duration_seconds{endpoint="/api/users", quantile="0.95"} = 0.234
```

---

## Component Details

### OpenTelemetry (OTEL)
**What it is**: A standard for instrumenting code to produce observability data

**Why use it:**
- Vendor-agnostic (can export to Jaeger, Zipkin, Prometheus, etc.)
- Community standard (multiple languages)
- Pre-built integrations with popular libraries

**In this project:**
- Automatically creates spans for instrumented functions
- Sends spans to Jaeger (UDP port 6831)
- Integrates with the `tracing` crate

```rust
// OTEL is configured once:
init_opentelemetry().await;

// Then every #[tracing::instrument] creates spans that go to Jaeger
#[tracing::instrument]
async fn my_function() { ... }
```

---

### Jaeger (Distributed Tracing Backend)
**What it is**: Storage and UI for viewing traces

**Port mapping:**
- `6831/UDP`: Jaeger agent (receives spans from apps)
- `14268/HTTP`: Jaeger collector (alternative)
- `16686/HTTP`: Web UI for viewing traces

**What you see in Jaeger:**
1. **Service list**: All services sending traces
2. **Operation list**: All functions being traced
3. **Trace timeline**: Visual timeline with spans
4. **Span details**: Attributes, logs, child spans

**Example Jaeger workflow:**
```
1. Request → Rust app → creates span "get_user"
2. Span sent to Jaeger (UDP, port 6831)
3. Jaeger stores it with trace_id, span_id, timestamps
4. You open Jaeger UI → see the span in timeline
```

---

### Loki (Log Aggregation)
**What it is**: Time-series database optimized for logs

**Why Loki instead of other log systems:**
- More resource-efficient than Elasticsearch/Splunk
- Works great with Grafana (both made by Grafana Labs)
- Uses labels instead of indexing every field
- Good for containerized environments

**Port mapping:**
- `3100`: HTTP API for pushing/querying logs

**How logs flow to Loki:**
```
1. Rust app → info!("message") → stdout (JSON)
2. Docker captures stdout
3. Promtail reads container logs
4. Promtail sends to Loki via HTTP (port 3100)
5. Loki indexes with labels
6. Grafana queries Loki
```

**Loki's Query Language (LogQL):**
```
{job="docker", container="otel-tutorial"} | json
{job="docker"} | level="ERROR"
rate({job="docker"}[5m])  # error rate
```

---

### Promtail (Log Collector)
**What it is**: Agent that collects logs and sends to Loki

**In this project:**
- Reads logs from Docker containers
- Parses and labels them
- Sends batches to Loki every few seconds

**Configuration:**
```yaml
clients:
  - url: http://loki:3100/loki/api/v1/push  # Where to send logs

scrape_configs:
  - job_name: docker
    docker_sd_configs:
      - host: unix:///var/run/docker.sock  # Read from Docker
```

**What it does:**
1. Discover containers via Docker API
2. Extract logs from container stdout/stderr
3. Add labels (container name, image, job)
4. Push to Loki periodically

---

### Prometheus (Metrics Storage)
**What it is**: Time-series database for metrics

**Port mapping:**
- `9090`: Prometheus UI and API

**How metrics are collected:**
```
1. Prometheus wakes up every 15 seconds (scrape_interval)
2. Makes HTTP GET request to each target: http://service:port/metrics
3. Parses response (Prometheus text format)
4. Stores in time-series database
5. Keeps ~15 days of data (retention)
```

**Prometheus query language (PromQL):**
```
up  # is each service up? (1 = yes, 0 = no)
rate(requests_total[5m])  # requests per second
histogram_quantile(0.95, duration_seconds)  # p95 latency
```

---

### Grafana (Visualization)
**What it is**: Dashboarding and visualization platform

**Port mapping:**
- `3000`: Web UI (default: admin/admin)

**Capabilities:**
- Query any datasource (Prometheus, Loki, Jaeger)
- Create custom dashboards
- Set up alerts
- Build multi-panel displays

**Grafana integrations in this project:**
```
Grafana Dashboard
├── Panel 1: Loki logs
│   └── Shows recent application logs
├── Panel 2: Prometheus metrics
│   └── Shows request rate graph
├── Panel 3: Jaeger trace explorer
│   └── Link to individual traces
└── Panel 4: Stats
    └── Error count, latency, etc.
```

---

## Data Flow Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                     Your Rust Application                         │
│  • Uses #[tracing::instrument] macros                             │
│  • Calls info!(), warn!(), error!() for logging                   │
│  • All happens in-process, very low overhead                      │
└────────────┬──────────────────────┬──────────────────────────────┘
             │                      │
    ┌────────▼────────┐    ┌────────▼────────┐
    │ stdout (JSON)   │    │ UDP Port 6831   │
    │ logs            │    │ (Jaeger agent)  │
    │                 │    │ spans           │
    └────────┬────────┘    └────────┬────────┘
             │                      │
    ┌────────▼────────┐    ┌────────▼────────┐
    │   Promtail      │    │     Jaeger      │
    │  (collector)    │    │  (aggregator)   │
    │  Port: docker   │    │  Port: 6831     │
    │  socket         │    │                 │
    └────────┬────────┘    └────────┬────────┘
             │                      │
    ┌────────▼────────┐    ┌────────▼────────┐
    │      Loki       │    │     Jaeger      │
    │  (log indexer)  │    │  (storage & UI) │
    │  Port: 3100     │    │  Port: 16686    │
    └────────┬────────┘    └────────┬────────┘
             │                      │
             └──────────┬───────────┘
                        │
                    ┌───▼──────┐
                    │ Grafana  │
                    │ Port:3000│
                    │          │
                    │Dashboard │
                    │with Loki,│
                    │Prometheus│
                    │& Jaeger  │
                    └──────────┘
```

---

## The OpenTelemetry Instrumentation Pattern

### 1. **Automatic Spans** (with macro)
```rust
#[tracing::instrument]
async fn process_request(id: String) {
    // Span created automatically
    // Function name = span name
    // Arguments recorded as fields
    // Return value and errors recorded

    info!("Processing");
    let result = do_something().await;
}
```

### 2. **Manual Spans** (for complex flows)
```rust
let span = tracing::info_span!("operation_name", user_id = %id);

let result = async {
    info!("Starting");
    // ... work ...
    info!("Done");
}.instrument(span).await;
```

### 3. **Log Events**
```rust
info!("message");              // INFO level
warn!("something odd");        // WARN level
error!("something failed");    // ERROR level
debug!("diagnostic info");     // DEBUG level
```

---

## Performance Characteristics

| Component | Memory | CPU | Disk | Notes |
|-----------|--------|-----|------|-------|
| App with OTEL | +5-10MB | <1% | Minimal | Very low overhead |
| Loki | 100-500MB | 5-15% | Depends on retention | Efficient compression |
| Prometheus | 100-1GB | 5-20% | Depends on retention | In-memory cache |
| Jaeger | 100-500MB | 5-10% | Depends on retention | Can use external storage |
| Grafana | 100-300MB | 3-8% | Minimal | Just visualization |

---

## Common Questions

**Q: Do these services talk to each other?**
A: Only through their APIs. Prometheus scrapes endpoints, Promtail pushes to Loki. The app is unaware of them.

**Q: Can I use this with microservices?**
A: Yes! Each service sends traces to Jaeger. Traces automatically link across services via trace_id.

**Q: What about security?**
A: In production, use authentication/encryption. This tutorial assumes a private network.

**Q: Can I replace components?**
A: Yes! OpenTelemetry is designed for this. Replace Jaeger with Zipkin, Loki with ELK, etc.

**Q: How much data volume can this handle?**
A: This setup handles ~1000 requests/second easily. Bigger volumes need distributed storage.

---

## Next Level Learning

1. **Read**: OpenTelemetry specification (opentelemetry.io)
2. **Explore**: Jaeger documentation (jaegertracing.io)
3. **Practice**: Add custom metrics to your application
4. **Deploy**: Move to production with persistent storage
5. **Monitor**: Set up alerting based on your metrics

---

You now understand how the entire observability stack works! 🚀
