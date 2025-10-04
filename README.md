# ICALDS - Intelligent Code Analysis and Learning Detection System

[![Build Status](https://github.com/attakdefand/icadLS/workflows/CI/badge.svg)](https://github.com/attakdefand/icadLS/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

![Demo](https://raw.githubusercontent.com/attakdefand/icadLS/main/docs/assets/demo.gif)

ICALDS (Intelligent Code Analysis and Learning Detection System) is an advanced tool for analyzing and identifying various codes, algorithms, and data structures. The project provides:

- A web-based advisor interface for interactive analysis
- A REST API for programmatic access
- Command-line tools for local analysis

## Features

- Code pattern recognition
- Advanced algorithm detection (20+ algorithms across 14 categories)
- Detailed algorithm information with complexity analysis
- Educational content with Wikipedia links
- Data structure identification
- Performance recommendations
- Persistent storage of analysis results in MySQL database

## Live Demo

Check out the live demo of ICALDS: [https://attakdefand.github.io/icadLS](https://attakdefand.github.io/icadLS)

## Getting Started

### Option 1: Using Docker (Recommended for quick start)

1. Install Rust and Docker
2. Run `docker-compose up` in the infra directory
3. Access the web interface at http://localhost:8080

### Option 2: Running Services Directly (For development)

See [HOW_TO_RUN.md](HOW_TO_RUN.md) for detailed instructions on running the services directly without Docker.

This approach requires running two separate terminals:
- One for the REST API server (port 8081)
- One for the web interface (port 8082)

Note: The API server now requires a MySQL database. See the database setup section in [HOW_TO_RUN.md](HOW_TO_RUN.md).

If you encounter any issues, refer to the [TROUBLESHOOTING.md](TROUBLESHOOTING.md) guide for solutions to common problems.

## Project Structure

- `services/analyzer-api`: REST API for analysis services
- `services/tools/analyze-algos`: Command-line analysis tools
- `clients/web-advisor-wasm`: Web-based advisor interface
- `infra`: Docker configuration for deployment
- `examples`: Sample code for testing

## Verification Scripts

The project includes several scripts to help verify your environment and setup:

- `test-startup.bat` / `test-startup.sh` - Basic environment verification
- `diagnose.bat` / `diagnose.sh` - Comprehensive environment diagnostics
- `verify-startup-scripts.bat` / `verify-startup-scripts.sh` - Startup script verification
- `make-scripts-executable.bat` - Makes shell scripts executable in WSL

## Example Usage

### Web Interface

After starting the services with Docker Compose, navigate to http://localhost:8080 and paste code into the text area.

### Command-Line Tool

```bash
# Analyze the sample code
cargo run --bin analyze-algos -- --file examples/sample_code.rs

# Get JSON output
cargo run --bin analyze-algos -- --file examples/sample_code.rs --format json
```

### REST API

```bash
# Health check
curl http://localhost:8081/health

# Analyze code
curl -X POST http://localhost:8081/analyze \
  -H "Content-Type: application/json" \
  -d '{"code": "fn main() { let vec = vec![1, 2, 3]; }"}'
```

## Documentation

- [User Guide](docs/USER_GUIDE.md) - Instructions for using ICALDS
- [Development Guide](docs/DEVELOPMENT.md) - Information for contributors
- [API Documentation](docs/API/openapi.yaml) - REST API specification
- [Algorithm Detection](docs/ALGORITHM_DETECTION.md) - Details on how algorithms are detected

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on how to get started.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.