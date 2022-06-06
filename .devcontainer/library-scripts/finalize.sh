#!/bin/sh
printf "Running 'postCreateCommand' Script\n"

# Install Rust Targets
printf "Installing Rust Targets\n"
rustup update stable --no-self-update
rustup default stable
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# A code size profiler for wasm
cargo install twiggy cargo-wasi cargo-expand
# wit-bindgen-cli
cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli

# Install Python stuff
printf "Installing Python Dependencies"
pip install mypy wasmtime
