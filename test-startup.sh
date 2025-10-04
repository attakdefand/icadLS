#!/bin/bash

# Test script to verify startup scripts are working
# This script will test the environment and paths

echo "Testing ICALDS Startup Environment..."
echo "====================================="

echo "Current directory: $(pwd)"
echo ""

echo "Checking required directories..."
if [[ -d "services/analyzer-api" ]]; then
    echo "[OK] analyzer-api directory found"
else
    echo "[ERROR] analyzer-api directory NOT found"
fi

if [[ -d "clients/web-advisor-wasm" ]]; then
    echo "[OK] web-advisor-wasm directory found"
else
    echo "[ERROR] web-advisor-wasm directory NOT found"
fi

echo ""
echo "Checking required tools..."
if command -v cargo &> /dev/null; then
    echo "[OK] cargo found"
    cargo --version
else
    echo "[ERROR] cargo NOT found"
fi

if command -v trunk &> /dev/null; then
    echo "[OK] trunk found"
    trunk --version
else
    echo "[ERROR] trunk NOT found"
fi

echo ""
echo "Checking WASM target..."
if rustup target list --installed 2>/dev/null | grep -q wasm; then
    echo "[OK] WASM target installed"
else
    echo "[WARNING] WASM target may not be installed"
    echo "Run: rustup target add wasm32-unknown-unknown"
fi

echo ""
echo "Test complete."
echo ""
echo "If all checks pass, try running ./start-dev.sh"
echo "If you see errors, fix them before running ./start-dev.sh"