# API Usage Examples

Real-world examples of how to use each endpoint.

## Complete User Workflow

```bash
# 1. Check health
curl http://localhost:8080/api/health

# 2. List users
curl http://localhost:8080/api/users

# 3. Create a user
USER_RESPONSE=$(curl -s -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Smith", "email": "alice.smith@example.com"}')

# Extract the user ID
USER_ID=$(echo $USER_RESPONSE | grep -o '"id":"[^"]*' | head -1 | cut -d'"' -f4)

# 4. Get the specific user
curl http://localhost:8080/api/users/$USER_ID

# 5. List users again to see the new one
curl http://localhost:8080/api/users
```

## Load Testing Simulation

```bash
#!/bin/bash

# Send 10 requests in quick succession
for i in {1..10}; do
  echo "Request $i"
  curl -X POST http://localhost:8080/api/users \
    -H "Content-Type: application/json" \
    -d "{\"name\": \"User$i\", \"email\": \"user$i@example.com\"}"
  sleep 0.1
done
```

## Performance Testing Examples

### Measure Health Check Performance

```bash
# Measure average latency
time for i in {1..100}; do
  curl -s http://localhost:8080/api/health > /dev/null
done
```

### Measure Computation Performance

```bash
# Quick computation
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 20}'

# Medium computation
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 25}'

# Heavy computation
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 30}'
```

## Error Handling Examples

### Validation Errors

```bash
# Missing required field
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John"}'
# Response: 400 Bad Request

# Invalid email format
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "email": "not-an-email"}'
# Response: 400 Bad Request

# Invalid computation parameter
curl -X POST http://localhost:8080/api/compute \
  -H "Content-Type: application/json" \
  -d '{"n": 100}'
# Response: 400 Bad Request (n must be <= 35)
```

### Not Found Errors

```bash
# User that doesn't exist
curl http://localhost:8080/api/users/550e8400-0000-0000-0000-000000000000
# Response: 404 Not Found
```

## Real-World Usage Patterns

### Pattern 1: User Registration Flow

```bash
# 1. Register new user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New User",
    "email": "newuser@example.com"
  }'

# 2. Get user details
curl http://localhost:8080/api/users/{returned_id}

# 3. List all users to verify
curl http://localhost:8080/api/users
```

### Pattern 2: Batch Processing

```bash
#!/bin/bash
# Create multiple users
NAMES=("Alice" "Bob" "Charlie" "Diana" "Eve")
DOMAINS=("gmail.com" "yahoo.com" "outlook.com" "company.com" "university.edu")

for name in "${NAMES[@]}"; do
  domain=${DOMAINS[$RANDOM % ${#DOMAINS[@]}]}
  email="${name,,}@${domain}"
  
  curl -X POST http://localhost:8080/api/users \
    -H "Content-Type: application/json" \
    -d "{\"name\": \"$name\", \"email\": \"$email\"}"
  
  sleep 0.5
done
```

### Pattern 3: Monitoring and Alerting

```bash
#!/bin/bash
# Check health and alert if down
while true; do
  if ! curl -s http://localhost:8080/api/health > /dev/null 2>&1; then
    echo "ALERT: Application is down!"
    # Send alert here (email, Slack, etc.)
  else
    echo "✓ Application is healthy"
  fi
  sleep 30
done
```

## Observability Data Collection Examples

### Collecting Data for Analysis

```bash
# 1. Generate some data
echo "Generating observability data..."

# Health checks (baseline)
for i in {1..5}; do
  curl -s http://localhost:8080/api/health > /dev/null
done

# User operations
for i in {1..3}; do
  curl -s -X POST http://localhost:8080/api/users \
    -H "Content-Type: application/json" \
    -d "{\"name\": \"TestUser$i\", \"email\": \"test$i@example.com\"}" > /dev/null
done

# List operations
for i in {1..5}; do
  curl -s http://localhost:8080/api/users > /dev/null
done

# Computation
for i in {1..3}; do
  curl -s -X POST http://localhost:8080/api/compute \
    -H "Content-Type: application/json" \
    -d '{"n": 25}' > /dev/null
done

echo "Data collection complete!"
echo "Check Grafana: http://localhost:3000"
echo "Check Jaeger: http://localhost:16686"
```

## Integration Examples

### Python Example

```python
import requests
import json

BASE_URL = "http://localhost:8080"

# Health check
response = requests.get(f"{BASE_URL}/api/health")
print(f"Status: {response.json()}")

# Create user
user_data = {
    "name": "Python User",
    "email": "python@example.com"
}
response = requests.post(f"{BASE_URL}/api/users", json=user_data)
user = response.json()
print(f"Created user: {user}")

# Get user
response = requests.get(f"{BASE_URL}/api/users/{user['id']}")
print(f"Retrieved user: {response.json()}")

# Compute
compute_data = {"n": 25}
response = requests.post(f"{BASE_URL}/api/compute", json=compute_data)
result = response.json()
print(f"Computation result: {result}")
```

### JavaScript/Node.js Example

```javascript
const BASE_URL = 'http://localhost:8080';

async function testAPI() {
  // Health check
  const health = await fetch(`${BASE_URL}/api/health`);
  console.log(await health.json());

  // Create user
  const userData = {
    name: 'JavaScript User',
    email: 'js@example.com'
  };
  
  const createResponse = await fetch(`${BASE_URL}/api/users`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(userData)
  });
  const user = await createResponse.json();
  console.log('Created user:', user);

  // Get user
  const getResponse = await fetch(`${BASE_URL}/api/users/${user.id}`);
  console.log('Retrieved user:', await getResponse.json());
}

testAPI();
```

## Load Testing Scenarios

### Baseline Load Test

```bash
# Light load: 5 users for 30 seconds
k6 run -u 5 -d 30s k6-stress-test.js
```

### Standard Load Test

```bash
# Medium load: 20 users for 2 minutes
k6 run -u 20 -d 2m k6-stress-test.js
```

### Stress Test

```bash
# Heavy load: 50 users for 5 minutes
k6 run -u 50 -d 5m k6-stress-test.js
```

## Next Steps

- [View all endpoints](endpoints.md)
- [K6 load testing guide](../guides/k6-testing.md)
- [View traces](http://localhost:16686) in Jaeger
- [Search logs](http://localhost:3000) in Grafana

