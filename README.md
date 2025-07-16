# univ-csv-stats

## Description

Rust library which will calculate stats for a csv file on disk and returns them.

Provides a rust CLI example and python and node.js bindings

## Structure

univ-csv-stats/
│
├── Cargo.toml         # Workspace root manifest
├── justfile           # Task runner configuration
│
├── core/              # Core library implementation
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
│
├── examples/          # CLI moved to examples
│   └── cli.rs
│
├── bindings/          # Unified bindings folder
│   ├── python/        # PyO3 Python bindings
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── nodejs/        # Node.js NAPI bindings
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
│
└── README.md

## Python setup

```sh
uv tool install maturin # add $HOME/.local/bin to $PATH or use uvx to run maturin
cd bindings/python
maturin init --name univ-csv-stats-python --mixed -b pyo3 -v
echo "3.13.5" > .python-version
echo "layout uv" > .envrc
uv init
maturin develop --uv
uv add --dev pytest
uv run pytest
```

## Node.js setup

```
 pnpm add -g @napi-rs/cli

```

## WASM setup

```
cargo install wasm-pack
cargo install cargo-generate
cargo install wasm-bindgen-cli
wasm-pack new # or just create
wasm-pack build --target nodejs --out-dir wasm/pkg
```
