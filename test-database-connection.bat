@echo off
REM Test script to verify database connection

echo Testing Database Connection...
echo =============================

echo Checking if .env file exists...
if exist ".env" (
    echo [OK] .env file found
    echo      Contents:
    type .env
) else (
    echo [WARNING] .env file not found
    echo      You need to create a .env file with your database configuration
    echo      Example: DATABASE_URL=mysql://username:password@localhost:3306/icalds
)

echo.
echo Testing database connectivity...
cd services\analyzer-api
cargo run --bin test_db
cd ..\..

echo.
echo Database test completed.
pause