# Contributing to efl-rs

Thank you for your interest in contributing to efl-rs! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone --recurse-submodules https://github.com/your-username/efl-rs.git
   cd efl-rs
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Dependencies

The `efltk-sys` crate uses `pkg-config` and `bindgen`, so you need EFL development headers, pkg-config, and libclang.

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y libefl-all-dev pkg-config clang libclang-dev
```

#### Linux (Fedora)
```bash
sudo dnf install -y efl-devel pkg-config clang-devel
```

#### macOS
```bash
brew install efl pkg-config llvm
```

#### Windows (MSYS2 clang64)

CI builds Windows with [MSYS2](https://www.msys2.org/) `clang64`. From an MSYS2 clang64 shell:

```bash
pacboy -S efl:p rust:p pkg-config:p clang:p
```

Set `EFL_DIR` only if Elementary is installed outside the default MSYS2 prefix.

### Building

```bash
# Check the code compiles
cargo check --workspace

# Build in release mode
cargo build --release --workspace

# Library tests (no display required)
cargo test --workspace --lib

# Run clippy (linter)
cargo clippy --workspace --lib --examples

# Format code
cargo fmt --all
```

`make check-all` runs format check, clippy, and tests.

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
- Use `EflError` / `EflResult` and `CStringExt` for FFI strings
- Document error conditions in doc comments
- Empty widget wrappers should be detectable with `is_set()`; `as_raw()` still panics if misused

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

Target `alpha/*` for the current development cycle, or `main` for hotfixes.

## Testing

```bash
cargo test --workspace --lib
cargo test --workspace --lib --all-features
```

Library tests must pass without a display server. Interactive examples still need a running EFL/Elementary install.

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
