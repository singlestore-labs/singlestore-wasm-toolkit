# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.234.0/containers/python-3/.devcontainer/base.Dockerfile

# [Choice] Python version (use -bullseye variants on local arm64/Apple Silicon): 3, 3.10, 3.9, 3.8, 3.7, 3.6, 3-bullseye, 3.10-bullseye, 3.9-bullseye, 3.8-bullseye, 3.7-bullseye, 3.6-bullseye, 3-buster, 3.10-buster, 3.9-buster, 3.8-buster, 3.7-buster, 3.6-buster
ARG VARIANT="3.10-bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/python:0-${VARIANT}

# install additional OS packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends mariadb-client

# install node
ARG NODE_VERSION="16"
RUN if [ "${NODE_VERSION}" != "none" ]; then su vscode -c "umask 0002 && . /usr/local/share/nvm/nvm.sh && nvm install ${NODE_VERSION} 2>&1"; fi

# install rust
ENV RUST_INSTALL_SCRIPT_VERSION=v0.238.0
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup
ENV PATH=${CARGO_HOME}/bin:${PATH}
RUN bash -c "$(curl -fsSL "https://raw.githubusercontent.com/microsoft/vscode-dev-containers/${RUST_INSTALL_SCRIPT_VERSION}/script-library/rust-debian.sh")" \
    -- "${CARGO_HOME}" "${RUSTUP_HOME}" \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# [Optional] Uncomment this line to install global node packages.
# RUN su vscode -c "source /usr/local/share/nvm/nvm.sh && npm install -g <your-package-here>" 2>&1

# setup wasmtime
ENV WASMTIME_VERSION=0.37.0
ENV WASMTIME_URL=https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux.tar.xz
RUN curl -L ${WASMTIME_URL} | tar -xJ --wildcards --no-anchored --strip-components 1 -C /usr/bin wasmtime

# setup wasm-sdk
ENV WASI_SDK_VERSION=16
ENV WASI_SDK_URL=https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION}/wasi-sdk-${WASI_SDK_VERSION}.0-linux.tar.gz
RUN curl -L ${WASI_SDK_URL} | tar -xz -C /opt
RUN ln -s /opt/wasi-sdk-${WASI_SDK_VERSION}.0/bin/clang /usr/bin/clang
RUN ln -s /opt/wasi-sdk-${WASI_SDK_VERSION}.0/bin/clang++ /usr/bin/clang++

# everything after this point is user specific
USER vscode

# setup python deps
RUN pip3 --disable-pip-version-check --no-cache-dir install mypy wasmtime \
    && rm -rf /tmp/pip-tmp

# configure rust
RUN rustup target add wasm32-unknown-unknown wasm32-wasi
RUN cargo install twiggy cargo-wasi cargo-expand
RUN cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli