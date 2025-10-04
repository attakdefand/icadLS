#!/bin/bash

# Script to start both ICALDS services for development
# Usage: ./start-dev.sh

echo "Starting ICALDS development environment..."
echo "========================================"

# Function to clean up background processes on exit
cleanup() {
    echo "Stopping services..."
    if [[ -n $API_PID ]]; then
        echo "Stopping API server (PID: $API_PID)..."
        kill $API_PID 2>/dev/null
    fi
    if [[ -n $WEB_PID ]]; then
        echo "Stopping Web interface (PID: $WEB_PID)..."
        kill $WEB_PID 2>/dev/null
    fi
    wait $API_PID $WEB_PID 2>/dev/null
    echo "All services stopped."
    exit 0
}

# Trap Ctrl+C and other termination signals
trap cleanup SIGINT SIGTERM

# Check if required directories exist
if [[ ! -d "services/analyzer-api" ]]; then
    echo "ERROR: analyzer-api directory not found!"
    echo "Please make sure you're running this script from the project root directory."
    echo "Current directory: $(pwd)"
    exit 1
fi

if [[ ! -d "clients/web-advisor-wasm" ]]; then
    echo "ERROR: web-advisor-wasm directory not found!"
    echo "Please make sure you're running this script from the project root directory."
    echo "Current directory: $(pwd)"
    exit 1
fi

# Check if required commands exist
if ! command -v cargo &> /dev/null; then
    echo "ERROR: cargo not found. Please install Rust."
    exit 1
fi

if ! command -v trunk &> /dev/null; then
    echo "ERROR: trunk not found. Please install trunk:"
    echo "cargo install trunk"
    exit 1
fi

# Start the API server in the background
echo "Starting API server on port 8081..."
echo "-----------------------------------"
cd services/analyzer-api
echo "Starting cargo run --bin analyzer-api in $(pwd)"
cargo run --bin analyzer-api > /tmp/icalds-api.log 2>&1 &
API_PID=$!
echo "API Server started with PID: $API_PID"
echo "API logs: /tmp/icalds-api.log"
cd ../..

# Wait a moment for the API to start
echo "Waiting for API server to start (5 seconds)..."
sleep 5

# Check if API server is running
if kill -0 $API_PID 2>/dev/null; then
    echo "API Server is running."
else
    echo "WARNING: API Server may have failed to start. Check /tmp/icalds-api.log for details."
fi

# Start the web interface in the background
echo "Starting web interface on port 8082..."
echo "-------------------------------------"
cd clients/web-advisor-wasm
echo "Starting trunk serve in $(pwd)"
trunk serve --port 8082 > /tmp/icalds-web.log 2>&1 &
WEB_PID=$!
echo "Web Interface started with PID: $WEB_PID"
echo "Web logs: /tmp/icalds-web.log"
cd ../..

# Check if Web interface is running
if kill -0 $WEB_PID 2>/dev/null; then
    echo "Web Interface is running."
else
    echo "WARNING: Web Interface may have failed to start. Check /tmp/icalds-web.log for details."
fi

echo ""
echo "Services started!"
echo ""
echo "API Server PID: $API_PID"
echo "Web Interface PID: $WEB_PID"
echo ""
echo "Logs are being written to:"
echo "- API logs: /tmp/icalds-api.log"
echo "- Web logs: /tmp/icalds-web.log"
echo ""
echo "You can now access:"
echo "- Web Interface: http://localhost:8082"
echo "- API Health Check: http://localhost:8081/health"
echo ""
echo "To view logs in real-time, open new terminals and run:"
echo "  tail -f /tmp/icalds-api.log"
echo "  tail -f /tmp/icalds-web.log"
echo ""
echo "Press Ctrl+C to stop both services"

# Wait for both processes
wait $API_PID $WEB_PID