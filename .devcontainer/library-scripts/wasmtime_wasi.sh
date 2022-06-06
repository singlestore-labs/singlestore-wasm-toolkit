#!/bin/sh
WASMTIME_VERSION=0.37.0

# wasmtime
curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz | \
    tar -xJ --wildcards --no-anchored --strip-components 1 -C /usr/bin wasmtime

# wasi-sdk
cd /opt && curl -L https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-16/wasi-sdk-16.0-linux.tar.gz |
    tar -xz
echo 'alias clang=/opt/wasi-sdk-16.0/bin/clang' >>/etc/bash.bashrc
echo 'alias clang++=/opt/wasi-sdk-16.0/bin/clang++' >>/etc/bash.bashrc
