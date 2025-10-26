# API Endpoints

The OpenTelemetry Tutorial Application provides 5 REST API endpoints for testing observability.

## Health Check

**Endpoint:** `GET /api/health`

**Purpose:** Check if the application is healthy. This is the lightest endpoint - useful for baseline performance measurement.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

**cURL Example:**
```bash
curl http://localhost:8080/api/health
```

**Observability:**
- **Traces:** Very quick span (typically < 5ms)
- **Logs:** Single info log per request
- **Metrics:** Baseline latency measurement

---

## List Users

**Endpoint:** `GET /api/users`

**Purpose:** Retrieve all users from the system. Demonstrates read operations in observability.

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Alice",
    "email": "alice@example.com"
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440002",
    "name": "Bob",
    "email": "bob@example.com"
  }
]
```

**cURL Example:**
```bash
curl http://localhost:8080/api/users
```

**Observability:**
- **Traces:** Shows nested span for list operation
- **Logs:** Logs count of users retrieved
- **Metrics:** Measures query performance

---

## Get User by ID

**Endpoint:** `GET /api/users/{id}`

**Purpose:** Retrieve a specific user by ID. Demonstrates error handling (404 when not found).

**Parameters:**
- `id` (path) - UUID of the user to retrieve

**Response (Success):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "Alice",
  "email": "alice@example.com"
}
```

**Response (Not Found):**
```json
{
  "error": "User not found"
}
```

**cURL Examples:**
```bash
# Get specific user
curl http://localhost:8080/api/users/550e8400-e29b-41d4-a716-446655440001

# Try with invalid ID
curl http://localhost:8080/api/users/invalid-id
```

**Observability:**
- **Traces:** Shows lookup span with user ID
- **Logs:** Logs user lookup attempts and errors
- **Metrics:** Tracks both success and failure rates

---

## Create User

**Endpoint:** `POST /api/users`

**Purpose:** Create a new user. Demonstrates write operations and request validation.

**Request Body:**
```json
{
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Response (Success):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440003",
  "name": "John Doe",
  "email": "john@example.com"
}
```

**Response (Validation Error):**
```json
{
  "error": "Invalid email format"
}
```

**cURL Example:**
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

**Observability:**
- **Traces:** Shows user creation span with ID
- **Logs:** Logs request validation and user creation
- **Metrics:** Tracks write operation latency

---

## Compute (Fibonacci)

**Endpoint:** `POST /api/compute`

**Purpose:** Perform a CPU-intensive computation. This endpoint demonstrates how long-running operations appear in traces.

**Request Body:**
```json
{
  "n": 30
}
```

**Response:**
```json
{
  "input": 30,
  "result": 832040,
  "duration_ms": 45
}
```

**Parameters:**
- `n` (integer) - The Fibonacci number to compute (10-30 recommended)

**cURL Examples:**
```bash
# Quick computation
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 20}'

# Longer computation
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 30}'

# Too large (will error)
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 50}'
```

**Observability:**
- **Traces:** Shows computation span with timing (most interesting to observe!)
- **Logs:** Logs computation start and completion
- **Metrics:** Excellent for measuring CPU usage and latency
- **Load Testing:** Great for stress testing as it uses real CPU

**Why Use This Endpoint?**
- It's CPU-bound, so it shows real computation time
- Helps you see how the system handles long-running operations
- Perfect for load testing to generate sustained load
- Demonstrates proper instrumentation of complex operations

---

## Error Handling

All endpoints handle errors gracefully:

### Bad Request (400)
```bash
# Missing required field
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John"}'
```

### Not Found (404)
```bash
# User doesn't exist
curl http://localhost:8080/api/users/nonexistent
```

### Invalid Data (400)
```bash
# Invalid email format
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "email": "not-an-email"}'

# Invalid computation parameter
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 100}'
```

All errors are logged with full context and appear in traces.

---

## Performance Baselines

Expected response times under normal conditions:

| Endpoint | Expected Latency | Best For |
|----------|------------------|----------|
| GET /api/health | < 10ms | Baseline performance |
| GET /api/users | 10-50ms | List operations |
| GET /api/users/{id} | 5-20ms | Lookup operations |
| POST /api/users | 20-100ms | Write operations |
| POST /api/compute (n=20) | 50-200ms | CPU work |
| POST /api/compute (n=30) | 200-1000ms | Long operations |

---

## Testing Workflow

### 1. Quick Test
```bash
# Just check it's running
curl http://localhost:8080/api/health
```

### 2. Full Test
```bash
# Run all endpoints
./test_api.sh
```

### 3. Load Test
```bash
# Generate realistic load
./START_K6_TEST.sh
```

---

## Next Steps

- [See API Examples](examples.md) for more detailed usage patterns
- [K6 Load Testing](../guides/k6-testing.md) to test endpoints under load
- [View Traces](http://localhost:16686) in Jaeger to see request flow
- [View Logs](http://localhost:3000) in Grafana to see structured logs

