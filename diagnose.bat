@echo off
REM Diagnostic script to check ICALDS environment
REM Usage: diagnose.bat

echo ICALDS Environment Diagnostic
echo =============================
echo.

echo Current Directory:
echo %CD%
echo.

echo Checking for required directories...
if exist "services\analyzer-api" (
    echo [OK] services\analyzer-api directory found
) else (
    echo [ERROR] services\analyzer-api directory NOT found
)

if exist "clients\web-advisor-wasm" (
    echo [OK] clients\web-advisor-wasm directory found
) else (
    echo [ERROR] clients\web-advisor-wasm directory NOT found
)

echo.
echo Checking for required tools...
echo.

where rustc >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [OK] Rust compiler found
    rustc --version
) else (
    echo [ERROR] Rust compiler NOT found
    echo Please install Rust from https://www.rust-lang.org/
)

echo.
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [OK] Cargo found
    cargo --version
) else (
    echo [ERROR] Cargo NOT found
)

echo.
where trunk >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [OK] Trunk found
    trunk --version
) else (
    echo [ERROR] Trunk NOT found
    echo Please install trunk with: cargo install trunk
)

echo.
echo Checking for WASM target...
rustup target list --installed | findstr wasm >nul
if %ERRORLEVEL% EQU 0 (
    echo [OK] WASM target installed
) else (
    echo [WARNING] WASM target not installed
    echo Please install with: rustup target add wasm32-unknown-unknown
)

echo.
echo Environment Variables:
echo PATH: %PATH%
echo.

echo Diagnostic complete.
echo.
pause