# Quick Reference Guide

## URLs & Ports

| Service | URL | Port | Purpose |
|---------|-----|------|---------|
| Rust App | http://localhost:8080 | 8080 | API server |
| Grafana | http://localhost:3000 | 3000 | Dashboards & visualization |
| Jaeger | http://localhost:16686 | 16686 | Trace visualization |
| Prometheus | http://localhost:9090 | 9090 | Metrics query |
| Loki | http://localhost:3100 | 3100 | Log API |
| Jaeger Agent | 127.0.0.1:6831/UDP | 6831 | Span receiver |

## API Endpoints

```bash
# Health Check
curl http://localhost:8080/api/health

# List Users
curl http://localhost:8080/api/users

# Create User
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'

# Get User
curl http://localhost:8080/api/users/user-id-here

# Compute Fibonacci
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 20}'
```

## Docker Commands

```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# View running containers
docker-compose ps

# View logs of a service
docker-compose logs loki
docker-compose logs grafana
docker-compose logs jaeger

# Restart a service
docker-compose restart loki

# Remove volumes (deletes all data)
docker-compose down -v
```

## Grafana

| Item | Value |
|------|-------|
| URL | http://localhost:3000 |
| Default Username | admin |
| Default Password | admin |

### Exploring Logs (Loki)
1. Explore → Select "Loki"
2. Query: `{job="docker", container="otel-tutorial"}`
3. Press Shift+Enter

### Exploring Metrics (Prometheus)
1. Explore → Select "Prometheus"
2. Query: `up` (shows service status)
3. Press Shift+Enter

## Jaeger

| Item | Value |
|------|-------|
| URL | http://localhost:16686 |
| Service | otel-tutorial |

### View Traces
1. Select Service: "otel-tutorial"
2. Click "Find Traces"
3. Click any trace to see details

## Loki Queries (LogQL)

```logql
# All logs from your service
{job="docker", container="otel-tutorial"}

# Only INFO level logs
{job="docker", container="otel-tutorial"} | level="INFO"

# Only ERROR level logs
{job="docker", container="otel-tutorial"} | level="ERROR"

# Search for specific message
{job="docker", container="otel-tutorial"} |= "User created"

# Count logs per level
count by (level) ({job="docker", container="otel-tutorial"})

# Parse JSON and filter
{job="docker", container="otel-tutorial"} | json | user_id="123"
```

## Prometheus Queries (PromQL)

```promql
# Is service up?
up

# Request rate (requests per second)
rate(requests_total[5m])

# Error rate
rate(requests_total{status=~"5.."}[5m])

# Request duration (95th percentile)
histogram_quantile(0.95, request_duration_seconds)

# Jaeger service status
jaeger_collector_spans_received_total
```

## Rust Tracing Macros

```rust
// Create a span automatically
#[tracing::instrument]
async fn my_function(user_id: String) { ... }

// Skip a parameter from the span
#[tracing::instrument(skip(req))]
async fn my_function(req: Request) { ... }

// Log an event
info!("User created");
warn!("Something suspicious");
error!("Operation failed");
debug!("Diagnostic info");

// Log with fields
info!(user_id = %id, email = %email, "User registered");
```

## Environment Variables

```bash
# Rust log level
export RUST_LOG=info

# Jaeger configuration
export JAEGER_AGENT_HOST=localhost
export JAEGER_AGENT_PORT=6831
export JAEGER_SERVICE_NAME=otel-tutorial

# Start app
./target/release/otel-tutorial
```

## File Locations

| File | Purpose |
|------|---------|
| src/main.rs | Application entry point |
| src/observability.rs | OTEL & tracing setup |
| src/handlers.rs | API endpoints |
| src/custom_middleware.rs | Request tracking |
| config/loki-config.yml | Loki configuration |
| config/prometheus.yml | Prometheus configuration |
| docker-compose.yml | Container orchestration |
| Cargo.toml | Rust dependencies |

## Key Dependencies

```toml
# In Cargo.toml

# Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }

# OpenTelemetry
opentelemetry = "0.20"
opentelemetry-jaeger = "0.19"
tracing-opentelemetry = "0.21"

# Web
actix-web = "4.4"

# Utilities
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.6", features = ["v4"] }
```

## Common Tasks

### Build the Project
```bash
cargo build --release
```

### Run the Application
```bash
RUST_LOG=info ./target/release/otel-tutorial
```

### View Application Logs
```bash
# In Grafana
# Explore → Loki → {container="otel-tutorial"}
```

### View Request Traces
```bash
# In Jaeger
# http://localhost:16686 → Select otel-tutorial → Find Traces
```

### View Metrics
```bash
# In Prometheus
# http://localhost:9090 → Query: up
```

### Check Container Status
```bash
docker-compose ps
```

### View Container Logs
```bash
docker-compose logs -f loki
```

### Restart Everything
```bash
docker-compose down
docker-compose up -d
```

## Troubleshooting Commands

```bash
# Test Loki connectivity
curl http://localhost:3100/ready

# Test Jaeger connectivity
curl http://localhost:16686/api/services

# Test Prometheus connectivity
curl http://localhost:9090/-/healthy

# Check app is responding
curl http://localhost:8080/api/health

# View docker network
docker network inspect otel-tutorial-rust_observability

# Check if ports are in use
lsof -i :8080      # App
lsof -i :3000      # Grafana
lsof -i :16686     # Jaeger
lsof -i :3100      # Loki
lsof -i :9090      # Prometheus
```

## Span Attributes in Code

When creating a span, you can add attributes:

```rust
let span = tracing::info_span!(
    "operation_name",
    user_id = %user_id,          // %user_id formats with Display
    email = %email,
    count = items.len(),
    active = true,
    duration_ms = tracing::field::Empty,  // Set later with record()
);

// Later, fill in the empty field
span.record("duration_ms", elapsed);
```

## Log Levels Explained

| Level | When to Use | Example |
|-------|------------|---------|
| DEBUG | Detailed diagnostic info | "User 123 loaded from DB" |
| INFO | Important events | "User registered successfully" |
| WARN | Unusual but handled | "Invalid email format, rejected" |
| ERROR | Errors | "Database connection failed" |
| TRACE | Very detailed | Rarely used |

## Common Errors & Solutions

| Error | Solution |
|-------|----------|
| "Connection refused" on curl | Ensure app is running on port 8080 |
| No logs in Grafana | Wait 15s for Promtail, refresh page |
| No traces in Jaeger | Check JAEGER_AGENT_HOST=localhost |
| Grafana datasources failing | Reload Grafana: `docker-compose restart grafana` |
| Container won't start | Check: `docker-compose logs service-name` |

## Performance Tips

- Use `skip()` in macros for large objects
- Set `RUST_LOG=warn` in production (more data = slower)
- Use sampling for high-volume services
- Archive old data from Loki & Prometheus

## Security Notes

- Default Grafana password is `admin/admin` - change in production
- Loki has no authentication - use firewall rules
- Promtail reads Docker socket - runs as unprivileged user
- No TLS by default - add reverse proxy for production

## File Sizes

- Rust binary: ~30MB (release)
- Docker images: ~500MB total
- Data per 1000 requests: ~1-2MB

---

**For more details, see the full documentation:**
- [Quick Start](../getting-started/QUICKSTART.md) - Get started fast
- [Tech Stack](../guides/TECH-STACK.md) - Understand the architecture
- [Architecture Guide](../guides/ARCHITECTURE.md) - Complete reference

