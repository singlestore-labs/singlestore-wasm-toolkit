#!/bin/sh

WASMTIME_VERSION="${WASMTIME_VERSION:=0.37.0}"
WASMTIME_URL="${WASMTIME_URL:=https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz}"
curl -L ${WASMTIME_URL} | tar -xJ --wildcards --no-anchored --strip-components 1 -C /usr/bin wasmtime
if [ $? -ne 0 ] ; then
    echo "ERROR: Failed to install Wasmtime"
    exit 1
fi
