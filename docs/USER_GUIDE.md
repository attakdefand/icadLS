# User Guide

## Introduction

ICALDS (Identify Codes, Algorithms, and Data Structures) is a tool that helps you analyze code to identify patterns, algorithms, and data structures. It provides multiple interfaces for different use cases.

## Installation

### Prerequisites

- Rust toolchain (https://www.rust-lang.org/tools/install)
- Docker and Docker Compose (for containerized deployment)
- For web interface: trunk (https://trunkrs.dev/)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd icalds

# Build all components
cargo build --workspace
```

## Usage

### Web Interface

The easiest way to use ICALDS is through the web interface:

1. Start the services:
   ```bash
   cd infra
   docker-compose up
   ```

2. Open your browser and navigate to http://localhost:8080

3. Paste your code in the text area and click "Analyze Code"

### Command-Line Tool

For local analysis without the web interface:

```bash
# Analyze a file
cargo run --bin analyze-algos -- --file path/to/your/code.rs

# Analyze code directly
cargo run --bin analyze-algos -- --code "fn main() { println!(\"Hello\"); }"

# Get JSON output
cargo run --bin analyze-algos -- --file path/to/your/code.rs --format json
```

### REST API

For programmatic access, you can use the REST API:

```bash
# Health check
curl http://localhost:8081/health

# Analyze code
curl -X POST http://localhost:8081/analyze \
  -H "Content-Type: application/json" \
  -d '{"code": "fn main() { let vec = vec![1, 2, 3]; }"}'
```

## Features

### Pattern Detection

ICALDS can identify common coding patterns such as:
- Range-based loops
- Recursive functions
- Iterator usage

### Algorithm Identification

The tool recognizes various algorithmic approaches:
- Sorting algorithms
- Search algorithms
- Graph traversal

### Data Structure Detection

ICALDS identifies data structures in your code:
- Dynamic arrays (Vec)
- Hash maps
- Linked lists
- Trees and graphs

### Complexity Analysis

Get an estimate of your code's complexity:
- Low (0-50 lines)
- Medium (51-100 lines)
- High (100+ lines)

### Recommendations

Receive suggestions for improvement:
- Code organization
- Documentation
- Performance optimizations

## Example Analysis

Given the following code:
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    for i in 0..<vec.len() {
        println!("{}", vec[i]);
    }
}
```

ICALDS would identify:
- Pattern: Range-based loop
- Data structure: Dynamic array
- Complexity: Low
- Recommendation: Consider using an iterator instead of indexing

## Contributing

Contributions are welcome! Please see the development guide for information on how to contribute new analysis capabilities or improve existing ones.