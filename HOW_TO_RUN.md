# How to Run the ICALDS Project

This document explains how to run the ICALDS (Identify Codes, Algorithms, and Data Structures) project.

## Project Architecture

ICALDS consists of three main components:
1. **CLI Tool** - Command-line interface for code analysis
2. **REST API Service** - Web API for code analysis
3. **Web Interface** - Browser-based interface for code analysis

## Prerequisites

- Rust toolchain installed
- `wasm32-unknown-unknown` target for WebAssembly compilation
- Trunk build tool for the web interface
- MySQL database server

Install required tools:
```bash
# Install wasm target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk
```

## Database Setup

1. Install MySQL server (version 8.0 or later)
2. Start MySQL service:
   ```bash
   # Windows (PowerShell as Administrator)
   Start-Service -Name mysql
   
   # Linux/WSL
   sudo systemctl start mysql
   # or
   sudo service mysql start
   ```
3. Log into MySQL as root:
   ```bash
   mysql -u root -p
   ```
4. Create a database for the application:
   ```sql
   CREATE DATABASE icalds;
   ```
5. Create a dedicated user with appropriate permissions:
   ```sql
   CREATE USER 'icalds_user'@'localhost' IDENTIFIED BY 'your_secure_password';
   GRANT ALL PRIVILEGES ON icalds.* TO 'icalds_user'@'localhost';
   FLUSH PRIVILEGES;
   EXIT;
   ```
6. Configure the database connection by creating a `.env` file in the project root:
   ```env
   DATABASE_URL=mysql://icalds_user:your_secure_password@localhost:3306/icalds
   ```
   
   Replace `your_secure_password` with the actual password you used when creating the user.

## Environment Verification

Before running the main startup scripts, you can verify your environment with the test scripts:

**Windows:**
```cmd
test-startup.bat
```

**Linux/macOS/WSL:**
```bash
./test-startup.sh
```

These scripts will check:
- Required directories
- Rust installation
- Cargo availability
- Trunk installation
- WASM target

## Running the Project

### Option 1: Using Docker Compose (Recommended for quick start)

The easiest way to run the complete system with all dependencies is using Docker Compose:

```bash
cd infra
docker-compose up
```

This will start:
- MySQL database server
- API server on port 8081
- Web interface on port 8080

Access the web interface at: http://localhost:8080

### Option 2: Using the Convenience Scripts (Recommended for development)

For Windows, use the batch script:
```cmd
start-dev.bat
```

For Linux/macOS/WSL, use the shell script:
```bash
./start-dev.sh
```

These scripts will automatically start both services in separate windows/processes.
Note: You'll need to have MySQL running separately for this option.

The improved startup scripts provide:
- Better error handling and validation
- Clear status messages
- Process IDs for monitoring
- Log file locations
- Health check information
- Directory validation to prevent path issues

### Option 3: Run API Server First (Manual)

**Terminal 1** - Start the API server:
```bash
cd services/analyzer-api
cargo run --bin analyzer-api
```
The API server will start on http://localhost:8081

**Terminal 2** - Start the web interface:
```bash
cd clients/web-advisor-wasm
trunk serve --port 8082
```
The web interface will be available at http://localhost:8082

### Option 4: Run Web Interface First (Manual)

**Terminal 1** - Start the web interface:
```bash
cd clients/web-advisor-wasm
trunk serve --port 8082
```

**Terminal 2** - Start the API server:
```bash
cd services/analyzer-api
cargo run --bin analyzer-api
```

All approaches work. The web interface will show connection status and automatically connect to the API when it becomes available.

Note: Options 2-4 require a separate MySQL database to be running.

## Using the Application

1. Open your browser and navigate to http://localhost:8082
2. You should see the ICALDS web interface with:
   - Connection status indicator (will show "Connected to analysis engine")
   - Code input area with example code
   - "Analyze Code" button
3. Enter or modify the code in the text area
4. Click "Analyze Code" to get analysis results
5. View the analysis results in the results section below

## API Endpoints

The REST API provides the following endpoints:
- `GET /health` - Health check endpoint
- `POST /analyze` - Code analysis endpoint

Example API usage:
```bash
# Health check
curl http://localhost:8081/health

# Code analysis
curl -X POST http://localhost:8081/analyze \
  -H "Content-Type: application/json" \
  -d '{"code":"fn main() { println!(\"Hello, world!\"); }"}'
```

## CLI Tool Usage

You can also use the command-line interface:
```bash
cd services/tools/analyze-algos
cargo run -- --help
```

## Database Testing

To test the database integration:
```bash
cd services/analyzer-api
cargo run --bin test_db
```

This will:
- Connect to the configured MySQL database
- Create the required tables if they don't exist
- Insert a test code sample
- Insert a test analysis result

## Wikipedia Algorithm Fetching

To fetch and store algorithms from Wikipedia:
```bash
cd services/analyzer-api
cargo run --bin fetch_wikipedia_algorithms
```

This will:
- Connect to the configured MySQL database
- Fetch algorithm information from Wikipedia
- Store the information in the wikipedia_algorithms table
- Display a summary of fetched algorithms

## Advanced Wikipedia Algorithm Fetching

To fetch detailed algorithm information from Wikipedia with advanced features:
```bash
cd services/analyzer-api
cargo run --bin advanced_wikipedia_fetch
```

This provides enhanced fetching capabilities with better error handling and more detailed information.

## Algorithm Testing

To test the algorithm detection system:
```bash
cd services/analyzer-api
cargo run --bin test_algorithms
```

This will run tests on the algorithm detection system with various code samples.

## Optimized Builds

For production deployments, you can build optimized versions of the services:

**API Server (Release Build):**
```bash
cd services/analyzer-api
cargo build --release --bin analyzer-api
```

**Web Interface (Release Build):**
```bash
cd clients/web-advisor-wasm
trunk build --release
```

Release builds will:
- Enable full optimizations (`opt-level = 3`)
- Remove debug symbols to reduce binary size
- Enable Link Time Optimization (LTO)
- Reduce codegen units for better optimization

Note: Release builds take longer to compile but result in faster and smaller binaries.

## Enhanced Algorithm Detection

The ICALDS system now includes an advanced algorithm detection system that:

1. Identifies over 20 common algorithms across multiple categories
2. Provides detailed information about each detected algorithm
3. Includes complexity analysis and educational resources
4. Offers links to Wikipedia articles for further learning

The system detects algorithms through pattern matching, keyword analysis, and name recognition. For a complete list of supported algorithms and implementation details, see [ALGORITHM_DETECTION.md](docs/ALGORITHM_DETECTION.md).

## Troubleshooting

### Port Conflicts
If you encounter port conflicts:
- API server: Change port in analyzer-api/src/main.rs
- Web interface: Use a different port with `trunk serve --port <port>`

### WASM Compilation Issues
If you encounter WASM compilation errors:
```bash
# Ensure the wasm target is installed
rustup target add wasm32-unknown-unknown

# Clean and rebuild
cd clients/web-advisor-wasm
rm -rf target
trunk serve --port 8082
```

### Empty Terminal Output
If terminal windows open but show no output:

1. Check that you're running the scripts from the correct directory (project root)
2. Verify all dependencies are installed correctly
3. Try running services manually in separate terminals to see detailed output
4. Check the troubleshooting guide at [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more detailed solutions

### Connection Issues
If the web interface shows connection errors:
1. Ensure the API server is running on port 8081
2. Check browser console for CORS or network errors
3. Verify that both services are accessible in your browser