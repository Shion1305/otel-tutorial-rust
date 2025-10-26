# K6 Load Testing Integration - Complete Summary

## Overview

K6 stress testing has been fully integrated into your observability tutorial project. This allows you to generate realistic load and observability data to see how your entire system performs under stress.

## What You Have

### Files Added
- **k6-stress-test.js** - Comprehensive load testing script with multiple scenarios
- **START_K6_TEST.sh** - Interactive launcher with environment checks
- **RUN_K6_TEST.md** - Detailed guide with advanced options and analysis

### Features

The k6 test script includes:

1. **Multiple Test Groups**
   - Health checks (baseline performance)
   - User CRUD operations (create, read, list)
   - Error handling (invalid inputs)
   - CPU-intensive operations (fibonacci)

2. **Realistic Load Patterns**
   - Ramp up: 5 → 20 users over 40 seconds
   - Sustained: 20 users for 60 seconds
   - Ramp down: 20 → 0 users over 40 seconds
   - Total runtime: ~2-3 minutes

3. **Comprehensive Metrics**
   - Request latency (avg, min, max, p95, p99)
   - Error rates
   - Success rates
   - Throughput (requests/second)
   - Custom duration tracking

## Quick Start

### Prerequisites
```bash
# Ensure k6 is installed
k6 version

# If not installed:
brew install k6  # macOS
```

### 3-Step Launch

```bash
# Terminal 1: Ensure containers are running
docker-compose ps

# Terminal 2: Start the application
RUST_LOG=info ./target/release/otel-tutorial

# Terminal 3: Run the load test
./START_K6_TEST.sh
```

Or directly:
```bash
k6 run k6-stress-test.js
```

## What Gets Generated

During the test, real observability data is created:

### Traces (Jaeger)
- Distributed traces for each request
- Spans showing operation timing
- Request metadata and attributes
- Error information in failed requests

### Logs (Loki)
- Structured JSON logs from the application
- One log entry per request
- Timestamps, levels, and contextual fields
- Searchable by any field

### Metrics (Prometheus)
- Request rate (requests per second)
- Response time histograms
- Error rates
- Throughput measurements

## Usage Examples

### Light Test
```bash
k6 run -u 5 -d 30s k6-stress-test.js
```
- 5 concurrent users
- 30 seconds duration
- Good for quick verification

### Medium Test
```bash
k6 run -u 20 -d 2m k6-stress-test.js
```
- 20 concurrent users
- 2 minute duration
- Default test profile

### Heavy Test
```bash
k6 run -u 50 -d 5m k6-stress-test.js
```
- 50 concurrent users
- 5 minute duration
- Tests app under stress

### Custom Server
```bash
BASE_URL=http://example.com:8080 k6 run k6-stress-test.js
```

## Monitoring During Test

### Watch Logs (Grafana)
1. Open http://localhost:3000
2. Go to Explore
3. Select Loki data source
4. Query: `{container="otel-tutorial"}`
5. Watch logs appear in real-time

### Watch Traces (Jaeger)
1. Open http://localhost:16686
2. Select "otel-tutorial" service
3. Click "Find Traces"
4. Watch new traces populate during test

### Watch Metrics (Prometheus)
1. Open http://localhost:9090
2. Query: `up` (shows service status)
3. Or: `rate(http_requests_total[1m])` (request rate)

## Understanding K6 Output

```
✓ status is 200                       1234 / 1234  (100%)
✓ response time < 500ms               1100 / 1234  (89%)

  http_req_duration..............: avg=245.3ms  min=12ms   max=5.2s    p(95)=512ms p(99)=1.3s
  http_req_failed................: 2.4%
  http_reqs......................: 5400  23.3/s
  iterations......................: 600   2.6/iter/s
  request_duration...............: avg=245.3ms  p(95)=512ms
```

### Key Metrics
- **http_req_duration**: How long HTTP requests took
- **http_req_failed**: Percentage of failed requests
- **http_reqs**: Total requests and rate (req/sec)
- **p(95)**: 95th percentile latency (most users see this or better)
- **p(99)**: 99th percentile latency (worst case for most)
- **iterations**: How many test cycles completed

## Test Scenarios Explained

### 1. Health Check
```
GET /api/health → 200 OK
Expected: < 100ms
Purpose: Baseline performance
```

### 2. Create User
```
POST /api/users → 201 Created
Payload: {name, email}
Expected: < 500ms
Purpose: Write operation performance
```

### 3. List Users
```
GET /api/users → 200 OK
Response: Array of users
Expected: < 300ms
Purpose: Read operation performance
```

### 4. Get User
```
GET /api/users/{id} → 200 OK
Expected: < 200ms
Purpose: Lookup performance
```

### 5. Fibonacci Computation
```
POST /api/compute → 200 OK
Payload: {n: 10|15|20}
Expected: < 5000ms (CPU-bound)
Purpose: Long-running operation performance
```

### 6. Error Cases
```
- Invalid email: POST /api/users with bad email
- Invalid parameter: POST /api/compute with n=100
- Not found: GET /api/users/nonexistent
Expected: Proper error handling
```

## Analysis After Test

### What to Look For

1. **Response Times**
   - Do they increase as load increases?
   - What's the p95/p99 latency?
   - Are there sudden spikes?

2. **Error Rates**
   - Should be very low (< 1%)
   - If high, app is overloaded
   - Check logs for what failed

3. **Throughput**
   - Should be steady
   - Should increase with ramp-up
   - Should decrease with ramp-down

4. **Log Volume**
   - How many logs per request?
   - Are they properly structured?
   - Can you search them effectively?

5. **Trace Quality**
   - Are spans properly nested?
   - Do timings make sense?
   - Are all operations traced?

## Customization

### Modify Load Profile
Edit `k6-stress-test.js`:

```javascript
export const options = {
  stages: [
    { duration: '30s', target: 100 },  // Ramp to 100 users
    { duration: '5m', target: 100 },   // Sustained for 5 min
    { duration: '20s', target: 0 },    // Ramp down
  ],
};
```

### Add New Endpoint
```javascript
group('New Endpoint', () => {
  const res = http.get(`${BASE_URL}/api/new-endpoint`);

  const success = check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 1000ms': (r) => r.timings.duration < 1000,
  });

  duration.add(res.timings.duration);
  successRate.add(success);
  errorRate.add(!success);
});
```

### Adjust Thresholds
```javascript
thresholds: {
  'http_req_duration': ['p(95)<500', 'p(99)<1000'],  // Stricter
  'errors': ['rate<0.05'],                           // < 5% errors
},
```

## Troubleshooting

### "Connection refused"
```bash
# Verify app is running
curl http://localhost:8080/api/health

# Start it
RUST_LOG=info ./target/release/otel-tutorial
```

### "k6 command not found"
```bash
# Install k6
brew install k6

# Verify
k6 version
```

### High error rate during test
1. Check app logs for exceptions
2. Verify request payloads are correct
3. Check if app is crashing
4. Increase performance thresholds if expected

### No data appearing in Grafana
1. Ensure Promtail is running: `docker-compose ps`
2. Check Loki logs: `docker-compose logs loki`
3. Wait 15 seconds for log collection
4. Refresh Grafana page

## Integration with Observability

### This demonstrates:
- How real load generates traces
- How logs appear during high throughput
- How metrics capture performance
- How to correlate all three

### Use cases:
- Performance testing
- Capacity planning
- Identifying bottlenecks
- Validating instrumentation
- Load scenario testing
- Regression testing

## Performance Expectations

For this tutorial application:

| Endpoint | Expected Latency | Expected Rate |
|----------|-----------------|---------------|
| Health Check | < 100ms | > 100 req/s |
| Create User | 100-500ms | > 50 req/s |
| List Users | 100-300ms | > 80 req/s |
| Get User | 50-200ms | > 100 req/s |
| Fibonacci(20) | 100-500ms | > 20 req/s |

These can vary based on your system specs.

## Next Steps

1. **Run a test**: `./START_K6_TEST.sh`
2. **Watch dashboards**: Open Grafana, Jaeger, Prometheus
3. **Analyze results**: Use queries provided in RUN_K6_TEST.md
4. **Modify load**: Try different user counts and durations
5. **Add endpoints**: Create custom test scenarios
6. **Performance tune**: Use insights to optimize your app

## Advanced Topics

### Export Results
```bash
k6 run --out json=results.json k6-stress-test.js
k6 run --out csv=results.csv k6-stress-test.js
```

### Cloud Integration
```bash
k6 cloud run k6-stress-test.js
```

### Combined with Other Tools
- Grafana for visualization
- Prometheus for metrics analysis
- Jaeger for trace analysis
- Loki for log analysis

## Files Reference

| File | Purpose |
|------|---------|
| k6-stress-test.js | Main load testing script |
| START_K6_TEST.sh | Interactive launcher |
| RUN_K6_TEST.md | Detailed guide |
| K6_SUMMARY.md | This file |

## Key Takeaways

✓ k6 provides realistic load testing
✓ Generates authentic observability data
✓ Helps validate instrumentation
✓ Shows real system behavior under stress
✓ Integrates seamlessly with your observability stack
✓ Highly customizable for different scenarios

## Support

For k6 documentation: https://k6.io/docs/

For specific questions about this script, see RUN_K6_TEST.md

---

You now have a complete load testing and observability demonstration system!
