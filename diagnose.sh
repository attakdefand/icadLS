#!/bin/bash

# Diagnostic script to check ICALDS environment
# Usage: ./diagnose.sh

echo "ICALDS Environment Diagnostic"
echo "============================="
echo

echo "Current Directory:"
pwd
echo

echo "Checking for required directories..."
if [[ -d "services/analyzer-api" ]]; then
    echo "[OK] services/analyzer-api directory found"
else
    echo "[ERROR] services/analyzer-api directory NOT found"
fi

if [[ -d "clients/web-advisor-wasm" ]]; then
    echo "[OK] clients/web-advisor-wasm directory found"
else
    echo "[ERROR] clients/web-advisor-wasm directory NOT found"
fi

echo
echo "Checking for required tools..."
echo

if command -v rustc &> /dev/null; then
    echo "[OK] Rust compiler found"
    rustc --version
else
    echo "[ERROR] Rust compiler NOT found"
    echo "Please install Rust from https://www.rust-lang.org/"
fi

echo
if command -v cargo &> /dev/null; then
    echo "[OK] Cargo found"
    cargo --version
else
    echo "[ERROR] Cargo NOT found"
fi

echo
if command -v trunk &> /dev/null; then
    echo "[OK] Trunk found"
    trunk --version
else
    echo "[ERROR] Trunk NOT found"
    echo "Please install trunk with: cargo install trunk"
fi

echo
echo "Checking for WASM target..."
if rustup target list --installed 2>/dev/null | grep -q wasm; then
    echo "[OK] WASM target installed"
else
    echo "[WARNING] WASM target not installed"
    echo "Please install with: rustup target add wasm32-unknown-unknown"
fi

echo
echo "Environment Variables:"
echo "PATH: $PATH"
echo

echo "Diagnostic complete."