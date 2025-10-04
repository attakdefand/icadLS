@echo off
REM Script to make shell scripts executable in WSL

echo Making shell scripts executable in WSL...
echo =========================================

echo Checking if WSL is available...
where wsl >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] WSL is available
    echo.
    echo Making scripts executable...
    wsl -e bash -c "cd /mnt/c/Users/RMT/Documents/vscodium/concurrence-parallel-async-await-rust/icalds && chmod +x start-dev.sh 2>/dev/null && echo [OK] start-dev.sh is now executable"
    wsl -e bash -c "cd /mnt/c/Users/RMT/Documents/vscodium/concurrence-parallel-async-await-rust/icalds && chmod +x test-startup.sh 2>/dev/null && echo [OK] test-startup.sh is now executable"
    wsl -e bash -c "cd /mnt/c/Users/RMT/Documents/vscodium/concurrence-parallel-async-await-rust/icalds && chmod +x diagnose.sh 2>/dev/null && echo [OK] diagnose.sh is now executable"
    wsl -e bash -c "cd /mnt/c/Users/RMT/Documents/vscodium/concurrence-parallel-async-await-rust/icalds && chmod +x verify-startup-scripts.sh 2>/dev/null && echo [OK] verify-startup-scripts.sh is now executable"
) else (
    echo [WARNING] WSL not found - cannot make scripts executable
    echo You'll need to make the scripts executable manually in WSL:
    echo   chmod +x start-dev.sh
    echo   chmod +x test-startup.sh
    echo   chmod +x diagnose.sh
    echo   chmod +x verify-startup-scripts.sh
)

echo.
echo Setup complete.
pause