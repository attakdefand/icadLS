# ICALDS Development Guide

## Table of Contents

- [Project Structure](#project-structure)
- [Development Environment Setup](#development-environment-setup)
- [Building and Running](#building-and-running)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Contributing](#contributing)
- [Architecture](#architecture)

## Project Structure

```
icalds/
├── services/
│   ├── analyzer-api/          # REST API service
│   └── tools/
│       └── analyze-algos/     # Command-line tool
├── clients/
│   └── web-advisor-wasm/      # Web interface (Yew/WASM)
├── examples/                  # Sample code files
├── infra/                     # Docker configuration
├── docs/                      # Documentation
├── scripts/                   # Utility scripts
├── analysis/                  # Analysis tools and configurations
└── Cargo.toml                 # Workspace configuration
```

## Development Environment Setup

### Prerequisites

1. Install Rust (1.70 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Trunk for WASM development:
   ```bash
   cargo install trunk
   ```

3. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. Install Docker (optional, for containerized development)

5. Install MySQL (optional, for database features)

### IDE Recommendations

- **VS Code** with Rust Analyzer extension
- **IntelliJ IDEA** with Rust plugin
- **Vim/Neovim** with rust.vim plugin

## Building and Running

### Building the Project

```bash
# Build all workspace members
cargo build

# Build in release mode
cargo build --release
```

### Running Services

#### REST API Server

```bash
cd services/analyzer-api
cargo run
```

The API will be available at http://localhost:8081

#### Web Interface

```bash
cd clients/web-advisor-wasm
trunk serve --port 8082
```

The web interface will be available at http://localhost:8082

#### Command-Line Tool

```bash
# Run directly
cargo run --bin analyze-algos -- --file examples/sample_code.rs

# Or build and run the binary
cargo build --bin analyze-algos
./target/debug/analyze-algos --file examples/sample_code.rs
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p analyzer-api

# Run tests with verbose output
cargo test -- --nocapture
```

### Writing Tests

Tests should follow Rust conventions:

1. Unit tests in the same file as the code (within `#[cfg(test)]` modules)
2. Integration tests in the `tests/` directory
3. Clear, descriptive test names
4. Test edge cases and error conditions

Example test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut data = vec![3, 1, 4, 1, 5];
        bubble_sort(&mut data);
        assert_eq!(data, vec![1, 1, 3, 4, 5]);
    }
}
```

## Code Quality

### Formatting

Use `rustfmt` to format code:

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt --all -- --check
```

### Linting

Use Clippy for additional linting:

```bash
# Run clippy
cargo clippy

# Run clippy with error level
cargo clippy -- -D warnings
```

### Documentation

Generate documentation:

```bash
# Generate and open documentation
cargo doc --open
```

Ensure all public APIs are documented with examples.

## Contributing

Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed contribution guidelines.

## Architecture

### Overview

ICALDS follows a microservices architecture with three main components:

1. **REST API Service** (`analyzer-api`): Provides HTTP endpoints for code analysis
2. **Web Interface** (`web-advisor-wasm`): Yew-based WASM application for browser usage
3. **Command-Line Tool** (`analyze-algos`): Standalone CLI tool for local analysis

### Data Flow

```
[User Code] → [Analysis Engine] → [Results]
     ↑              ↓              ↓
[Input]      [Algorithm Detection] [Output]
             [Data Structure ID]   
```

### Key Components

#### Analysis Engine

The core analysis engine is responsible for:

- Parsing Rust code
- Detecting algorithm patterns
- Identifying data structures
- Providing complexity analysis
- Generating recommendations

#### Database Integration

MySQL is used for persistent storage of:

- Analysis results
- User preferences (future feature)
- Historical data (future feature)

#### Web Interface

The web interface is built with:

- Yew framework for WASM-based UI
- Tailwind CSS for styling
- REST API for backend communication

### Extending Functionality

#### Adding New Algorithms

1. Add detection logic in the analysis module
2. Update the algorithm database
3. Add tests for the new algorithm
4. Update documentation

#### Adding New Data Structures

1. Implement detection patterns
2. Add to the data structure catalog
3. Include educational content
4. Add tests

### Performance Considerations

- Use efficient parsing algorithms
- Cache analysis results when appropriate
- Minimize memory allocations
- Profile performance regularly