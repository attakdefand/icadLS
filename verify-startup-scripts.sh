#!/bin/bash

# Script to verify that the startup scripts are properly configured
# This script checks the content and structure of start-dev.bat and start-dev.sh

echo "Verifying ICALDS Startup Scripts..."
echo "==================================="

echo ""
echo "Checking start-dev.bat..."
if [[ -f "start-dev.bat" ]]; then
    echo "[OK] start-dev.bat found"
    echo "     Checking content..."
    if grep -q "ICALDS development environment" start-dev.bat; then
        echo "[OK] Script contains expected content"
    else
        echo "[WARNING] Script may be outdated or modified"
    fi
else
    echo "[ERROR] start-dev.bat NOT found"
fi

echo ""
echo "Checking start-dev.sh..."
if [[ -f "start-dev.sh" ]]; then
    echo "[OK] start-dev.sh found"
    echo "     Checking content..."
    if grep -q "ICALDS development environment" start-dev.sh; then
        echo "[OK] Script contains expected content"
    else
        echo "[WARNING] Script may be outdated or modified"
    fi
else
    echo "[ERROR] start-dev.sh NOT found"
fi

echo ""
echo "Checking script permissions..."
echo "     Making sure start-dev.sh is executable..."
if [[ -f "start-dev.sh" ]]; then
    if [[ -x "start-dev.sh" ]]; then
        echo "[OK] start-dev.sh is executable"
    else
        echo "[WARNING] start-dev.sh is not executable"
        echo "     Run: chmod +x start-dev.sh"
    fi
else
    echo "[ERROR] Cannot check permissions - start-dev.sh not found"
fi

echo ""
echo "Verification complete."
echo ""
echo "To run the services:"
echo "Windows: start-dev.bat"
echo "Linux/WSL: ./start-dev.sh"