# ICALDS Troubleshooting Guide

This document provides solutions for common issues you might encounter when running the ICALDS project.

## Diagnostic Tools

The project now includes diagnostic scripts to help identify environment issues:

**Windows:**
```cmd
diagnose.bat
```

**Linux/WSL:**
```bash
./diagnose.sh
```

These scripts will check:
- Required directories
- Rust installation
- Cargo availability
- Trunk installation
- WASM target
- Environment variables

## Environment Verification Scripts

Additional test scripts are available to verify your environment before running the main startup scripts:

**Windows:**
```cmd
test-startup.bat
```

**Linux/WSL:**
```bash
./test-startup.sh
```

These scripts perform similar checks to the diagnostic scripts but are simpler and focused on the essential requirements for running the startup scripts.

## Common Issues and Solutions

### 1. "Could not determine which binary to run" Error

**Problem**: Error message "`cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key."

**Solutions**:
1. **Specify the correct binary**:
   ```bash
   # To run the main API server
   cargo run --bin analyzer-api
   
   # To run database tests
   cargo run --bin test_db
   
   # To fetch Wikipedia algorithms
   cargo run --bin fetch_wikipedia_algorithms
   ```

2. **Add default-run to Cargo.toml** (already done in the workspace root):
   The project is configured as a workspace, so binaries should be specified explicitly.

### 2. "PoolTimedOut" Database Connection Error

**Problem**: Error message "Failed to connect to database: PoolTimedOut"

**Solutions**:
1. **Check if MySQL is running**:
   ```bash
   # Windows (PowerShell)
   Get-Service -Name mysql*
   
   # Linux/WSL
   sudo systemctl status mysql
   # or
   sudo service mysql status
   ```

2. **Start MySQL if it's not running**:
   ```bash
   # Windows (PowerShell as Administrator)
   Start-Service -Name mysql
   
   # Linux/WSL
   sudo systemctl start mysql
   # or
   sudo service mysql start
   ```

3. **Verify database configuration**:
   - Check that you have a `.env` file in the project root
   - Verify the DATABASE_URL is correct:
     ```env
     DATABASE_URL=mysql://username:password@localhost:3306/database_name
     ```

4. **Test database connection**:
   ```bash
   cd services/analyzer-api
   cargo run --bin test_db
   ```

5. **Create database and user if they don't exist**:
   ```sql
   CREATE DATABASE icalds;
   CREATE USER 'icalds_user'@'localhost' IDENTIFIED BY 'your_secure_password';
   GRANT ALL PRIVILEGES ON icalds.* TO 'icalds_user'@'localhost';
   FLUSH PRIVILEGES;
   ```

### 3. Empty Terminal Output

**Problem**: Terminal windows open but show no output or appear empty.

**Solutions**:
1. **Check directory paths**: Ensure you're running the scripts from the project root directory
2. **Verify dependencies**: Make sure all required tools are installed:
   ```bash
   # Check Rust installation
   rustc --version
   
   # Check WASM target
   rustup target list --installed | grep wasm
   
   # Check trunk installation
   trunk --version
   ```
3. **Run services manually**: Try running each service in separate terminals to see detailed output:
   ```bash
   # Terminal 1 - API Server
   cd services/analyzer-api
   cargo run --bin analyzer-api
   
   # Terminal 2 - Web Interface
   cd clients/web-advisor-wasm
   trunk serve --port 8082
   ```

### 4. Port Conflicts

**Problem**: Error messages about ports already being in use.

**Solutions**:
1. **Check for running processes**:
   ```bash
   # Windows
   netstat -ano | findstr :8081
   netstat -ano | findstr :8082
   
   # Linux/WSL
   netstat -tulpn | grep :8081
   netstat -tulpn | grep :8082
   ```
2. **Kill conflicting processes**:
   ```bash
   # Windows
   taskkill /PID <process_id> /F
   
   # Linux/WSL
   kill -9 <process_id>
   ```
3. **Use different ports**:
   ```bash
   # Change API port in services/analyzer-api/src/main.rs
   # Change web interface port in start scripts
   ```

### 5. Blank Web Page

**Problem**: Browser shows a blank page when accessing http://localhost:8082

**Solutions**:
1. **Check if services are running**:
   ```bash
   # Test API health endpoint
   curl http://localhost:8081/health
   
   # Test web interface
   curl http://localhost:8082
   ```
2. **Check browser console**: Open browser developer tools (F12) and check for errors
3. **Verify WASM compilation**:
   ```bash
   cd clients/web-advisor-wasm
   trunk build
   ```

### 6. Build Errors

**Problem**: Compilation errors when running `cargo run` or `trunk serve`

**Solutions**:
1. **Update dependencies**:
   ```bash
   cargo update
   ```
2. **Clean and rebuild**:
   ```bash
   cargo clean
   cargo build
   ```
3. **Check Rust version**:
   ```bash
   rustc --version
   # Should be 1.60 or higher
   ```

## Debugging Steps

### Step 1: Verify Installation
```bash
# Check Rust installation
rustc --version
cargo --version

# Check WASM target
rustup target add wasm32-unknown-unknown

# Check trunk installation
cargo install trunk
```

### Step 2: Test Database Connection
```bash
# Test database connectivity
cd services/analyzer-api
cargo run --bin test_db
```

### Step 3: Run Services Individually
```bash
# Terminal 1 - Start API server
cd services/analyzer-api
cargo run --bin analyzer-api

# Terminal 2 - Start web interface
cd clients/web-advisor-wasm
trunk serve --port 8082
```

### Step 4: Check Logs
```bash
# Check API logs (Windows)
type C:\tmp\icalds-api.log

# Check API logs (Linux/WSL)
cat /tmp/icalds-api.log

# Check web logs (Windows)
type C:\tmp\icalds-web.log

# Check web logs (Linux/WSL)
cat /tmp/icalds-web.log
```

## Windows-Specific Issues

### PowerShell Execution Policy
**Problem**: Scripts won't run due to execution policy

**Solution**:
```powershell
# Run PowerShell as Administrator and execute:
Set-ExecutionPolicy RemoteSigned
```

### Path Issues in WSL
**Problem**: Path conflicts between Windows and WSL

**Solution**:
```bash
# In WSL, navigate to the project directory
cd /mnt/c/Users/YourUsername/Documents/vscodium/concurrence-parallel-async-await-rust/icalds
```

## Linux/WSL-Specific Issues

### Permission Issues
**Problem**: Permission denied when running scripts

**Solution**:
```bash
# Make scripts executable
chmod +x start-dev.sh
chmod +x diagnose.sh
chmod +x test-startup.sh
```

### Display Issues in WSL
**Problem**: GUI applications don't display properly

**Solution**:
1. Install an X-server for Windows (like VcXsrv)
2. Set the DISPLAY variable in WSL:
   ```bash
   export DISPLAY=:0
   ```

## Network and Firewall Issues

### Windows Firewall
**Problem**: Services are running but not accessible

**Solution**:
1. Check Windows Firewall settings
2. Allow Rust and Node.js through the firewall
3. Temporarily disable firewall for testing

### Docker Issues
**Problem**: Docker containers won't start or connect

**Solution**:
```bash
# Check Docker status
docker-compose ps

# Restart Docker containers
docker-compose down
docker-compose up
```

## Performance Issues

### Slow Compilation
**Solution**:
1. Use release builds for better performance:
   ```bash
   cargo run --release --bin analyzer-api
   trunk serve --release
   ```
2. Increase system resources allocated to WSL/Docker

## Getting Help

If you're still experiencing issues:

1. **Check the logs** in `/tmp/icalds-api.log` and `/tmp/icalds-web.log`
2. **Run services manually** to see detailed error messages
3. **Verify all dependencies** are installed correctly
4. **Check GitHub issues** for similar problems
5. **Contact the development team** with detailed error information