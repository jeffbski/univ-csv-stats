# Run CLI example
run-cli *ARGS:
    cargo run --example cli -- {{ARGS}}

# Build all targets
build:
    cargo build --workspace

# Build Python bindings
build-python:
    maturin build --release -m bindings/python/Cargo.toml

# Build Node.js bindings
build-nodejs:
    npm install -g napi-rs/cli
    napi build --release --manifest-path bindings/nodejs/Cargo.toml

# Run tests
test:
    cargo test --workspace

# Clean all build artifacts
clean:
    cargo clean
    rm -rf target

# Format code
fmt:
    cargo fmt --all

# Lint code
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings
