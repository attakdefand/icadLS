# Development Guide

## Project Structure

```
icalds/
├── clients/
│   └── web-advisor-wasm/     # Web frontend using Yew/WASM
├── docs/                     # Documentation
├── infra/                    # Docker deployment files
├── services/
│   ├── analyzer-api/         # REST API service
│   └── tools/
│       └── analyze-algos/    # Command-line tool
└── README.md
```

## Architecture

### Analyzer API

The core analysis service is built with:
- Rust
- Actix-web framework
- Serde for serialization

The API provides two endpoints:
- `GET /health` - Health check endpoint
- `POST /analyze` - Code analysis endpoint

### Web Advisor

The web interface uses:
- Yew framework (Rust to WASM compilation)
- Gloo for web APIs
- Trunk for building and serving

### Command-line Tool

The CLI tool is built with:
- Clap for argument parsing
- Serde for JSON output

## Adding New Analysis Capabilities

### 1. Update the AnalysisResult struct

Add new fields to the AnalysisResult struct in all three components:
- services/analyzer-api/src/main.rs
- services/tools/analyze-algos/src/main.rs
- clients/web-advisor-wasm/src/lib.rs

### 2. Implement the analysis logic

Update the `analyze_code_logic` function in all three components to detect your new pattern/algorithm/data structure.

### 3. Update the UI

For the web interface, update the result display component to show your new analysis results.

## Building and Testing

### Running Locally

```bash
# Run the API service
cd services/analyzer-api
cargo run

# Run the CLI tool
cd services/tools/analyze-algos
cargo run -- --code "fn main() { println!(\"Hello\"); }"

# Run the web interface (requires trunk)
cd clients/web-advisor-wasm
trunk serve
```

### Running with Docker

```bash
cd infra
docker-compose up
```

## Testing

Currently, the project lacks comprehensive tests. To add tests:

1. Create unit tests in each component's `src` directory
2. Add integration tests in a new `tests` directory
3. Consider adding end-to-end tests for the web interface

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection() {
        let code = "for i in 0..<10 {}";
        let result = analyze_code_logic(code);
        assert!(result.patterns.contains(&"Range-based loop".to_string()));
    }
}
```

## Deployment

The project uses Docker Compose for deployment. The docker-compose.yml file defines two services:
1. `api` - The analyzer API service
2. `web` - The web interface

To deploy to production:
1. Build production-ready Docker images
2. Update the docker-compose.yml with appropriate resource limits
3. Add monitoring and logging configurations
4. Configure a reverse proxy (nginx, traefik) for SSL termination

## Future Improvements

1. **Enhanced Analysis**: Implement more sophisticated code analysis using AST parsing
2. **Machine Learning**: Use ML models to improve pattern recognition
3. **Language Support**: Add support for languages beyond Rust
4. **Performance Metrics**: Add execution time and memory usage analysis
5. **Security Analysis**: Identify potential security vulnerabilities
6. **Code Quality**: Integrate with existing linting tools