# ICALDS Project Overview

## Introduction

ICALDS (Intelligent Code Analysis and Learning Detection System) is an advanced tool for analyzing and identifying various codes, algorithms, and data structures. The system provides multiple interfaces for different use cases:

- A web-based advisor interface for interactive analysis
- A REST API for programmatic access
- Command-line tools for local analysis

## Key Features

- Code pattern recognition
- Advanced algorithm detection (20+ algorithms across 14 categories)
- Detailed algorithm information with complexity analysis
- Educational content with Wikipedia links
- Data structure identification
- Performance recommendations
- Persistent storage of analysis results in MySQL database

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

## Architecture

ICALDS follows a microservices architecture with three main components:

1. **REST API Service** (`analyzer-api`): Provides HTTP endpoints for code analysis
2. **Web Interface** (`web-advisor-wasm`): Yew-based WASM application for browser usage
3. **Command-Line Tool** (`analyze-algos`): Standalone CLI tool for local analysis

## Documentation

- [README.md](../README.md) - Project overview and quick start guide
- [USER_GUIDE.md](USER_GUIDE.md) - Detailed instructions for using ICALDS
- [DEVELOPMENT.md](DEVELOPMENT.md) - Information for contributors and developers
- [API Documentation](API/openapi.yaml) - REST API specification
- [Algorithm Detection](ALGORITHM_DETECTION.md) - Details on how algorithms are detected
- [HOW_TO_RUN.md](../HOW_TO_RUN.md) - Detailed setup and running instructions
- [TROUBLESHOOTING.md](../TROUBLESHOOTING.md) - Solutions to common problems
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Guidelines for contributing to the project
- [CHANGELOG.md](../CHANGELOG.md) - Version history and changes

## Getting Started

For quick setup instructions, see the [README.md](../README.md).

For detailed development setup, see [DEVELOPMENT.md](DEVELOPMENT.md).

## Contributing

We welcome contributions from the community. Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on how to contribute.

## License

This project is licensed under the MIT License. See [LICENSE](../LICENSE) for details.