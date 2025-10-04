# Test database connection with current .env configuration

Write-Host "Testing Database Connection with Current Configuration..."
Write-Host "======================================================="

# Change to the icalds directory
Set-Location "C:\Users\RMT\Documents\vscodium\concurrence-parallel-async-await-rust\icalds"

Write-Host "Current .env contents:"
Get-Content .env
Write-Host ""

Write-Host "Testing database connection..."
Write-Host "============================="
Set-Location "C:\Users\RMT\Documents\vscodium\concurrence-parallel-async-await-rust\icalds\services\analyzer-api"
Write-Host "Running: cargo run --bin test_db"
cargo run --bin test_db
if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "[SUCCESS] Database connection test passed!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "[ERROR] Database connection test failed!" -ForegroundColor Red
    Write-Host "Please check:" -ForegroundColor Yellow
    Write-Host "1. MySQL service is running" -ForegroundColor Yellow
    Write-Host "2. Database credentials in .env file are correct" -ForegroundColor Yellow
    Write-Host "3. Database and user exist" -ForegroundColor Yellow
}
Set-Location "C:\Users\RMT\Documents\vscodium\concurrence-parallel-async-await-rust\icalds"
Write-Host ""