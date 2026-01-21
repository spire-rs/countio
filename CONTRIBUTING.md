# Contributing

Thank you for your interest in contributing to countio! We appreciate your help
in making this project better.

## Code of Conduct

This project follows the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By
participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/countio.git
   cd countio
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/spire-rs/countio.git
   ```

## Development Setup

### Prerequisites

- Rust 1.85 or later
- Cargo (comes with Rust)

### Building the Project

```bash
# Build with default features (std)
cargo build

# Build with all features
cargo build --all-features

# Run tests
cargo test --all-features

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-features -- -D warnings
```

## Making Changes

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following our code style

3. Add tests for any new functionality

4. Ensure all tests pass:
   ```bash
   cargo test --all-features
   ```

5. Update documentation if needed

## Submitting Changes

1. Commit your changes with clear, descriptive commit messages

2. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub

### Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Update CHANGELOG.md with your changes
- Ensure CI passes (tests, formatting, clippy)

## Reporting Bugs

If you find a bug, please create an issue on GitHub with:

- Steps to reproduce the issue
- Expected vs actual behavior
- Rust version and operating system

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.
