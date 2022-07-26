#!/bin/sh
# assumes rust and cargo are already installed

rustup target add wasm32-wasi wasm32-unknown-unknown

rustup component add rust-src rustfmt clippy

cargo install \
        cargo-wasi \
        cargo-expand \
        cargo-get \
        cargo-workspaces \
        cargo-cache \
        mdbook \
        mdbook-linkcheck
