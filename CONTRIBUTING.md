# Contributing to Stunnel-Space

Thank you for your interest in contributing to stunnel-space! This document provides guidelines and instructions for contributing.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stunnel.git
   cd stunnel
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/Kannan112/stunnel.git
   ```

## Development Setup

### Prerequisites
- Rust 1.73 or later
- Protocol Buffers compiler (`protoc`)
- Stunnel (for testing)
- Git

### Installing Dependencies

**On Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler stunnel4
```

**On macOS:**
```bash
brew install protobuf stunnel
```

### Building the Project
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Setting Up Environment
```bash
# Copy the example environment file
cp .env.example .env

# Edit with your configuration
nano .env
```

## Making Changes

1. **Create a new branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the [Coding Standards](#coding-standards)

3. **Write or update tests** for your changes

4. **Run the test suite**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit your changes** with a clear commit message:
   ```bash
   git commit -m "feat: add new feature" -m "Detailed description of changes"
   ```

### Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks
- `ci:` CI/CD changes

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests
- Add unit tests in the same file as the code being tested
- Add integration tests in the `tests/` directory
- Ensure all new code has test coverage
- Test both success and error cases

## Submitting Changes

1. **Push your changes** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub:
   - Use a clear, descriptive title
   - Fill out the PR template completely
   - Link any related issues
   - Describe your changes in detail

3. **Wait for review**:
   - Address any feedback from reviewers
   - Make requested changes in new commits
   - Keep the PR updated with the main branch

4. **After approval**:
   - Your PR will be merged by a maintainer
   - Delete your feature branch after merging

## Coding Standards

### Rust Style Guide
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `rustfmt` for automatic formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- All clippy warnings must be fixed

### Best Practices
- Write clear, self-documenting code
- Add doc comments (`///`) for public APIs
- Keep functions small and focused
- Use meaningful variable and function names
- Avoid unwrap() in production code - use proper error handling
- Write comprehensive error messages

### Documentation
- Add rustdoc comments for all public items
- Include examples in doc comments where appropriate
- Update README.md for user-facing changes
- Update CHANGELOG.md for all changes

## Reporting Bugs

Found a bug? Please report it using our [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md).

Include:
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Relevant logs and error messages
- Minimal reproducible example if possible

## Suggesting Features

Have an idea for a new feature? Use our [Feature Request Template](.github/ISSUE_TEMPLATE/feature_request.md).

Include:
- Clear description of the feature
- Use case and benefits
- Example usage or API design
- Any implementation ideas

## Questions?

- Open a [Discussion](https://github.com/Kannan112/stunnel/discussions) for general questions
- Check existing issues and PRs first
- Tag maintainers if you need help: @Kannan112

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to stunnel-space! 🎉
