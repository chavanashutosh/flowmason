# Contributing to FlowMason

Thank you for your interest in contributing to FlowMason! This document provides guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a branch for your changes
4. Make your changes
5. Submit a pull request

## Development Setup

See [Getting Started](getting-started.md) for setup instructions.

## Code Style

- Follow Rust formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Write tests for new features
- Document public APIs

## Pull Request Process

1. **Update Documentation**: Update relevant documentation
2. **Add Tests**: Include tests for new features
3. **Update CHANGELOG**: Document your changes
4. **Submit PR**: Create a pull request with a clear description

## Areas for Contribution

### New Bricks

Add integrations with new services:

1. Create a new brick in `crates/bricks/src/`
2. Implement the `Brick` trait
3. Add configuration schema
4. Update brick list in API
5. Add documentation

### Bug Fixes

Report bugs via GitHub issues. Include:

- Description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details

### Documentation

Improve documentation:

- Fix typos and errors
- Add examples
- Improve clarity
- Add missing information

### Testing

Add tests:

- Unit tests for individual functions
- Integration tests for flows
- API endpoint tests

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

## Questions?

Open an issue on GitHub for questions or discussions.

