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
- Type parameters should be single uppercase letters (T, U, V) or descriptive names (Widget, Value)

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
- Consider starting the commit message with an applicable emoji:
  - `✨` for new features
  - `🐛` for bug fixes
  - `📝` for documentation
  - `🔧` for configuration changes
  - `⚡` for performance improvements
  - `🎨` for UI/UX changes
  - `🚀` for releases

### Commit Structure
- Each commit should be a single logical change
- Keep commits small and focused
- Include tests for bug fixes and new features
- Update documentation as needed

## Pull Request Process

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub:
   - Use a clear, descriptive title
   - Include a detailed description of the changes
   - Reference any related issues
   - Include screenshots if applicable

3. **Address Review Comments**:
   - Respond to all review comments
   - Make requested changes or explain why they're not needed
   - Update the PR description if the scope changes

4. **Wait for Approval**:
   - At least one maintainer must approve the PR
   - All CI checks must pass
   - The PR will be merged by a maintainer

## Testing

### Running Tests
```bash
cargo test
cargo test --all-features
```

### Adding Tests
- Add unit tests in the same file as the code being tested
- Use `#[cfg(test)]` module for tests
- Test both happy paths and error cases
- Include property-based tests where appropriate

## Reporting Issues

When reporting issues, please include:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your operating system and version
- Your Rust version (`rustc --version`)
- Any relevant code or error messages

## Code Review

When reviewing code:
- Be kind and constructive
- Focus on the code, not the person
- Explain the reasoning behind your suggestions
- Be open to discussion and alternative approaches
- Approve PRs that improve the codebase, even if they're not perfect

## Maintainers

- [@theavege](https://github.com/theavege) - Project maintainer

## License

By contributing to this project, you agree to license your contributions under the [LGPL-2.1-only](LICENSE) license.
