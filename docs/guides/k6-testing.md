# K6 Load Testing Guide

This guide explains how to use k6 to stress test your observability system and generate realistic data across traces, logs, and metrics.

## What is k6?

k6 is a modern load testing tool that:
- Simulates multiple concurrent users
- Tests API endpoints under stress
- Generates realistic performance data
- Creates distributed traces and logs
- Measures latency, throughput, and error rates

## Prerequisites

```bash
# Verify k6 is installed
k6 version

# If not installed on macOS:
brew install k6

# On Linux:
sudo apt-get install k6

# On Windows:
choco install k6
```

## Setup

Before running the test, ensure:

### 1. Start the Observability Stack
```bash
docker-compose up -d
```

### 2. Start the Rust Application
```bash
# Terminal 1
RUST_LOG=info ./target/release/otel-tutorial
```

### 3. Verify Everything is Running
```bash
# Check all containers
docker-compose ps

# Check app is responding
curl http://localhost:8080/api/health
```

## Running the Stress Test

### Basic Test (Default Settings)

```bash
# Run with default settings (5-20 concurrent users for ~2 minutes)
k6 run k6-stress-test.js
```

### Custom Load Profile

```bash
# Light load: 5 users for 30 seconds
k6 run -u 5 -d 30s k6-stress-test.js

# Medium load: 20 users for 1 minute
k6 run -u 20 -d 1m k6-stress-test.js

# Heavy load: 50 users for 2 minutes
k6 run -u 50 -d 2m k6-stress-test.js

# Ramp up gradually (built-in to script)
k6 run k6-stress-test.js
```

### Custom Server URL

```bash
# Point to different server
BASE_URL=http://example.com:8080 k6 run k6-stress-test.js
```

### With Verbose Logging

```bash
# Show detailed output
k6 run -v k6-stress-test.js

# Show only important metrics
k6 run --summary-export=summary.json k6-stress-test.js
```

## What the Test Does

### Test Scenarios (repeated in each iteration):

1. **Health Check**
   - Simple GET /api/health
   - Verifies server is responding
   - Baseline performance check

2. **User CRUD Operations**
   - Create a new user (POST)
   - List all users (GET)
   - Get specific user (GET)
   - Handle 404/error cases

3. **CPU-Intensive Work**
   - Fibonacci computation (POST)
   - Tests with n=10, 15, or 20
   - Generates longer spans and traces
   - Useful for performance analysis

4. **Error Handling**
   - Invalid email format
   - Invalid fibonacci parameter (n > 50)
   - Tests error logging and tracing

### Load Stages (Built-in):

```
Ramp-up (0-40s):     5 → 20 users
Sustained (40-100s): 20 users (steady state)
Ramp-down (100-140s): 20 → 0 users
```

Total runtime: ~2-3 minutes

## Monitoring During Test

### Terminal 1: Application Logs
```bash
# Watch the app's JSON logs as requests arrive
RUST_LOG=debug ./target/release/otel-tutorial | jq
```

### Terminal 2: K6 Output
```bash
# Running the test
k6 run k6-stress-test.js
```

### Terminal 3: Watch Live Data

#### In Grafana (Logs):
```bash
# Open http://localhost:3000
# Explore → Loki
# Query: {container="otel-tutorial"} | json | level="INFO"
# Watch logs appear in real-time
```

#### In Jaeger (Traces):
```bash
# Open http://localhost:16686
# Select "otel-tutorial" service
# Watch traces populate with operation timings
```

#### In Prometheus (Metrics):
```bash
# Open http://localhost:9090
# Query: up (shows all services)
# Watch real-time data
```

## Understanding K6 Output

When you run k6, you'll see metrics like:

```
✓ status is 200                       1234 / 1234  (100%)
✗ status is 201                        500 / 1000  (50%)

  http_req_duration..............: avg=245.3ms  min=12ms   max=5.2s    p(95)=512ms p(99)=1.3s
  http_req_failed................: 2.4%
  http_reqs......................: 5400  23.3/s
  iteration_duration..............: avg=8.2s    min=4.1s   max=15.3s
  iterations......................: 600   2.6/iter/s
```

**Key Metrics:**
- `http_req_duration` - How long requests took
- `http_req_failed` - Percentage of failed requests
- `http_reqs` - Total requests and rate
- `iterations` - How many test cycles completed
- `p(95)` - 95th percentile (most users experience this latency)
- `p(99)` - 99th percentile (worst case for most users)

## Analyzing Results in Observability

### 1. Performance Dashboard (Prometheus)

```promql
# Request rate
rate(http_requests_total[1m])

# P95 latency
histogram_quantile(0.95, request_duration_seconds)

# Error rate
rate(http_errors_total[1m])
```

### 2. Log Analysis (Loki/Grafana)

```logql
# All requests during test
{container="otel-tutorial"} | json
  | duration_ms > 1000  # Requests > 1 second

# Error logs during test
{container="otel-tutorial"} | level="ERROR"

# Fibonacci operations
{container="otel-tutorial"} | json
  | message =~ "Fibonacci|Computing"
```

### 3. Trace Analysis (Jaeger)

- Look at `compute_fibonacci` spans during high load
- See how response time increases with concurrent users
- Identify hot spots: which operations are slowest?
- Check `http_request` span times

## Custom Test Scenarios

To modify the test, edit `k6-stress-test.js`:

### Change Load Profile
```javascript
export const options = {
  stages: [
    { duration: '20s', target: 50 },  // Ramp to 50 users
    { duration: '2m', target: 50 },   // Stay for 2 minutes
    { duration: '10s', target: 0 },   // Ramp down
  ],
};
```

### Add New Endpoint Test
```javascript
group('New Feature', () => {
  const res = http.get(`${BASE_URL}/api/new-endpoint`);

  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 1000ms': (r) => r.timings.duration < 1000,
  });

  duration.add(res.timings.duration);
  successRate.add(res.status === 200);
});
```

### Adjust Thresholds
```javascript
thresholds: {
  'http_req_duration': ['p(95)<500', 'p(99)<1000'],  // Stricter
  'errors': ['rate<0.05'],                           // < 5% errors
},
```

## Common Issues

### "Connection refused"
- Make sure the Rust app is running on port 8080
- Check: `curl http://localhost:8080/api/health`

### "Server is not responding"
- Verify containers are running: `docker-compose ps`
- Check logs: `docker-compose logs`
- Restart app: `./target/release/otel-tutorial`

### k6 Command Not Found
- Install k6: `brew install k6` (macOS)
- Verify: `k6 version`

### High Error Rate
- Check app logs for issues
- Look at request payloads in the script
- May need to increase response timeout thresholds

## What to Observe

### During the Test

1. **Logs in Grafana**
   - Watch request rate increase as load ramps up
   - See error logs for invalid fibonacci/email
   - Observe JSON structured logs with all fields

2. **Traces in Jaeger**
   - See span count increase with load
   - Notice trace duration increases under load
   - Identify slow operations

3. **Metrics in Prometheus**
   - Request rate (should increase to ~20 req/s at peak)
   - Response time (p95 should be < 1s)
   - Error rate (should be < 10%)

### After the Test

Compare before/after:
- Average response time with/without load
- Error rates under stress
- How traces look during high throughput
- Log volume and searchability

## Advanced: Export Results

```bash
# Export to JSON
k6 run --out json=results.json k6-stress-test.js

# Export to CSV
k6 run --out csv=results.csv k6-stress-test.js

# Send to cloud
k6 cloud run k6-stress-test.js
```

## Next Steps

1. Run the basic test: `k6 run k6-stress-test.js`
2. Observe data in Grafana, Jaeger, Prometheus
3. Modify load profile and run again
4. Analyze how your app performs under different loads
5. Optimize based on what you see in traces/logs

---

The goal is to generate realistic data that populates your entire observability system, so you can see how to use it in practice!
