# ICALDS User Guide

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Web Interface Usage](#web-interface-usage)
- [Command-Line Tool Usage](#command-line-tool-usage)
- [REST API Usage](#rest-api-usage)
- [Understanding the Results](#understanding-the-results)
- [Troubleshooting](#troubleshooting)

## Introduction

ICALDS (Intelligent Code Analysis and Learning Detection System) is a powerful tool designed to analyze code and identify algorithms, data structures, and provide educational insights. Whether you're a student learning programming concepts or a developer looking to understand code patterns, ICALDS can help.

## Installation

### Prerequisites

- Rust 1.70 or later
- Docker (optional, for containerized deployment)
- MySQL (for persistent storage)

### Quick Start with Docker

1. Clone the repository:
   ```bash
   git clone https://github.com/attakdefand/icadLS.git
   cd icadLS
   ```

2. Navigate to the infrastructure directory:
   ```bash
   cd infra
   ```

3. Start the services:
   ```bash
   docker-compose up
   ```

4. Access the web interface at http://localhost:8080

### Development Setup

For detailed development setup instructions, see [HOW_TO_RUN.md](../HOW_TO_RUN.md).

## Web Interface Usage

The web interface provides an easy-to-use graphical way to analyze code:

1. Navigate to http://localhost:8080 (or your deployment URL)
2. Paste your code into the text area
3. Click the "Analyze" button
4. Review the results in the analysis panel

### Web Interface Features

- **Real-time Analysis**: Get instant feedback on your code
- **Interactive Results**: Click on detected algorithms and data structures for more information
- **Educational Content**: Access Wikipedia links for deeper understanding
- **Performance Recommendations**: Get suggestions for code optimization

## Command-Line Tool Usage

ICALDS provides a powerful command-line tool for local analysis:

### Basic Usage

```bash
# Analyze a file
cargo run --bin analyze-algos -- --file examples/sample_code.rs

# Analyze code from stdin
echo "fn main() { let vec = vec![1, 2, 3]; }" | cargo run --bin analyze-algos -- --stdin
```

### Output Formats

```bash
# Get JSON output for programmatic use
cargo run --bin analyze-algos -- --file examples/sample_code.rs --format json

# Get detailed output
cargo run --bin analyze-algos -- --file examples/sample_code.rs --verbose
```

### Command-Line Options

- `--file <PATH>`: Analyze code from a file
- `--stdin`: Read code from standard input
- `--format <FORMAT>`: Output format (text, json)
- `--verbose`: Enable verbose output
- `--help`: Show help information

## REST API Usage

The REST API provides programmatic access to ICALDS functionality:

### Health Check

```bash
curl http://localhost:8081/health
```

### Analyze Code

```bash
curl -X POST http://localhost:8081/analyze \
  -H "Content-Type: application/json" \
  -d '{"code": "fn main() { let vec = vec![1, 2, 3]; }"}'
```

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Check service health |
| `/analyze` | POST | Analyze provided code |

For detailed API documentation, see [API Documentation](API/openapi.yaml).

## Understanding the Results

### Algorithms

When ICALDS detects algorithms in your code, it provides:

- **Name**: The name of the algorithm
- **Category**: The category (e.g., Sorting, Searching)
- **Complexity**: Time and space complexity analysis
- **Description**: Detailed explanation of the algorithm
- **Wikipedia Link**: External resource for deeper learning

### Data Structures

For data structures, you'll see:

- **Name**: The name of the data structure
- **Description**: Explanation of the data structure
- **Wikipedia Link**: External resource for more information

### Recommendations

ICALDS may provide performance recommendations such as:

- Suggestions for more efficient algorithms
- Tips for optimizing code patterns
- Best practices for specific use cases

## Troubleshooting

If you encounter issues, check the [TROUBLESHOOTING.md](../TROUBLESHOOTING.md) guide for solutions to common problems.

For additional help, please [open an issue](https://github.com/attakdefand/icadLS/issues) on GitHub.