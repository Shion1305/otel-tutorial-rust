# Documentation Index

Welcome! This folder contains all documentation for the OpenTelemetry + Rust + K6 tutorial.

## 📍 Start Here

**New to this tutorial?**
→ Start with [`getting-started/START_HERE.md`](getting-started/START_HERE.md)

**Want to get running in 5 minutes?**
→ Follow [`getting-started/QUICKSTART.md`](getting-started/QUICKSTART.md)

## 📂 Documentation Structure

### 1. Getting Started (`getting-started/`)
For people just beginning with the tutorial.

| Document | Purpose |
|----------|---------|
| [START_HERE.md](getting-started/START_HERE.md) | **START HERE** - Main entry point explaining the project |
| [QUICKSTART.md](getting-started/QUICKSTART.md) | 5-minute setup guide with step-by-step instructions |

**Best for:**
- First-time users
- Understanding what the project does
- Getting the system running quickly

---

### 2. Guides (`guides/`)
For learning and understanding the system in depth.

| Document | Purpose |
|----------|---------|
| [ARCHITECTURE.md](guides/ARCHITECTURE.md) | Complete architecture reference (previously README.md) |
| [TECH-STACK.md](guides/TECH-STACK.md) | Deep dive into how each component works |
| [ONBOARDING.md](guides/ONBOARDING.md) | Structured learning journey with phases and exercises |
| [RUN_K6_TEST.md](guides/RUN_K6_TEST.md) | Comprehensive guide to k6 load testing |
| [K6_SUMMARY.md](guides/K6_SUMMARY.md) | K6 integration overview and reference |

**Best for:**
- Understanding the architecture
- Learning observability concepts
- Running and analyzing load tests
- Hands-on exercises

---

### 3. Reference (`reference/`)
For quick lookup when you need information fast.

| Document | Purpose |
|----------|---------|
| [REFERENCE.md](reference/REFERENCE.md) | Quick lookup: URLs, ports, commands, queries |
| [PROJECT_CONTENTS.txt](reference/PROJECT_CONTENTS.txt) | Complete file listing and project inventory |
| [PROJECT_SUMMARY.txt](reference/PROJECT_SUMMARY.txt) | Detailed project overview and statistics |

**Best for:**
- Finding specific commands
- Looking up URLs and ports
- API examples
- Configuration reference

---

## 🎯 Quick Navigation by Use Case

**I'm completely new and want a quick overview**
1. [START_HERE.md](getting-started/START_HERE.md) (5 min)
2. [QUICKSTART.md](getting-started/QUICKSTART.md) (5 min)

**I want to understand the architecture**
1. [TECH-STACK.md](guides/TECH-STACK.md) (30 min)
2. [ARCHITECTURE.md](guides/ARCHITECTURE.md) (detailed reference)

**I want to run load tests and see results**
1. [RUN_K6_TEST.md](guides/RUN_K6_TEST.md)
2. [K6_SUMMARY.md](guides/K6_SUMMARY.md)

**I need a specific command or URL**
1. [REFERENCE.md](reference/REFERENCE.md)

**I want to do a structured learning path**
1. [ONBOARDING.md](guides/ONBOARDING.md)

**I need complete project details**
1. [PROJECT_CONTENTS.txt](reference/PROJECT_CONTENTS.txt)
2. [PROJECT_SUMMARY.txt](reference/PROJECT_SUMMARY.txt)

---

## 📊 Documentation Statistics

| Folder | Documents | Purpose |
|--------|-----------|---------|
| `getting-started/` | 2 | Quick setup |
| `guides/` | 5 | Learning materials |
| `reference/` | 3 | Quick lookup |
| **Total** | **10** | **Complete tutorial** |

**Total words:** ~50,000 words of comprehensive documentation

---

## 🗺️ Project Layout

```
otel-tutorial-rust/
├── README.md                 ← Main project README
├── docs/
│   ├── INDEX.md             ← This file
│   ├── getting-started/
│   │   ├── START_HERE.md    ← Your entry point
│   │   └── QUICKSTART.md    ← 5-minute setup
│   ├── guides/
│   │   ├── ARCHITECTURE.md
│   │   ├── TECH-STACK.md
│   │   ├── ONBOARDING.md
│   │   ├── RUN_K6_TEST.md
│   │   └── K6_SUMMARY.md
│   └── reference/
│       ├── REFERENCE.md
│       ├── PROJECT_CONTENTS.txt
│       └── PROJECT_SUMMARY.txt
├── src/                     ← Rust source code
├── config/                  ← Service configurations
├── k6-stress-test.js        ← Load testing script
├── docker-compose.yml       ← Infrastructure
└── Cargo.toml              ← Dependencies
```

---

## 🔍 Finding What You Need

### By Topic

**Getting Started**
- [START_HERE.md](getting-started/START_HERE.md) - Project overview
- [QUICKSTART.md](getting-started/QUICKSTART.md) - Setup instructions

**Architecture & Design**
- [ARCHITECTURE.md](guides/ARCHITECTURE.md) - Complete reference
- [TECH-STACK.md](guides/TECH-STACK.md) - Component explanations
- [PROJECT_CONTENTS.txt](reference/PROJECT_CONTENTS.txt) - File structure

**Learning & Training**
- [ONBOARDING.md](guides/ONBOARDING.md) - Structured learning
- [TECH-STACK.md](guides/TECH-STACK.md) - Concept explanations
- [ARCHITECTURE.md](guides/ARCHITECTURE.md) - Advanced topics

**Load Testing**
- [RUN_K6_TEST.md](guides/RUN_K6_TEST.md) - Detailed guide
- [K6_SUMMARY.md](guides/K6_SUMMARY.md) - Quick overview
- [REFERENCE.md](reference/REFERENCE.md) - Commands

**Quick Lookup**
- [REFERENCE.md](reference/REFERENCE.md) - URLs, ports, commands
- [PROJECT_SUMMARY.txt](reference/PROJECT_SUMMARY.txt) - Statistics

---

## ⏱️ Estimated Reading Times

| Document | Time | Effort |
|----------|------|--------|
| START_HERE.md | 5 min | Easy |
| QUICKSTART.md | 5 min | Easy |
| TECH-STACK.md | 30 min | Medium |
| ARCHITECTURE.md | 30 min | Medium |
| ONBOARDING.md | 15 min | Easy |
| RUN_K6_TEST.md | 20 min | Medium |
| K6_SUMMARY.md | 10 min | Easy |
| REFERENCE.md | Variable | Quick lookup |

**Total comprehensive learning:** ~2 hours

---

## 🎓 Recommended Learning Sequence

### Day 1: Experience It (45 minutes)
1. Read [START_HERE.md](getting-started/START_HERE.md) (5 min)
2. Follow [QUICKSTART.md](getting-started/QUICKSTART.md) (5 min)
3. Get it running and explore dashboards (15 min)
4. Skim [TECH-STACK.md](guides/TECH-STACK.md) overview (20 min)

### Day 2: Understand It (1.5 hours)
1. Read full [TECH-STACK.md](guides/TECH-STACK.md) (30 min)
2. Read [ONBOARDING.md](guides/ONBOARDING.md) (15 min)
3. Do the exercises (20 min)
4. Review [ARCHITECTURE.md](guides/ARCHITECTURE.md) sections (25 min)

### Day 3: Analyze & Apply (1 hour)
1. Complete [RUN_K6_TEST.md](guides/RUN_K6_TEST.md) (20 min)
2. Run load tests and analyze results (20 min)
3. Read [K6_SUMMARY.md](guides/K6_SUMMARY.md) (10 min)
4. Plan your own use cases (10 min)

---

## 🚀 Getting Started Right Now

### Step 1: Choose Your Path

**Option A: Fast Track** (10 minutes)
- Read [START_HERE.md](getting-started/START_HERE.md)
- Follow [QUICKSTART.md](getting-started/QUICKSTART.md)

**Option B: Thorough Learning** (2 hours)
- Follow the "Recommended Learning Sequence" above

**Option C: Jump to Specific Topic**
- Find what you need in "Finding What You Need" section

### Step 2: Open the File

Click on the document from the lists above, or:
```
docs/getting-started/START_HERE.md  # Best starting point
docs/getting-started/QUICKSTART.md  # Fast setup
```

### Step 3: Follow Along

Each document includes instructions for what to do next.

---

## 📚 Key Concepts Explained

Documents explain these key concepts:

| Concept | Document |
|---------|----------|
| Traces & Spans | [TECH-STACK.md](guides/TECH-STACK.md) |
| Structured Logging | [ARCHITECTURE.md](guides/ARCHITECTURE.md) |
| Metrics Collection | [TECH-STACK.md](guides/TECH-STACK.md) |
| OTEL Instrumentation | [ARCHITECTURE.md](guides/ARCHITECTURE.md) |
| K6 Load Testing | [RUN_K6_TEST.md](guides/RUN_K6_TEST.md) |
| Best Practices | [ARCHITECTURE.md](guides/ARCHITECTURE.md) |

---

## ✅ What Each Document Provides

**START_HERE.md**
- ✓ Project overview
- ✓ Learning map
- ✓ File navigation
- ✓ Quick start commands

**QUICKSTART.md**
- ✓ Step-by-step setup
- ✓ Troubleshooting
- ✓ Common issues
- ✓ Testing endpoints

**TECH-STACK.md**
- ✓ Architecture explanation
- ✓ Component details
- ✓ Data flow diagrams
- ✓ Performance info

**ARCHITECTURE.md**
- ✓ Complete reference
- ✓ API documentation
- ✓ Advanced topics
- ✓ Production patterns

**ONBOARDING.md**
- ✓ Learning phases
- ✓ Hands-on exercises
- ✓ Key concepts
- ✓ Progress milestones

**RUN_K6_TEST.md**
- ✓ K6 usage guide
- ✓ Test scenarios
- ✓ Metrics explanation
- ✓ Advanced options

**K6_SUMMARY.md**
- ✓ Integration overview
- ✓ Quick reference
- ✓ Performance expectations
- ✓ Learning exercises

**REFERENCE.md**
- ✓ URLs and ports
- ✓ API commands
- ✓ Docker commands
- ✓ Query examples
- ✓ Troubleshooting table

---

## 🎯 Your Next Step

**Ready to begin?**

→ **Open [`getting-started/START_HERE.md`](getting-started/START_HERE.md) now!**

It will guide you through everything you need to know.

---

**Questions about navigation?** This INDEX.md file is your guide!
