@echo off
REM Test script to verify startup scripts are working
REM This script will test the environment and paths

echo Testing ICALDS Startup Environment...
echo =====================================

echo Current directory: %CD%
echo.

echo Checking required directories...
if exist "services\analyzer-api" (
    echo [OK] analyzer-api directory found
) else (
    echo [ERROR] analyzer-api directory NOT found
)

if exist "clients\web-advisor-wasm" (
    echo [OK] web-advisor-wasm directory found
) else (
    echo [ERROR] web-advisor-wasm directory NOT found
)

echo.
echo Checking required tools...
where cargo >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] cargo found
    cargo --version
) else (
    echo [ERROR] cargo NOT found
)

where trunk >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] trunk found
    trunk --version
) else (
    echo [ERROR] trunk NOT found
)

echo.
echo Checking WASM target...
rustup target list --installed | findstr wasm >nul
if %errorlevel% == 0 (
    echo [OK] WASM target installed
) else (
    echo [WARNING] WASM target may not be installed
    echo Run: rustup target add wasm32-unknown-unknown
)

echo.
echo Test complete.
echo.
echo If all checks pass, try running start-dev.bat
echo If you see errors, fix them before running start-dev.bat
pause