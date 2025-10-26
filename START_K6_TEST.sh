#!/bin/bash

# K6 Stress Test Launcher Script
# This script helps you quickly run k6 load tests against the application

set -e

BASE_URL="${BASE_URL:-http://localhost:8080}"
USERS="${1:-20}"
DURATION="${2:-2m}"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           K6 Load Test Launcher                               ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📊 Test Configuration:"
echo "   Base URL:    $BASE_URL"
echo "   Users:       $USERS concurrent users"
echo "   Duration:    $DURATION"
echo ""

# Check if app is responding
echo "✓ Checking if application is running..."
if ! curl -s "${BASE_URL}/api/health" > /dev/null; then
    echo "✗ Application not responding at ${BASE_URL}"
    echo ""
    echo "Please start the application first:"
    echo "  RUST_LOG=info ./target/release/otel-tutorial"
    exit 1
fi
echo "✓ Application is responding"
echo ""

# Check if k6 is installed
if ! command -v k6 &> /dev/null; then
    echo "✗ k6 is not installed"
    echo ""
    echo "Install k6:"
    echo "  macOS:  brew install k6"
    echo "  Linux:  sudo apt-get install k6"
    echo "  Windows: choco install k6"
    exit 1
fi
echo "✓ k6 is installed ($(k6 version))"
echo ""

# Ask user to open dashboards
echo "📈 Open these in your browser to watch live data:"
echo "   • Grafana (logs):    http://localhost:3000"
echo "   • Jaeger (traces):   http://localhost:16686"
echo "   • Prometheus:        http://localhost:9090"
echo ""

echo "Press Enter to start the load test (or Ctrl+C to cancel)..."
read

echo ""
echo "🚀 Starting load test..."
echo "   This will run for approximately $DURATION with ramping load"
echo ""

# Run k6 test
BASE_URL="$BASE_URL" k6 run -u "$USERS" -d "$DURATION" k6-stress-test.js

echo ""
echo "✓ Load test completed!"
echo ""
echo "Next steps:"
echo "  1. Check Grafana for log patterns during the test"
echo "  2. Look at Jaeger for trace timings and spans"
echo "  3. View Prometheus metrics for performance data"
echo "  4. Read RUN_K6_TEST.md for detailed analysis guide"
