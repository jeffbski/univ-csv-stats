# univ-csv-stats

## Description

Rust library which will calculate stats for a csv file on disk and returns them as a structure.

Provides CLI examples in rust, python and node.js which use compare native use with using a rust library. Also includes a Node.js wasm implementation.

## Structure

univ-csv-stats/
│
├── Cargo.toml                         # Workspace root manifest
├── justfile                           # Task runner configuration
│
├── core/                              # Core implementation in Rust
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                     # Core Rust library
│       └── main.rs                    # Rust CLI implementation
│
├── bindings/                           # Unified bindings folder
│   ├── python/                         # PyO3 Python bindings
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs                 # PyO3 Rust library wrapper
│   │   └── python/
│   │       ├── cli.py                  # Python CLI calling PyO3 Rust Library
│   │       └── cli-native.py           # Native Python CLI
│   │
│   ├── nodejs/                         # Node.js NAPI-rs bindings
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs                 # NAPI-rs Rust library wrapper
│   │   ├── cli.mjs                     # Node.js CLI calling NAPI-rs Rust Library
│   │   └── cli-native.mjs              # Native Node.js CLI
│   │
│   └── nodejs-wasm/                   # Node.js WASM bindings
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs                 # Node.js WASM library wrapper
│       └── nodejs/
│           └── cli.mjs                 # Node.js CLI calling WASM Rust Library
│
├── test_data/                         # Test CSV files
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

To fix rust analyzer in zed add the following to root/.zed/settings.json
```json
{
  "lsp": {
    "rust-analyzer": {
      "extra_env": {
        "PYO3_PYTHON": "bindings/python/.venv/bin/python"
      }
    }
  }
}
```

or for vscode root/.vscode/settings.json
```json
{
  "rust-analyzer.cargo.extraEnv": {
    "PYO3_PYTHON": "${workspaceFolder}/bindings/python/.venv/bin/python"
  }
}
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
