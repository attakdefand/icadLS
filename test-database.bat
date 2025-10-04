@echo off
REM Test database connection with current .env configuration

echo Testing Database Connection with Current Configuration...
echo =======================================================

echo Current .env contents:
type .env
echo.

echo Testing database connection...
echo =============================
cd services\analyzer-api
cargo run --bin test_db
if %errorlevel% == 0 (
    echo.
    echo [SUCCESS] Database connection test passed!
) else (
    echo.
    echo [ERROR] Database connection test failed!
    echo Please check:
    echo 1. MySQL service is running
    echo 2. Database credentials in .env file are correct
    echo 3. Database and user exist
)
cd ..\..
echo.
pause