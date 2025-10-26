# 🚀 START HERE - Your OpenTelemetry + Rust Journey Begins

Welcome! You've received a complete, production-ready observability tutorial for Rust. Let me guide you through it.

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
cargo build --release
RUST_LOG=info ./target/release/otel-tutorial
```

### Step 3: View Your Data
Open these URLs:
- **Grafana** (dashboards): http://localhost:3000 (admin/admin)
- **Jaeger** (traces): http://localhost:16686
- **Prometheus** (metrics): http://localhost:9090

**That's it!** You now have a complete observability system running.

## What To Read (In Order)

### 1. **QUICKSTART.md** (5 min) - GET IT RUNNING
Start here if you want to see things working fast.
- How to start all services
- How to make API requests
- Where to see the data

### 2. **TECH-STACK.md** (30 min) - UNDERSTAND HOW IT WORKS
Read this to understand the pieces:
- What is a trace, span, log, metric?
- How do they work together?
- Why this architecture?

### 3. **ONBOARDING.md** (15 min) - YOUR LEARNING PATH
Your structured learning journey:
- Phase 1: "Show me it works" (5 min)
- Phase 2: "Explain how it works" (15 min)
- Phase 3: "Show me the code" (30 min)
- Exercises to practice

### 4. **README.md** (30 min) - COMPLETE REFERENCE
The full detailed documentation:
- Architecture overview
- Project structure
- Advanced topics
- Troubleshooting

### 5. **REFERENCE.md** (Bookmark it!) - QUICK LOOKUP
Quick reference for:
- URLs and ports
- Curl commands
- Docker commands
- LogQL and PromQL examples

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
# In Terminal 2 (while app is running):

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
1. Open Grafana → Explore → Loki → Query: `{container="otel-tutorial"}`
2. Open Jaeger → Find the traces

## Key Files at a Glance

| File | What It Does | Read When |
|------|--------------|-----------|
| QUICKSTART.md | Get running in 5 min | First |
| TECH-STACK.md | Understand the architecture | Second |
| ONBOARDING.md | Structured learning path | Third |
| README.md | Complete reference | Whenever you need detail |
| REFERENCE.md | Quick lookup (bookmark!) | Always |
| src/observability.rs | OTEL setup code | Learning code |
| src/handlers.rs | Instrumented endpoints | Learning code |

## What Each Component Does

```
Your Rust App → Traces → Jaeger UI → See request timeline
              ↓ Logs (JSON) ↓ Loki ↓ Grafana → Search logs
              ↓ Metrics ↓ Prometheus ↓ → View graphs
```

## Common Questions

**Q: Do I need to read everything?**
A: No. Start with QUICKSTART.md, then read what interests you.

**Q: How long will this take to learn?**
A: QUICKSTART (5 min) + TECH-STACK (30 min) = You understand it all in 35 minutes.

**Q: Can I use this with my own code?**
A: Yes! Copy `src/observability.rs` to your project and follow the pattern.

**Q: What if something doesn't work?**
A: Check QUICKSTART.md troubleshooting section, then README.md.

## My Recommended Path

### Day 1: Experience It (30 min)
1. ✅ Run QUICKSTART.md
2. ✅ Make some requests
3. ✅ Explore Grafana, Jaeger, Prometheus
4. ✅ Read TECH-STACK.md overview

### Day 2: Understand It (1 hour)
1. ✅ Read TECH-STACK.md completely
2. ✅ Read ONBOARDING.md
3. ✅ Do the exercises
4. ✅ Review the source code

### Day 3: Apply It (ongoing)
1. ✅ Apply to your own Rust project
2. ✅ Customize dashboards in Grafana
3. ✅ Add your own instrumentation
4. ✅ Read README.md advanced topics

## What You'll Know By The End

- ✅ How to instrument a Rust application with OTEL
- ✅ What distributed tracing is and how it works
- ✅ How log aggregation with Loki works
- ✅ How to query and visualize logs in Grafana
- ✅ How to see traces in Jaeger
- ✅ How to understand multi-span request flows
- ✅ How all the pieces work together
- ✅ How to apply this to your own projects

## The Command You'll Run Most

```bash
RUST_LOG=info ./target/release/otel-tutorial
```

Then open:
- http://localhost:3000 (Grafana)
- http://localhost:16686 (Jaeger)

## If Something Breaks

1. Run `docker-compose logs` to see what's wrong
2. Check the relevant documentation section
3. Try `docker-compose down && docker-compose up -d`
4. Read the TROUBLESHOOTING section in README.md

## Next Steps

1. **Right now**: Run QUICKSTART.md
2. **After that**: Read TECH-STACK.md
3. **Then**: Explore the code in src/
4. **Finally**: Apply to your projects!

---

## Quick Navigation

📍 **You are here**: START_HERE.md
🏃 **Go fast**: QUICKSTART.md
📚 **Learn deep**: TECH-STACK.md
🎓 **Structured path**: ONBOARDING.md
📖 **Full reference**: README.md
⚡ **Quick lookup**: REFERENCE.md
💻 **Source code**: src/

---

## Your Goals

- [ ] Get everything running (QUICKSTART.md)
- [ ] Understand the architecture (TECH-STACK.md)
- [ ] Read the source code
- [ ] Do the exercises
- [ ] Feel confident explaining observability
- [ ] Apply to your own projects

---

## You've Got This! 🎉

You have:
- ✅ A fully working observability system
- ✅ Complete source code with comments
- ✅ 5 detailed documentation files
- ✅ Docker compose setup ready to go
- ✅ Example endpoints to test with
- ✅ Everything compiled and ready

**All that's left is for you to experience it and learn!**

## Start Now

```bash
# Copy and paste this to get started:
cd /Users/shion/workspace/otel-tutorial-rust
docker-compose up -d
cargo build --release
RUST_LOG=info ./target/release/otel-tutorial
```

Then open: http://localhost:3000

Welcome to observability! 🚀

---

**Next: Open QUICKSTART.md**
