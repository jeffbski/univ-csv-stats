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
[working-directory('bindings/nodejs')]
build-nodejs:
    napi build --platform --release

# Build Node.js wasm bindings
[working-directory('bindings/nodejs-wasm')]
build-nodejs-wasm:
    wasm-pack build --target nodejs --out-dir wasm/pkg

# Run tests
test:
    cargo test --workspace

test-core:
    cargo test -p univ-csv-stats-core

[working-directory('bindings/nodejs')]
test-nodejs:
    pnpm test

[working-directory('bindings/nodejs-wasm')]
test-nodejs-wasm:
    pnpm test

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
