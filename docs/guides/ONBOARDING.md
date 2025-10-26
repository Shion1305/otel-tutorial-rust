# Your OTEL + Grafana + Loki Learning Journey

Welcome! This guide walks you through your observability learning path step-by-step.

## Your Learning Map

```
Start Here
   ↓
1. [Quick Start](../getting-started/QUICKSTART.md) ← Run everything in 5 minutes
   ↓
2. [This File] ← Understand the pieces
   ↓
3. [Tech Stack](../guides/TECH-STACK.md) ← Deep dive into how it works
   ↓
4. [Architecture](../guides/ARCHITECTURE.md) ← Advanced usage and troubleshooting
   ↓
5. [Source Code] ← Read and modify the implementation
   ↓
Apply & Extend ← Build your own observability
```

## What You're Learning Today

By the end of this tutorial, you'll understand:

✅ **Tracing**: How to track a request through your system (Jaeger)
✅ **Logging**: How to collect structured logs from services (Loki)
✅ **Metrics**: How to measure system performance (Prometheus)
✅ **Visualization**: How to see it all in one place (Grafana)
✅ **OTEL**: Industry standard for instrumentation

## The Problem We're Solving

Imagine you're running an API service and someone reports "the API is slow."

**Without observability:**
- Where did the slowness happen? (app, database, network?)
- Which requests were affected?
- What was the system doing at that time?
- How long did each operation take?
→ Pure guesswork

**With observability (what you'll build):**
- See complete trace of each request with exact timings
- Read structured logs showing exactly what happened
- View metrics graph showing when slowness started
- Correlate traces, logs, and metrics to find the issue in minutes

→ Data-driven debugging

## The Architecture You're Building

```
Your Rust API
├─ Traces (detailed request flow)
│  └→ Jaeger (traces.otel.io-like interface)
│
├─ Logs (structured events)
│  └→ Loki (log search and storage)
│
└─ Metrics (performance data)
   └→ Prometheus (time-series metrics)

All visible in → Grafana (unified dashboard)
```

## Getting Started: The Three Phases

### Phase 1: "Show Me It Works" (5 min) 📦

**File**: [Quick Start](../getting-started/QUICKSTART.md)

What you'll do:
1. Start Docker containers
2. Run the Rust app
3. Make some API requests
4. View data in Grafana and Jaeger

**Learning outcome**: Understand what observability data looks like

### Phase 2: "Explain How It Works" (15 min) 🔍

**File**: [Tech Stack](../guides/TECH-STACK.md)

What you'll learn:
1. What does each component do?
2. How do they talk to each other?
3. Why this architecture?
4. What is a trace, span, log, metric?

**Key insight**: These aren't magic - they're just organized data collection

### Phase 3: "Show Me the Code" (30 min) 💻

**Files**: `src/*.rs`

What you'll read:
1. `src/observability.rs` - How OTEL is configured
2. `src/handlers.rs` - How to instrument endpoints
3. `src/custom_middleware.rs` - Request ID tracking
4. `src/main.rs` - Putting it all together

**Key insight**: Only ~200 lines of instrumentation code needed!

### Phase 4: "Deep Dive" (optional) 🚀

**File**: [Architecture](../guides/ARCHITECTURE.md)

Advanced topics:
- Custom spans for complex flows
- Multi-service tracing
- Connecting logs to traces
- Production-ready setups

---

## Your First Hands-On Exercise

### Exercise 1: View Your Logs (10 min)

```bash
# 1. Start everything
docker-compose up -d
./target/release/otel-tutorial  # or cargo run

# 2. Make some requests in another terminal
curl http://localhost:8080/api/users

# 3. Open Grafana
# http://localhost:3000
# Login: admin / admin

# 4. Explore → Loki
# Query: {container="otel-tutorial"}
# Press Shift+Enter to run
```

**What you should see:**
JSON logs with fields like:
- `timestamp`: When it happened
- `level`: INFO, WARN, ERROR
- `message`: What happened
- `target`: Which module

**Try this:**
- Modify the query: `{container="otel-tutorial"} | level="INFO"`
- See only INFO-level logs

---

### Exercise 2: View Your Traces (10 min)

```bash
# Make a request that takes time
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 25}'

# Open Jaeger
# http://localhost:16686

# Select otel-tutorial service
# Click "Find Traces"
```

**What you should see:**
A timeline showing:
- Span name (operation_name)
- Duration (23.45 ms)
- Child spans inside
- Each color is a different span

**Try this:**
- Click different spans to see their attributes
- Click "Logs" tab to see logged events
- Look for "duration_ms" attribute

---

### Exercise 3: Add Your Own Instrumentation (15 min)

Edit `src/handlers.rs` and modify the health_check function:

```rust
#[tracing::instrument]  // Add this line
pub async fn health_check() -> ActixResult<HttpResponse> {
    info!("Health check called with custom field");  // Add this

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    })))
}
```

Then:
```bash
cargo build --release
./target/release/otel-tutorial

# In another terminal
curl http://localhost:8080/api/health

# Check Jaeger - you should see the span now!
```

---

## Key Concepts to Know

### Span
- **What**: One operation (function call, HTTP request, DB query)
- **Contains**: Start time, duration, status, attributes
- **In code**: `#[tracing::instrument]` creates one
- **In UI**: One colored box in the trace timeline

### Trace
- **What**: Complete journey of one request
- **Contains**: Multiple spans linked together
- **In code**: Automatically created, linked by trace_id
- **In UI**: Full timeline with multiple colored boxes

### Log
- **What**: Textual record of an event
- **Contains**: Timestamp, level, message, fields
- **In code**: `info!()`, `warn!()`, `error!()`
- **In UI**: Searchable table in Grafana/Loki

### Metric
- **What**: Quantitative measurement over time
- **Contains**: Name, labels, numeric value
- **In code**: Usually external library (we skipped for now)
- **In UI**: Graphs in Prometheus/Grafana

---

## The Learning Resources You Have

```
📁 /Users/shion/workspace/otel-tutorial-rust/
│
├── 📚 docs/
│   ├── 🚀 ../getting-started/QUICKSTART.md     [Start here! 5 min]
│   ├── 📚 ../guides/TECH-STACK.md              [How it all works]
│   ├── 📖 ../guides/ARCHITECTURE.md            [Complete reference]
│   └── 👋 ../guides/ONBOARDING.md              [You are here!]
│
├── 🔧 src/
│   ├── main.rs                [App entry point]
│   ├── observability.rs       [OTEL setup]
│   ├── handlers.rs            [API with tracing]
│   └── custom_middleware.rs   [Request tracking]
│
├── ⚙️  config/                 [Service configs]
│   ├── loki-config.yml
│   ├── prometheus.yml
│   └── grafana/
│
├── 🐳 docker-compose.yml      [Full stack]
└── 📦 Cargo.toml              [Dependencies]
```

---

## Recommended Reading Order

### First Time (30 min)
1. This file (Onboarding) → you are here
2. [Quick Start](../getting-started/QUICKSTART.md) → get it running
3. Do Exercise 1 & 2 above
4. [Tech Stack](../guides/TECH-STACK.md) (just overview sections)

### Getting Deeper (1-2 hours)
1. [Tech Stack](../guides/TECH-STACK.md) (complete read)
2. Source code walkthrough (start with src/observability.rs)
3. Do Exercise 3
4. [Architecture](../guides/ARCHITECTURE.md) sections on "Understanding the Code"

### Production Ready (ongoing)
1. [Architecture](../guides/ARCHITECTURE.md) "Advanced Topics"
2. Official docs: opentelemetry.io
3. Build your own service with this setup
4. Add metrics collection
5. Set up alerting in Grafana

---

## Common Questions While Learning

**Q: Do I need to understand all the YAML configs?**
A: Not initially. Focus on understanding the code first. Configs are mostly preconfigured.

**Q: Why so many tools (Loki, Prometheus, Jaeger, Grafana)?**
A: They each do one thing well. Together they give you complete visibility.

**Q: What if I want to use different tools?**
A: OpenTelemetry is vendor-agnostic. Replace any component.

**Q: How do I add metrics?**
A: See [Architecture](../guides/ARCHITECTURE.md) section "Advanced Topics → Adding Metrics"

**Q: How do multiple services trace together?**
A: Trace IDs are passed between services (see trace context spec)

---

## Milestones

Check these off as you progress:

- [ ] Completed Quick Start
- [ ] Saw logs in Grafana
- [ ] Saw traces in Jaeger
- [ ] Understood the architecture
- [ ] Read Tech Stack
- [ ] Modified and recompiled the code
- [ ] Read and understood src/observability.rs
- [ ] Read and understood src/handlers.rs
- [ ] Completed Architecture Guide
- [ ] Created a custom dashboard in Grafana

---

## When You Get Stuck

1. **Check [Quick Start](../getting-started/QUICKSTART.md) troubleshooting section**
2. **Check docker-compose is running**: `docker-compose ps`
3. **Check app is running**: Look for "Server running on" message
4. **Check logs**: `docker-compose logs loki` or similar
5. **Read [Troubleshooting](../reference/troubleshooting.md) section**

---

## What's Next After This Tutorial?

### Build Something Real
Take this setup and add it to your own Rust application:
```bash
# Copy the observability module
cp src/observability.rs your-project/src/

# Copy Cargo.toml dependencies
# Update your own code with #[tracing::instrument]
```

### Add More Observability
- Metrics: Install `prometheus` crate
- Custom dashboards: Build in Grafana
- Alerting: Configure alert rules
- Multiple services: Link traces across services

### Go Deeper
- Read OpenTelemetry specs (opentelemetry.io)
- Learn PromQL (Prometheus query language)
- Learn LogQL (Loki query language)
- Explore Grafana advanced features

---

## You've Got This! 💪

The reason observability seems complex is because you're learning multiple tools at once. But each tool is actually simple:

- **Jaeger**: "Store and view traces"
- **Loki**: "Store and search logs"
- **Prometheus**: "Collect and store metrics"
- **Grafana**: "Visualize all the data"
- **OTEL**: "Standard way to instrument code"

Master them one at a time, and you'll be expert in observability in no time!

---

## Your Onboarding Checklist

- [ ] Understand the problem (why observability matters)
- [ ] Run [Quick Start](../getting-started/QUICKSTART.md)
- [ ] See data in all three systems (logs, traces, metrics)
- [ ] Understand the architecture ([Tech Stack](../guides/TECH-STACK.md))
- [ ] Read the source code
- [ ] Modify something and see it reflected
- [ ] Feel confident explaining observability to others

When all are checked, you're ready to apply this to real projects!

Start with [Quick Start](../getting-started/QUICKSTART.md) now! 🚀
