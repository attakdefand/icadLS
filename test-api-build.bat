@echo off
REM Test script to verify that the analyzer-api can be built and run

echo Testing ICALDS API Build...
echo ==========================

echo Changing to analyzer-api directory...
cd services\analyzer-api

echo.
echo Building analyzer-api...
cargo build --bin analyzer-api
if %errorlevel% == 0 (
    echo [SUCCESS] Build completed successfully
) else (
    echo [ERROR] Build failed
    cd ..\..
    pause
    exit /b 1
)

echo.
echo Checking available binaries...
cargo run --bin analyzer-api -- --help >nul 2>&1
if %errorlevel% == 0 (
    echo [SUCCESS] analyzer-api binary is available
) else (
    echo [ERROR] analyzer-api binary not found
    cd ..\..
    pause
    exit /b 1
)

echo.
echo Test completed successfully!
echo You can now run the API server with:
echo   cargo run --bin analyzer-api
cd ..\..
pause