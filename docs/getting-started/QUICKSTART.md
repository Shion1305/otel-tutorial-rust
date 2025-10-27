# Quick Start Guide - 5 Minutes to Observability

## Prerequisites
- Docker and Docker Compose installed
- Rust 1.70+ installed
- curl or Postman for testing

## Step 1: Start the Observability Stack (1 min)

```bash
cd /Users/shion/workspace/otel-tutorial-rust

# Start all services: Loki, Promtail, Jaeger, Prometheus, Grafana
docker-compose up -d

# Verify all containers are running
docker-compose ps
```

**Expected output:** 5 containers running

## Step 2: Build and Run the Application (2 min)

```bash
# Build the application
cargo build --release

# Run the application
RUST_LOG=info ./target/release/otel-tutorial
```

**Expected output:**
```
{"timestamp":"...","level":"INFO","message":"Starting OpenTelemetry Tutorial Application",...}
{"timestamp":"...","level":"INFO","message":"Server running on http://127.0.0.1:8080",...}
```

## Step 3: Generate Some Observability Data (1 min)

In another terminal, run these commands:

```bash
# Health check
curl http://localhost:8080/api/health

# Create a user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "email": "john@example.com"}'

# List users
curl http://localhost:8080/api/users

# Do something that takes time (triggers tracing)
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 30}'
```

## Step 4: View Your Data

### 📊 Logs (Loki in Grafana)
1. Go to http://localhost:3000 (login: admin/admin)
2. Click **Explore**
3. Select **Loki** from datasource dropdown
4. Enter query: `{job="rust-app"} | json`
5. Click **Run query**

You'll see all your application logs in JSON format!

### 🔍 Traces (Jaeger)
1. Go to http://localhost:16686
2. Select **otel-tutorial** from "Service" dropdown
3. Click **Find Traces**
4. Click any trace to see the request journey with timings

You'll see spans for each operation with exact durations!

### 📈 Metrics (Prometheus in Grafana)
1. Go to http://localhost:3000
2. Click **Explore**
3. Select **Prometheus**
4. Try a simple query: `sum(rate(http_requests_total[1m]))` (requests/sec)
5. Want raw metrics? Visit `http://localhost:8080/metrics`

> **Note:** On Linux you may need to replace `host.docker.internal` with the host IP in `config/prometheus.yml` so Prometheus can scrape the application.

## What You're Seeing

### In Logs (JSON format):
```json
{
  "timestamp": "2024-10-27T...",
  "level": "INFO",
  "message": "Creating new user: John",
  "target": "otel_tutorial::handlers",
  "module_path": "otel_tutorial::handlers",
  "file": "src/handlers.rs",
  "line": 84
}
```

**Key fields:**
- `timestamp`: When the event happened
- `level`: Severity (INFO, WARN, ERROR, DEBUG)
- `message`: What happened
- Other fields: Context about the operation

### In Traces (Jaeger):
Each request shows:
- **Operation name**: What function was called
- **Duration**: How long it took (microseconds)
- **Tags**: Key metadata (user_id, status code, etc.)
- **Logs**: Events that happened during the span
- **Children**: Sub-operations within this operation

### Example Trace Structure:
```
GET /api/users                              [23.5 ms total]
├── http_request (middleware)               [23.2 ms]
│   └── list_users (handler)               [22.8 ms]
│       ├── database_query                 [15.2 ms]
│       └── serialization                  [7.6 ms]
└── logging                                 [0.3 ms]
```

## Key Concepts Explained

### Span = Unit of Work
A span represents one operation (API call, database query, function call). It records:
- **Start time** and **duration**
- **Status** (success/failure)
- **Tags** (metadata like user_id)
- **Events** (log messages)

### Trace = Request Journey
A trace is the complete path a request takes. It contains multiple spans showing:
- Which services it called
- How long each step took
- Where errors occurred

### Logs = Event Records
Structured logs in JSON format that can be:
- Searched by any field
- Aggregated across services
- Correlated with traces

## Stopping Everything

```bash
# Stop the application
# Press Ctrl+C in the terminal running the app

# Stop the Docker containers
docker-compose down

# Remove volumes (optional - deletes all data)
docker-compose down -v
```

## Common Issues

**Q: No logs appearing in Grafana?**
A: Give Promtail 10-15 seconds to collect logs. Refresh the Grafana page.

**Q: Can't see traces in Jaeger?**
A: Make sure the app is still running and JAEGER_AGENT_HOST is set to localhost.

**Q: "Connection refused" when curling the app?**
A: Ensure you ran the app and it's on port 8080. Check with `lsof -i :8080`.

## Next: Explore More

- Check `README.md` for detailed documentation
- Modify `src/handlers.rs` to add custom instrumentation
- Create a dashboard in Grafana using the Loki data source
- Try connecting Jaeger traces to Loki logs

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | Application entry point |
| `src/observability.rs` | Tracing/OTEL setup |
| `src/handlers.rs` | API endpoints with `#[tracing::instrument]` |
| `src/custom_middleware.rs` | Request ID tracking |
| `docker-compose.yml` | Full observability stack |
| `config/loki-config.yml` | Loki settings |
| `config/prometheus.yml` | Metrics scraping rules |

---

You now have a complete, working observability system! 🎉
