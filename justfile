default:
    just --list

# Run CLI example with subset-ibm.csv
run-cli-subset:
    cargo run -p univ-csv-stats-core --example cli -- /Users/jeff/Downloads/ibm-trans-anti-money-laundering/subset-ibm.csv

# Run CLI example
run-cli *ARGS:
    cargo run -p univ-csv-stats-core --example cli -- {{ ARGS }}

# Build all targets
build:
    cargo build --workspace --release

# Build core library
build-core:
    cargo build -p univ-csv-stats-core --release

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

test-core:
    cargo test -p univ-csv-stats-core

[working-directory('bindings/python')]
test-python:
    ./.venv/bin/pytest

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
