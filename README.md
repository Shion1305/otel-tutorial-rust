# OpenTelemetry + Rust + K6 Load Testing Tutorial

A comprehensive, production-ready tutorial demonstrating distributed observability with OpenTelemetry, Loki, Grafana, and Jaeger. Includes realistic load testing with k6.

## 🚀 Quick Start

```bash
# 1. Start the infrastructure
docker-compose up -d

# 2. Run the application
RUST_LOG=info ./target/release/otel-tutorial

# 3. Run load test (in another terminal)
./START_K6_TEST.sh

# 4. Open dashboards (no login required!)
open http://localhost:3000    # Grafana (logs + metrics)
open http://localhost:16686   # Jaeger (traces)
open http://localhost:9090    # Prometheus (raw metrics)
```

## 📚 Documentation

### 🌐 View Full Documentation Website

The documentation is now published as a professional MkDocs website!

**[View Documentation Website →](https://yourusername.github.io/otel-tutorial-rust/)**

Or serve locally:
```bash
mkdocs serve
# Visit http://localhost:8000
```

### 📖 Documentation Files

| Section | Files |
|---------|-------|
| **Getting Started** | [Welcome](docs/getting-started/index.md) • [Quick Start](docs/getting-started/quickstart.md) |
| **Learning Guides** | [Architecture](docs/guides/architecture.md) • [Tech Stack](docs/guides/tech-stack.md) • [Onboarding](docs/guides/onboarding.md) • [K6 Testing](docs/guides/k6-testing.md) • [MkDocs Setup](docs/guides/mkdocs-setup.md) |
| **Reference** | [Commands](docs/reference/commands.md) • [Project Contents](docs/reference/project-contents.md) • [Troubleshooting](docs/reference/troubleshooting.md) |
| **API Docs** | [Endpoints](docs/api/endpoints.md) • [Examples](docs/api/examples.md) |

See [DOCS_SETUP.md](DOCS_SETUP.md) for documentation system setup instructions.

## 📁 Project Structure

```
otel-tutorial-rust/
├── src/                          # Rust source code
│   ├── main.rs                   # Application entry point
│   ├── observability.rs          # OTEL & tracing setup
│   ├── handlers.rs               # 5 API endpoints
│   └── custom_middleware.rs      # Request tracking
│
├── config/                       # Service configurations
│   ├── loki-config.yml
│   ├── prometheus.yml
│   ├── promtail-config.yml
│   └── grafana/
│
├── docs/                         # Documentation (organized)
│   ├── getting-started/          # Quick start guides
│   ├── guides/                   # Detailed learning materials
│   └── reference/                # Quick lookup reference
│
├── docker-compose.yml            # Infrastructure as code
├── k6-stress-test.js             # Load testing script
├── START_K6_TEST.sh              # Interactive k6 launcher
├── Cargo.toml                    # Rust dependencies
└── .env.example                  # Configuration template
```

## 🎯 What You Learn

✅ OpenTelemetry instrumentation in Rust
✅ Distributed tracing concepts
✅ Structured logging with Loki
✅ Metrics collection with Prometheus
✅ Real-time visualization with Grafana
✅ Load testing with k6
✅ Performance analysis
✅ Production observability patterns

## 🏗️ Tech Stack

| Component | Purpose |
|-----------|---------|
| **Rust 2024** | Application language |
| **Actix-web 4.4** | HTTP framework |
| **OpenTelemetry 0.20** | Distributed tracing |
| **Tracing 0.1** | Structured logging |
| **Jaeger 1.41** | Trace storage & UI |
| **Loki 2.9.3** | Log aggregation |
| **Prometheus 2.48.0** | Metrics collection |
| **Grafana 10.2.0** | Visualization |
| **k6 (latest)** | Load testing |
| **Docker Compose** | Orchestration |

## 🔧 API Endpoints

```
GET  /api/health             # Health check
POST /api/users              # Create user
GET  /api/users              # List users
GET  /api/users/{id}         # Get user by ID
POST /api/compute            # Fibonacci (CPU-intensive)
```

## 📊 Monitoring Dashboards

| Dashboard | URL | Purpose |
|-----------|-----|---------|
| **Grafana** | http://localhost:3000 | Logs + Metrics visualization |
| **Jaeger** | http://localhost:16686 | Distributed trace visualization |
| **Prometheus** | http://localhost:9090 | Raw metrics & queries |

All dashboards require **no login** - direct access enabled!

## 📈 K6 Load Testing

```bash
# Default test (20 users for ~2 minutes)
./START_K6_TEST.sh

# Light test (5 users for 30 seconds)
k6 run -u 5 -d 30s k6-stress-test.js

# Heavy test (50 users for 5 minutes)
k6 run -u 50 -d 5m k6-stress-test.js
```

The test covers:
- Health checks
- User CRUD operations
- CPU-intensive work (fibonacci)
- Error handling scenarios

## 🎓 Learning Path

**New to observability?**
1. Read [START_HERE.md](docs/getting-started/START_HERE.md) (5 min)
2. Follow [QUICKSTART.md](docs/getting-started/QUICKSTART.md) (5 min)
3. Study [TECH-STACK.md](docs/guides/TECH-STACK.md) (30 min)
4. Complete [ONBOARDING.md](docs/guides/ONBOARDING.md) exercises (1 hour)

**Ready for details?**
- Read [ARCHITECTURE.md](docs/guides/ARCHITECTURE.md)
- Explore the source code
- Run load tests and analyze results

## ⚡ Features

✨ **Production-Ready**
- Proper error handling
- Structured logging
- Request correlation
- Performance monitoring

✨ **Comprehensive**
- Multiple test scenarios
- Real-world patterns
- Best practices
- Complete documentation

✨ **Easy to Use**
- Docker-based setup
- Interactive launchers
- No complex configuration
- All dashboards pre-configured

## 🔍 Quick Commands

```bash
# View all containers
docker-compose ps

# Start everything
docker-compose up -d

# Stop everything
docker-compose down

# View app logs
docker-compose logs -f

# Run app with debug logging
RUST_LOG=debug ./target/release/otel-tutorial

# Make test request
curl http://localhost:8080/api/health
```

## 📖 Documentation Organization

### Getting Started (`docs/getting-started/`)
- [START_HERE.md](docs/getting-started/START_HERE.md) - Your main guide
- [QUICKSTART.md](docs/getting-started/QUICKSTART.md) - Fast setup

### Learning Guides (`docs/guides/`)
- [ARCHITECTURE.md](docs/guides/ARCHITECTURE.md) - Complete architecture reference
- [TECH-STACK.md](docs/guides/TECH-STACK.md) - How each component works
- [ONBOARDING.md](docs/guides/ONBOARDING.md) - Structured learning journey
- [RUN_K6_TEST.md](docs/guides/RUN_K6_TEST.md) - Detailed k6 guide
- [K6_SUMMARY.md](docs/guides/K6_SUMMARY.md) - K6 integration overview

### Reference (`docs/reference/`)
- [REFERENCE.md](docs/reference/REFERENCE.md) - Quick command lookup
- [PROJECT_CONTENTS.txt](docs/reference/PROJECT_CONTENTS.txt) - File inventory
- [PROJECT_SUMMARY.txt](docs/reference/PROJECT_SUMMARY.txt) - Detailed overview

## 🚀 Next Steps

1. **Start here**: Read [docs/getting-started/START_HERE.md](docs/getting-started/START_HERE.md)
2. **Quick setup**: Follow [docs/getting-started/QUICKSTART.md](docs/getting-started/QUICKSTART.md)
3. **Run it**: Execute `./START_K6_TEST.sh`
4. **Learn**: Study [docs/guides/](docs/guides/)
5. **Apply**: Use patterns in your own projects

## 💡 Example: Running Everything

```bash
# Terminal 1: Infrastructure
docker-compose up -d

# Terminal 2: Application
RUST_LOG=info ./target/release/otel-tutorial

# Terminal 3: Load test
./START_K6_TEST.sh

# Terminal 4: Monitor
open http://localhost:3000    # Grafana
open http://localhost:16686   # Jaeger
```

Watch as:
- Requests flow through your system
- Logs appear in Grafana (Loki)
- Traces appear in Jaeger
- Metrics update in Prometheus

## 🎓 What This Teaches

Through hands-on examples, you'll learn:

**Observability**
- How distributed tracing captures request flow
- How structured logging enables searching
- How metrics measure system health
- How to correlate all three

**Best Practices**
- Proper instrumentation patterns
- Request correlation IDs
- Meaningful span names
- Useful log fields
- Production-ready error handling

**Real-World Scenarios**
- Load testing impact
- Performance analysis
- Error identification
- System behavior understanding

## 📝 License

This tutorial is provided as-is for educational purposes.

## 🤝 Contributing

This is a learning resource. Feel free to:
- Modify the code for your needs
- Extend the API endpoints
- Add new instrumentation
- Customize the dashboards
- Create new test scenarios

## ❓ Questions?

Refer to the comprehensive documentation:
- **Getting Started?** → [START_HERE.md](docs/getting-started/START_HERE.md)
- **Setup Issues?** → [QUICKSTART.md](docs/getting-started/QUICKSTART.md)
- **Understanding Architecture?** → [TECH-STACK.md](docs/guides/TECH-STACK.md)
- **Quick Lookup?** → [REFERENCE.md](docs/reference/REFERENCE.md)
- **Loading Testing?** → [RUN_K6_TEST.md](docs/guides/RUN_K6_TEST.md)

---

**Ready to get started? Open [docs/getting-started/START_HERE.md](docs/getting-started/START_HERE.md) now!**

Your complete observability learning platform awaits! 🚀
