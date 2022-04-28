# Rust+WASM contracts

This guide assumes a Unix-like environment.

## Prerequisites

- Rust nightly toolchain (see [rustup.rs](https://rustup.rs))
- Rust wasm32-unknown-unkown target
- Toolchain zip from PBC (insert link)


## Create a new contract

1. Extract the toolchain to a folder: `/tmp/pbc-rust-wasm`
1. Inside the folder you will find a folder called `token-contract`.
1. To compile the example contract  run `cargo +nightly build --target wasm32-unknown-unknown` from the `token-contract` folder
1. There should now be a WASM file named `token-contract.wasm` in `/tmp/pbc-rust-wasm/token-contract/target/wasm32-unknown-unknown/debug/`.
