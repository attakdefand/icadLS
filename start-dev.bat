@echo off
REM Script to start both ICALDS services for development on Windows
REM Usage: start-dev.bat

echo Starting ICALDS development environment...
echo ========================================

REM Check if required directories exist
if not exist "services\analyzer-api" (
    echo ERROR: analyzer-api directory not found!
    echo Please make sure you're running this script from the project root directory.
    echo Current directory: %CD%
    pause
    exit /b 1
)

if not exist "clients\web-advisor-wasm" (
    echo ERROR: web-advisor-wasm directory not found!
    echo Please make sure you're running this script from the project root directory.
    echo Current directory: %CD%
    pause
    exit /b 1
)

echo Starting API server on port 8081...
echo -----------------------------------
cd services\analyzer-api
echo Starting cargo run --bin analyzer-api in %CD%
start "ICALDS API Server" /D "%CD%" cmd /k "echo API Server Terminal && echo Starting cargo run --bin analyzer-api... && cargo run --bin analyzer-api"
cd ..\..

timeout /t 5 /nobreak >nul

echo Starting web interface on port 8082...
echo -------------------------------------
cd clients\web-advisor-wasm
echo Starting trunk serve in %CD%
start "ICALDS Web Interface" /D "%CD%" cmd /k "echo Web Interface Terminal && echo Starting trunk serve... && trunk serve --port 8082"
cd ..\..

echo.
echo Services started successfully!
echo.
echo Two new command windows have been opened:
echo 1. ICALDS API Server - Running the analyzer API on http://localhost:8081
echo 2. ICALDS Web Interface - Running the web advisor on http://localhost:8082
echo.
echo To stop the services, close the respective command windows.
echo.
echo You can now access:
echo - Web Interface: http://localhost:8082
echo - API Health Check: http://localhost:8081/health
echo.
echo If you don't see the service windows, check:
echo 1. That Rust is properly installed
echo 2. That cargo and trunk are in your PATH
echo 3. That no antivirus is blocking the windows
echo.
echo Press any key to exit this script (services will continue running)...
pause >nul