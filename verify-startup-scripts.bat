@echo off
REM Script to verify that the startup scripts are properly configured
REM This script checks the content and structure of start-dev.bat and start-dev.sh

echo Verifying ICALDS Startup Scripts...
echo ===================================

echo.
echo Checking start-dev.bat...
if exist "start-dev.bat" (
    echo [OK] start-dev.bat found
    echo      Checking content...
    findstr /C:"ICALDS development environment" start-dev.bat >nul
    if %errorlevel% == 0 (
        echo [OK] Script contains expected content
    ) else (
        echo [WARNING] Script may be outdated or modified
    )
) else (
    echo [ERROR] start-dev.bat NOT found
)

echo.
echo Checking start-dev.sh...
if exist "start-dev.sh" (
    echo [OK] start-dev.sh found
    echo      Checking content...
    findstr /C:"ICALDS development environment" start-dev.sh >nul
    if %errorlevel% == 0 (
        echo [OK] Script contains expected content
    ) else (
        echo [WARNING] Script may be outdated or modified
    )
) else (
    echo [ERROR] start-dev.sh NOT found
)

echo.
echo Checking script permissions...
echo      Making sure start-dev.sh is executable...
REM Note: Windows doesn't have chmod, but WSL does
echo      If you're using WSL, run: chmod +x start-dev.sh

echo.
echo Verification complete.
echo.
echo To run the services:
echo Windows: start-dev.bat
echo Linux/WSL: ./start-dev.sh
pause