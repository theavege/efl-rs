# Makefile for efl-rs
# Provides convenient targets for common development tasks

.PHONY: all build check test doc fmt clippy clean

# Default target
all: build

# Build the project
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Build with all features
build-all:
	cargo build --features all

# Check the code compiles
check:
	cargo check

# Run tests
test:
	cargo test

test-all:
	cargo test --all-features

# Generate documentation
doc:
	cargo doc --no-deps --open

doc-all:
	cargo doc --all-features --no-deps --open

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt --check

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

clippy-all:
	cargo clippy --all-features -- -D warnings

# Run all checks (fmt, clippy, test)
check-all: fmt-check clippy test

# Clean build artifacts
clean:
	cargo clean

# Install the library
install:
	cargo install --path efltk

# Run examples
run-simple:
	cargo run --example simple

# Show help
help:
	@echo "Available targets:"
	@echo "  all          - Build the project (default)"
	@echo "  build        - Build in debug mode"
	@echo "  release      - Build in release mode"
	@echo "  build-all    - Build with all features"
	@echo "  check        - Check compilation"
	@echo "  test         - Run tests"
	@echo "  test-all     - Run tests with all features"
	@echo "  doc          - Generate documentation"
	@echo "  doc-all      - Generate documentation with all features"
	@echo "  fmt          - Format code"
	@echo "  fmt-check    - Check formatting"
	@echo "  clippy       - Run clippy linter"
	@echo "  clippy-all   - Run clippy with all features"
	@echo "  check-all    - Run all checks (fmt, clippy, test)"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install the library"
	@echo "  run-simple   - Run the simple example"
