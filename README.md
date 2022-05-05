# What

This repo is intended to be a collection of tools to interact with any Cosmos-compatible Blockchain, e.g. "retrieving a list of validator nodes for a specific chain".

# How

The tools are to be written in rust, then compiled into WebAssembly (.wasm) which should be `import`able in any typical Node.js script & invoked directly e.g. with `node --experimental-wasm-modules` (at writing time).

As a result, for this project, the `Cargo.lock` file is tracked, as [explained here](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html#cargotoml-vs-cargolock)