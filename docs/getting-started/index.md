# Welcome to OpenTelemetry + Rust + K6

Welcome! You've received a complete, production-ready observability tutorial for Rust.

## What You Have

A comprehensive learning project that teaches you:

- **Distributed Tracing** (Jaeger) - See requests flowing through your system
- **Structured Logging** (Loki) - Search and aggregate logs effortlessly
- **Metrics Collection** (Prometheus) - Measure performance in real-time
- **Unified Dashboards** (Grafana) - See everything in one place

All in a real, running Rust application.

## Your 3-Step Quick Start (5 minutes)

### Step 1: Start the Infrastructure

```bash
cd /Users/shion/workspace/otel-tutorial-rust
docker-compose up -d
```

### Step 2: Start the Application

```bash
RUST_LOG=info ./target/release/otel-tutorial
```

### Step 3: View Your Data

Open these URLs in your browser:

| Dashboard | URL | Purpose |
|-----------|-----|---------|
| **Grafana** | http://localhost:3000 | Logs + Metrics |
| **Jaeger** | http://localhost:16686 | Distributed Traces |
| **Prometheus** | http://localhost:9090 | Raw Metrics |

**That's it!** You now have a complete observability system running.

## What To Read (In Order)

### 1. **Quick Start** (5 min) - GET IT RUNNING
Start here if you want to see things working fast.

### 2. **Tech Stack** (30 min) - UNDERSTAND HOW IT WORKS
Read this to understand the pieces:
- What is a trace, span, log, metric?
- How do they work together?
- Why this architecture?

### 3. **Onboarding** (15 min) - YOUR LEARNING PATH
Your structured learning journey with exercises.

### 4. **Architecture** (30 min) - COMPLETE REFERENCE
The full detailed documentation with advanced topics.

## The Code You'll Learn From

```
src/
├── main.rs              ← Entry point
├── observability.rs     ← How OTEL is set up
├── handlers.rs          ← How endpoints are instrumented
└── custom_middleware.rs ← Request tracking
```

## Test It Out

Make these requests and watch the data appear:

```bash
# Simple request
curl http://localhost:8080/api/health

# CPU-intensive operation (interesting trace!)
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 25}'

# Create a user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'
```

Then:
1. Open Grafana → Explore → Loki → Query: `{job="otel-tutorial"}`
2. Open Jaeger → Find the traces

## Common Questions

**Q: Do I need to read everything?**
A: No. Start with Quick Start, then read what interests you.

**Q: How long will this take to learn?**
A: Quick Start (5 min) + Tech Stack (30 min) = You understand it all in 35 minutes.

**Q: Can I use this with my own code?**
A: Yes! Copy `src/observability.rs` to your project and follow the pattern.

**Q: What if something doesn't work?**
A: Check the Quick Start troubleshooting section.

## My Recommended Path

### Day 1: Experience It (30 min)
1. Run the Quick Start
2. Make some requests
3. Explore Grafana, Jaeger, Prometheus
4. Read Tech Stack overview

### Day 2: Understand It (1 hour)
1. Read Tech Stack completely
2. Read Onboarding
3. Do the exercises
4. Review the source code

### Day 3: Apply It (ongoing)
1. Apply to your own Rust project
2. Customize dashboards in Grafana
3. Add your own instrumentation
4. Read Architecture advanced topics

## What You'll Know By The End

- ✅ How to instrument a Rust application with OTEL
- ✅ What distributed tracing is and how it works
- ✅ How log aggregation with Loki works
- ✅ How to query and visualize logs in Grafana
- ✅ How to see traces in Jaeger
- ✅ How to understand multi-span request flows
- ✅ How all the pieces work together
- ✅ How to apply this to your own projects

## Next Steps

1. **Right now**: Follow [Quick Start](quickstart.md)
2. **After that**: Read [Tech Stack](../guides/tech-stack.md)
3. **Then**: Explore the code in `src/`
4. **Finally**: Apply to your projects!

---

**Ready to get started?** → [Go to Quick Start](quickstart.md)
