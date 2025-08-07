# List the just commands
default:
    just --list

# Run CLI example with subset-ibm.csv
run-cli-subset:
    ./target/release/univ-csv-stats /Users/jeff/Downloads/ibm-trans-anti-money-laundering/subset-ibm.csv

# Run CLI example
run-cli *ARGS:
    ./target/release/univ-csv-stats {{ ARGS }}

# Run Python CLI example which uses the rust library
[working-directory('bindings/python')]
run-cli-python *ARGS:
    uv run python/cli.py -- {{ARGS}}

# Run Python native CLI example
[working-directory('bindings/python')]
run-cli-python-native *ARGS:
    uv run python/cli-native.py -- {{ARGS}}

# Run Nodejs CLI example which uses the rust library
[working-directory('bindings/nodejs')]
run-cli-nodejs *ARGS:
    node cli.mjs {{ARGS}}

# Run Nodejs native CLI example
[working-directory('bindings/nodejs')]
run-cli-nodejs-native *ARGS:
    node cli-native.mjs {{ARGS}}

# Run Nodejs-wasm CLI example which uses the rust library
[working-directory('bindings/nodejs-wasm')]
run-cli-nodejs-wasm *ARGS:
    node nodejs/cli.mjs {{ARGS}}

# Build all targets in release mode
build: build-core build-python-wheel build-nodejs build-nodejs-wasm

# Build all in rust workspace
build-workspace:
    cargo build --workspace --release

# Build core library
build-core:
    cargo build -p univ-csv-stats-core --release

# Build Python bindings
build-python:
    maturin build --release -m bindings/python/Cargo.toml

# Build Python wheel
[working-directory('bindings/python')]
build-python-wheel: build-python
   uv run maturin develop --release

# Build Node.js bindings
[working-directory('bindings/nodejs')]
build-nodejs:
    napi build --platform --release
    pnpm install --prefer-offline

# Build Node.js wasm bindings
[working-directory('bindings/nodejs-wasm')]
build-nodejs-wasm:
    wasm-pack build --target nodejs --out-dir wasm/pkg
    pnpm install --prefer-offline

# Run all tests
test: test-workspace test-python test-nodejs test-nodejs-wasm

# Run cargo tests in workspace
test-workspace:
    cargo test --workspace

test-core:
    cargo test -p univ-csv-stats-core

[working-directory('bindings/nodejs')]
test-nodejs:
    napi build --platform
    pnpm install --prefer-offline
    pnpm test

[working-directory('bindings/nodejs-wasm')]
test-nodejs-wasm:
    wasm-pack build --dev --target nodejs --out-dir wasm/pkg
    pnpm install --prefer-offline
    pnpm test

[working-directory('bindings/python')]
test-python:
    uv run maturin develop
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

# Benchmarks - record time and memory

# Run CLI example
bench-run-cli *ARGS:
    /usr/bin/time -l -h -p ./target/release/univ-csv-stats {{ ARGS }}

# Run Python CLI example which uses the rust library
[working-directory('bindings/python')]
bench-run-cli-python *ARGS:
    /usr/bin/time -l -h -p uv run python/cli.py -- {{ARGS}}

# Run Python native CLI example
[working-directory('bindings/python')]
bench-run-cli-python-native *ARGS:
    /usr/bin/time -l -h -p uv run python/cli-native.py -- {{ARGS}}

# Run Nodejs CLI example which uses the rust library
[working-directory('bindings/nodejs')]
bench-run-cli-nodejs *ARGS:
    /usr/bin/time -l -h -p node cli.mjs {{ARGS}}

# Run Nodejs native CLI example
[working-directory('bindings/nodejs')]
bench-run-cli-nodejs-native *ARGS:
    /usr/bin/time -l -h -p node cli-native.mjs {{ARGS}}

# Run Nodejs-wasm CLI example which uses the rust library
[working-directory('bindings/nodejs-wasm')]
bench-run-cli-nodejs-wasm *ARGS:
    /usr/bin/time -l -h -p node nodejs/cli.mjs {{ARGS}}

# Capture stats
capture-stats *ARGS:
    ./bench-serial.sh {{ARGS}} 2>&1 | rg "Output|Count|real|resident"

# Capture mini stats (w/WASM)
capture-mini-stats *ARGS:
    ./bench-mini-serial.sh {{ARGS}} 2>&1 | rg "Output|Count|real|resident"

# Race
race *ARGS:
    ./bench-all.sh {{ARGS}} 2>&1 | rg "Output|Count|real|resident"

# Race with sound
race-sound *ARGS:
    ./bench-all.sh {{ARGS}} 2>&1 | rg "Output|Count|real|resident" --line-buffered | tee /dev/stderr | (read line; afplay ~/Downloads/456966__funwithsound__success-fanfare-trumpets.mp3)
