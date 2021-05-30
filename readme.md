zkSNARK Prover in WASM
=======================

1. Install dependency:
```bash
cargo install wasm-pack
```

2. Build Package:
```bash
wasm-pack build --target web
```

3. Serve the website locally:
```bash
python3 -m http.server
```

WASM setting is derived from [Mozilla Doc](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#install_node.js_and_npm).