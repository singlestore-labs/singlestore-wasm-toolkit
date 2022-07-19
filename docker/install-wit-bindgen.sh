#!/bin/sh

# wit-bindgen is not stable. This pins to a last-known-good
WIT_BINDGEN_REVISION="60e3c5b41e616fee239304d92128e117dd9be0a7"

cargo install                                                           \
        --git https://github.com/bytecodealliance/wit-bindgen \
        --rev $WIT_BINDGEN_REVISION \
        wit-bindgen-cli
cargo cache -r all
