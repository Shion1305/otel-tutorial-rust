/**
 * K6 Load Testing Script for OTEL Tutorial Application
 *
 * This script stress tests the observability system by:
 * - Making concurrent requests to various endpoints
 * - Generating traces, logs, and metrics data
 * - Testing different API operations
 * - Monitoring response times and error rates
 *
 * Usage:
 *   k6 run k6-stress-test.js
 *
 * With custom options:
 *   k6 run -u 50 -d 30s k6-stress-test.js  # 50 users for 30 seconds
 */

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Rate, Trend, Counter, Gauge } from 'k6/metrics';

// Define custom metrics
const errorRate = new Rate('errors');
const duration = new Trend('request_duration');
const successRate = new Rate('success');
const requestCounter = new Counter('requests_total');
const activeUsers = new Gauge('active_users');

// Configuration
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export const options = {
  stages: [
    { duration: '10s', target: 5 },    // Ramp up to 5 users over 10s
    { duration: '30s', target: 20 },   // Ramp up to 20 users over 30s
    { duration: '1m', target: 20 },    // Stay at 20 users for 1 minute
    { duration: '30s', target: 5 },    // Ramp down to 5 users over 30s
    { duration: '10s', target: 0 },    // Ramp down to 0 users over 10s
  ],
  thresholds: {
    'http_req_duration': ['p(95)<1000', 'p(99)<2000'], // 95th percentile < 1s
    'errors': ['rate<0.1'],                              // Error rate < 10%
    'request_duration': ['p(95)<1000'],                  // Custom metric threshold
  },
  // Report results to stdout
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(95)', 'p(99)'],
};

export default function () {
  activeUsers.set(1);
  requestCounter.add(1);

  // Group 1: Health Check
  group('Health Check', () => {
    const res = http.get(`${BASE_URL}/api/health`);

    const success = check(res, {
      'status is 200': (r) => r.status === 200,
      'response time < 100ms': (r) => r.timings.duration < 100,
      'has status field': (r) => r.json('status') !== null,
    });

    duration.add(res.timings.duration);
    successRate.add(success);
    errorRate.add(!success);

    sleep(0.1);
  });

  // Group 2: User Operations
  group('User Operations', () => {
    let userId;

    // Create user
    group('Create User', () => {
      const payload = {
        name: `User-${Math.random() * 100000}`,
        email: `user${Math.random() * 100000}@example.com`,
      };

      const res = http.post(
        `${BASE_URL}/api/users`,
        JSON.stringify(payload),
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      const success = check(res, {
        'status is 201': (r) => r.status === 201,
        'has user id': (r) => r.json('id') !== null,
        'response time < 500ms': (r) => r.timings.duration < 500,
      });

      if (success && res.json('id')) {
        userId = res.json('id');
      }

      duration.add(res.timings.duration);
      successRate.add(success);
      errorRate.add(!success);
    });

    sleep(0.2);

    // List users
    group('List Users', () => {
      const res = http.get(`${BASE_URL}/api/users`);

      const success = check(res, {
        'status is 200': (r) => r.status === 200,
        'is array': (r) => Array.isArray(r.json()),
        'response time < 300ms': (r) => r.timings.duration < 300,
      });

      duration.add(res.timings.duration);
      successRate.add(success);
      errorRate.add(!success);
    });

    sleep(0.2);

    // Get specific user (if we created one)
    if (userId) {
      group('Get User', () => {
        const res = http.get(`${BASE_URL}/api/users/${userId}`);

        const success = check(res, {
          'status is 200': (r) => r.status === 200,
          'has id': (r) => r.json('id') !== null,
          'response time < 200ms': (r) => r.timings.duration < 200,
        });

        duration.add(res.timings.duration);
        successRate.add(success);
        errorRate.add(!success);
      });

      sleep(0.2);
    }

    // Test 404 case
    group('Get Non-existent User', () => {
      const res = http.get(`${BASE_URL}/api/users/nonexistent-user-${Math.random()}`);

      const success = check(res, {
        'status is 500': (r) => r.status === 500,  // Our app returns 500 for not found
        'response time < 200ms': (r) => r.timings.duration < 200,
      });

      duration.add(res.timings.duration);
      successRate.add(success);
      errorRate.add(!success);
    });

    sleep(0.2);
  });

  // Group 3: CPU-Intensive Operations (Fibonacci)
  group('Fibonacci Computation', () => {
    const testCases = [
      { n: 10, expectedTime: 10 },   // Fast
      { n: 15, expectedTime: 50 },   // Medium
      { n: 20, expectedTime: 100 },  // Slower
    ];

    const testCase = testCases[Math.floor(Math.random() * testCases.length)];

    const res = http.post(
      `${BASE_URL}/api/compute`,
      JSON.stringify({ n: testCase.n }),
      {
        headers: {
          'Content-Type': 'application/json',
        },
      }
    );

    const success = check(res, {
      'status is 200': (r) => r.status === 200,
      'has result': (r) => r.json('result') !== null,
      'response time < 5000ms': (r) => r.timings.duration < 5000,
    });

    duration.add(res.timings.duration);
    successRate.add(success);
    errorRate.add(!success);

    console.log(`Fibonacci(${testCase.n}) completed in ${res.timings.duration}ms`);

    sleep(0.5);
  });

  // Group 4: Error Cases
  group('Error Handling', () => {
    // Invalid email
    group('Invalid Email', () => {
      const res = http.post(
        `${BASE_URL}/api/users`,
        JSON.stringify({
          name: 'Invalid User',
          email: 'not-an-email',
        }),
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      const success = check(res, {
        'status is 500': (r) => r.status === 500,
        'response time < 500ms': (r) => r.timings.duration < 500,
      });

      duration.add(res.timings.duration);
      successRate.add(success);
      errorRate.add(!success);
    });

    sleep(0.2);

    // Invalid fibonacci (too large)
    group('Invalid Fibonacci Parameter', () => {
      const res = http.post(
        `${BASE_URL}/api/compute`,
        JSON.stringify({ n: 100 }),  // n > 50 should fail
        {
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      const success = check(res, {
        'status is 500': (r) => r.status === 500,
        'response time < 500ms': (r) => r.timings.duration < 500,
      });

      duration.add(res.timings.duration);
      successRate.add(success);
      errorRate.add(!success);
    });

    sleep(0.2);
  });

  // Random think time between iterations
  sleep(Math.random() * 2);
}

/**
 * Handle setup - runs once before all tests
 */
export function setup() {
  console.log(`Starting load test against ${BASE_URL}`);

  // Verify server is up
  const res = http.get(`${BASE_URL}/api/health`);
  if (res.status !== 200) {
    throw new Error(`Server at ${BASE_URL} is not responding!`);
  }

  console.log('Server is healthy, starting load test...');
}

/**
 * Handle teardown - runs once after all tests
 */
export function teardown(data) {
  console.log('Load test completed!');
}
