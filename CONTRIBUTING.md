# Contributing to VantisOffice

Thank you for your interest in contributing to VantisOffice! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust 1.70 or later
- CMake 3.20 or later
- Git
- TPM 2.0 hardware (for full functionality)
- Vulkan-compatible GPU (for rendering)

### Setting Up Development Environment

1. Clone the repository:
```bash
git clone https://github.com/vantisCorp/VantisOffice.git
cd VantisOffice
```

2. Install Rust dependencies:
```bash
cargo install cargo-watch
cargo install cargo-tarpaulin
```

3. Build the project:
```bash
./scripts/build.sh
```

## Development Workflow

### Branch Naming

Use descriptive branch names:
- `feature/new-feature`
- `fix/bug-description`
- `docs/update-documentation`
- `refactor/code-improvement`

### Commit Messages

Follow conventional commits:
- `feat: add new feature`
- `fix: resolve issue`
- `docs: update documentation`
- `refactor: improve code structure`

Example:
```
feat(writer): add markdown preview mode

Implement live markdown preview with WYSIWYG rendering.
Resolves #123
```

### Pull Request Process

1. Update documentation
2. Add tests for new functionality
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Submit pull request

## Coding Standards

### Rust Style

Follow the official Rust style guide:
```bash
cargo fmt --check
```

### Linting

Run clippy before committing:
```bash
cargo clippy -- -D warnings
```

### Documentation

Document all public APIs:
```rust
/// Brief description
///
/// More detailed explanation
///
/// # Arguments
///
/// * `arg1` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// let result = function(arg1);
/// ```
pub fn function(arg1: Type) -> ReturnType {
    // implementation
}
```

## Testing

### Unit Tests

Write unit tests for all functions:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let result = function(input);
        assert_eq!(result, expected);
    }
}
```

### Integration Tests

Add integration tests in `tests/` directory.

### Test Coverage

Maintain test coverage above 80%:
```bash
cargo tarpaulin --out Html
```

## Documentation

### README Files

Each module should have a comprehensive README.md with:
- Overview
- Key Features
- Architecture
- API Examples
- Performance Metrics
- Build Requirements

### API Documentation

Generate and check API docs:
```bash
cargo doc --no-deps --open
```

## Security

### Vulnerability Reporting

Report security vulnerabilities privately to:
security@vantis.ai

### Security Best Practices

- Never commit secrets or keys
- Use TPM 2.0 for encryption
- Validate all inputs
- Follow secure coding practices

## Issue Reporting

### Bug Reports

Include:
- VantisOffice version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
- Logs/error messages

### Feature Requests

Include:
- Use case description
- Proposed solution
- Alternative approaches
- Additional context

## Release Process

1. Update version numbers
2. Update CHANGELOG.md
3. Create release branch
4. Run full test suite
5. Create GitHub release
6. Tag version
7. Merge to main

## Getting Help

- Documentation: [docs/](docs/)
- Issues: [GitHub Issues](https://github.com/vantisCorp/VantisOffice/issues)
- Discussions: [GitHub Discussions](https://github.com/vantisCorp/VantisOffice/discussions)

## License

By contributing to VantisOffice, you agree that your contributions will be licensed under the project's license.