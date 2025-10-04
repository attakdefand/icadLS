# Contributing to ICALDS

Thank you for your interest in contributing to ICALDS (Intelligent Code Analysis and Learning Detection System)! We welcome contributions from the community and are excited to see what you'll bring to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Style Guide](#style-guide)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/icadLS.git`
3. Create a new branch for your feature or bug fix: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Commit your changes: `git commit -am "Add some feature"`
6. Push to the branch: `git push origin feature/your-feature-name`
7. Create a new Pull Request

## How to Contribute

There are many ways you can contribute to ICALDS:

- **Bug Reports**: Submit bug reports for issues you encounter
- **Feature Requests**: Suggest new features or improvements
- **Code Contributions**: Implement new features or fix bugs
- **Documentation**: Improve documentation, examples, or tutorials
- **Testing**: Add test cases or help with testing
- **Translations**: Help translate the project into other languages

## Development Setup

1. Install Rust (1.70 or later)
2. Install Trunk for WASM development: `cargo install trunk`
3. Install the WASM target: `rustup target add wasm32-unknown-unknown`
4. Install Docker (optional, for containerized development)
5. Set up a MySQL database for testing (optional)

For detailed setup instructions, see [DEVELOPMENT.md](docs/DEVELOPMENT.md).

## Style Guide

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `rustfmt` to format your code: `cargo fmt`
- Run Clippy to catch common mistakes: `cargo clippy`
- Write documentation for public APIs
- Include unit tests for new functionality

### Web Interface (Yew/WASM)

- Follow the [Yew Framework conventions](https://yew.rs/docs/)
- Use TypeScript for any JavaScript/TypeScript code
- Follow modern CSS practices

### Documentation

- Use Markdown for documentation
- Follow the [Microsoft Style Guide](https://docs.microsoft.com/en-us/style-guide/welcome/) for technical documentation
- Keep documentation up-to-date with code changes

## Pull Request Process

1. Ensure any install or build dependencies are removed before the end of the layer when doing a build
2. Update the README.md with details of changes to the interface, including new environment variables, exposed ports, useful file locations and container parameters
3. Increase the version numbers in any examples files and the README.md to the new version that this Pull Request would represent. The versioning scheme we use is [SemVer](http://semver.org/)
4. You may merge the Pull Request in once you have the sign-off of two other developers, or if you do not have permission to do that, you may request the second reviewer to merge it for you

## Reporting Issues

### Before Submitting an Issue

1. Check the [troubleshooting guide](TROUBLESHOOTING.md)
2. Search existing issues to see if your issue has already been reported
3. Make sure you're using the latest version of ICALDS

### How to Submit an Issue

When submitting an issue, please include:

1. A clear and descriptive title
2. A detailed description of the problem
3. Steps to reproduce the issue
4. Expected behavior vs. actual behavior
5. Screenshots if applicable
6. Your environment information:
   - Operating System
   - Rust version
   - ICALDS version
   - Any relevant configuration

## Community

If you have questions or need help, feel free to:

- Join our [Discord community](#) (link to be added)
- Ask questions on [Stack Overflow](https://stackoverflow.com/questions/tagged/icadls)
- Email the maintainers at [maintainer@example.com](mailto:maintainer@example.com)

Thank you for contributing to ICALDS!