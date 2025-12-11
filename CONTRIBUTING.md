# Contributing to FlowMason

Thank you for your interest in contributing to FlowMason! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Reporting Issues](#reporting-issues)
- [Submitting Pull Requests](#submitting-pull-requests)
- [Code Style Guidelines](#code-style-guidelines)
- [Areas for Contribution](#areas-for-contribution)
- [License](#license)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. Be kind, constructive, and professional in all interactions.

## Getting Started

**Important Note**: FlowMason does not allow public forks per the license agreement. Instead, please follow these steps:

1. **Clone the repository** (do not create a public fork):
   ```bash
   git clone https://github.com/chavanashutosh/flowmason.git
   cd flowmason
   ```

2. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

3. **Make your changes** following the code style guidelines

4. **Test your changes** thoroughly

5. **Submit a pull request** to the original repository

## Development Setup

See [Getting Started](docs/getting-started.md) for detailed setup instructions.

### Quick Setup

1. **Prerequisites**:
   - Rust 1.70+ ([install Rust](https://rustup.rs/))
   - Dioxus CLI: `cargo install dioxus-cli`
   - PostgreSQL (for database)

2. **Environment Setup**:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## How to Contribute

There are many ways to contribute to FlowMason:

- **Bug Reports**: Report bugs and issues
- **Feature Requests**: Suggest new features
- **Code Contributions**: Submit pull requests with improvements
- **Documentation**: Improve or add documentation
- **Testing**: Add tests or improve test coverage
- **New Bricks**: Add integrations with new services

## Reporting Issues

Before reporting an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** to ensure it's not a configuration issue
3. **Gather relevant information**

### Bug Reports

When reporting a bug, please include:

- **Description**: Clear description of the bug
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**:
  - Operating System and version
  - Rust version (`rustc --version`)
  - FlowMason version or commit hash
- **Logs**: Relevant error messages or logs
- **Screenshots**: If applicable

### Feature Requests

When requesting a feature, please include:

- **Use Case**: Why this feature would be useful
- **Proposed Solution**: How you envision it working
- **Alternatives**: Other solutions you've considered
- **Additional Context**: Any other relevant information

## Submitting Pull Requests

### Before Submitting

1. **Update Documentation**: Update relevant documentation files
2. **Add Tests**: Include tests for new features or bug fixes
3. **Run Tests**: Ensure all tests pass (`cargo test`)
4. **Check Code Style**: Run `cargo fmt` and `cargo clippy`
5. **Update CHANGELOG**: Document your changes (if applicable)

### Pull Request Process

1. **Create a branch** from `main`:
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines

3. **Commit your changes** with clear, descriptive commit messages:
   ```bash
   git commit -m "Add feature: description of what you added"
   ```

4. **Push to your local repository** (not a fork):
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request** on GitHub:
   - Provide a clear title and description
   - Reference any related issues
   - Explain what changes you made and why
   - Include screenshots or examples if applicable

6. **Respond to feedback**: Be responsive to code review comments

### Pull Request Guidelines

- **Keep PRs focused**: One feature or bug fix per PR
- **Write clear descriptions**: Explain what and why, not just how
- **Link related issues**: Use "Fixes #123" or "Closes #123" in PR description
- **Keep commits logical**: Each commit should represent a logical change
- **Request review**: Assign reviewers or ask for feedback

## Code Style Guidelines

### Rust Code Style

- **Formatting**: Use `cargo fmt` to format code
- **Linting**: Run `cargo clippy` and fix warnings
- **Documentation**: Document all public APIs with doc comments
- **Naming**: Follow Rust naming conventions
  - Functions and variables: `snake_case`
  - Types and structs: `PascalCase`
  - Constants: `UPPER_SNAKE_CASE`

### Code Quality

- **Write tests**: Add unit tests for new functions
- **Handle errors**: Use proper error handling (`Result`, `Option`)
- **Avoid panics**: Don't use `unwrap()` or `expect()` in production code
- **Comments**: Add comments for complex logic
- **DRY**: Don't repeat yourself - extract common functionality

### Example

```rust
/// Executes a flow with the given configuration.
///
/// # Arguments
///
/// * `flow_id` - The unique identifier of the flow to execute
/// * `config` - Configuration parameters for the flow execution
///
/// # Returns
///
/// Returns `Ok(ExecutionResult)` on success, or `Err(FlowError)` on failure.
pub async fn execute_flow(
    flow_id: &str,
    config: &FlowConfig,
) -> Result<ExecutionResult, FlowError> {
    // Implementation
}
```

## Areas for Contribution

### New Bricks

Bricks are integrations with external services. To add a new brick:

1. **Create a new brick file** in `crates/bricks/src/`:
   ```rust
   // crates/bricks/src/my_service_brick.rs
   ```

2. **Implement the `Brick` trait**:
   ```rust
   use crate::core::brick_traits::Brick;
   
   pub struct MyServiceBrick;
   
   impl Brick for MyServiceBrick {
       // Implement required methods
   }
   ```

3. **Add configuration schema**:
   - Define input/output types
   - Add validation logic
   - Document configuration options

4. **Register the brick**:
   - Add to `crates/bricks/src/lib.rs`
   - Update API brick list in `services/api/src/routes/bricks.rs`

5. **Add documentation**:
   - Create documentation in `docs/bricks/my-service.md`
   - Add example configuration in `examples/integrations/`

6. **Add tests**:
   - Unit tests for brick logic
   - Integration tests if applicable

### Bug Fixes

- **Identify the bug**: Reproduce and understand the issue
- **Find the root cause**: Trace through the code
- **Fix the bug**: Implement the fix
- **Add tests**: Ensure the bug doesn't regress
- **Document**: Update documentation if behavior changes

### Documentation

Improve documentation by:

- Fixing typos and errors
- Adding missing information
- Improving clarity and examples
- Adding code examples
- Translating documentation (if applicable)

Documentation files are in the `docs/` directory.

### Testing

Add or improve tests:

- **Unit tests**: Test individual functions and methods
- **Integration tests**: Test complete flows and API endpoints
- **Edge cases**: Test error conditions and boundary cases
- **Performance tests**: If applicable

## License

By contributing to FlowMason, you agree that your contributions will be licensed under the same license as the project (FlowMason License Version 2.0). You retain copyright to your contributions but grant the project maintainers the right to use, modify, and distribute your contributions.

## Questions?

If you have questions about contributing:

- **Open an Issue**: For general questions or discussions
- **Check Documentation**: Review existing docs in `docs/`
- **Review Existing Code**: Look at similar contributions for examples

## Recognition

Contributors will be recognized in:
- Project documentation
- Release notes (for significant contributions)
- GitHub contributors list

Thank you for contributing to FlowMason! Your efforts help make the project better for everyone.
