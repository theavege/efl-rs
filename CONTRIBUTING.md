# Contributing to efl-rs

Thank you for your interest in contributing to efl-rs! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/efl-rs.git
   cd efl-rs
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Dependencies

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y libefl-all-dev pkg-config
```

#### Linux (Fedora)
```bash
sudo dnf install -y efl-devel pkg-config
```

#### macOS
```bash
brew install efl pkg-config
```

#### Windows
The build system will automatically download the required EFL libraries.

### Building

```bash
# Check the code compiles
cargo check

# Build in release mode
cargo build --release

# Run clippy (linter)
cargo clippy

# Format code
cargo fmt
```

## Code Style

### Rust Formatting
- Use `cargo fmt` to format code
- All code should be formatted before committing

### Documentation
- All public items (structs, enums, traits, functions) should have doc comments
- Use `///` for doc comments on items
- Use `//!` for module-level documentation
- Include examples in doc comments where helpful
- Document all parameters and return values

### Naming Conventions
- Use `snake_case` for functions, variables, and modules
- Use `PascalCase` for types, traits, and enum variants
- Use `SCREAMING_SNAKE_CASE` for constants

### Error Handling
- Prefer `Result` over `expect()` and `unwrap()`
- Use custom error types for library errors
- Document error conditions in doc comments

## Commit Guidelines

### Commit Messages
- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Fix bug" not "Fixes bug")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally

### Commit Structure
- Each commit should be a single logical change
- Keep commits small and focused
- Include tests for bug fixes and new features
- Update documentation as needed

## Pull Request Process

1. **Push your branch** to your fork
2. **Open a Pull Request** on GitHub with a clear title and description
3. **Address Review Comments**
4. **Wait for Approval**

## Testing

```bash
cargo test
cargo test --all-features
```

## Reporting Issues

Please include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- OS and version
- Rust version
- Relevant code or error messages

## Maintainers

- [@theavege](https://github.com/theavege)

## License

By contributing, you agree to license your contributions under [LGPL-2.1-only](LICENSE).
