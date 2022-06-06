#!/usr/bin/env bash
set -eu

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]:-$0}"; )" &> /dev/null && pwd 2> /dev/null; )";
REPO_DIR="$(dirname "${SCRIPT_DIR}")"

pushd "${REPO_DIR}" > /dev/null
docker build -f ".devcontainer/Dockerfile" -t "singlestore-wasm-toolkit-shell" .
popd > /dev/null

docker run -it --rm \
    -v "${REPO_DIR}":/workspaces/singlestore-wasm-toolkit \
    -w /workspaces/singlestore-wasm-toolkit \
    singlestore-wasm-toolkit-shell \
    /bin/bash