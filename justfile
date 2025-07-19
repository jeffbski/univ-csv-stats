default:
    just --list

# Run CLI example with subset-ibm.csv
run-cli-subset:
    cargo run -p univ-csv-stats-core --example cli -- /Users/jeff/Downloads/ibm-trans-anti-money-laundering/subset-ibm.csv

# Run CLI example
run-cli *ARGS:
    cargo run -r -p univ-csv-stats-core --example cli -- {{ ARGS }}

# Build all targets
build: build-core build-python build-nodejs build-nodejs-wasm

# Build all in rust workspace
build-workspace:
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
    pnpm install

# Build Node.js wasm bindings
[working-directory('bindings/nodejs-wasm')]
build-nodejs-wasm:
    wasm-pack build --target nodejs --out-dir wasm/pkg
    pnpm install

# Run all tests
test: test-workspace test-python test-nodejs test-nodejs-wasm

# Run cargo tests in workspace
test-workspace:
    cargo test --workspace

test-core:
    cargo test -p univ-csv-stats-core

[working-directory('bindings/nodejs')]
test-nodejs:
    pnpm install
    pnpm test

[working-directory('bindings/nodejs-wasm')]
test-nodejs-wasm:
    pnpm install
    pnpm test

[working-directory('bindings/python')]
test-python:
    uv venv --allow-existing
    uv run --extra tests pytest

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
