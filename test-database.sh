#!/bin/bash

# Test database connection with current .env configuration

echo "Testing Database Connection with Current Configuration..."
echo "======================================================="

echo "Current .env contents:"
cat .env
echo ""

echo "Testing database connection..."
echo "============================="
cd services/analyzer-api
echo "Running: cargo run --bin test_db"
cargo run --bin test_db
if [ $? -eq 0 ]; then
    echo ""
    echo "[SUCCESS] Database connection test passed!"
else
    echo ""
    echo "[ERROR] Database connection test failed!"
    echo "Please check:"
    echo "1. MySQL service is running"
    echo "2. Database credentials in .env file are correct"
    echo "3. Database and user exist"
fi
cd ../..
echo ""