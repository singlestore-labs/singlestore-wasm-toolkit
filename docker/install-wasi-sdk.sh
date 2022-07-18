#!/bin/sh

UPDATE_RC="${UPDATE_RC:=true}"
WASI_SDK_VERSION="${WASI_SDK_VERSION:=16}"
WASI_SDK_URL="${WASI_SDK_URL:=https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION}/wasi-sdk-${WASI_SDK_VERSION}.0-linux.tar.gz}"
WASI_SDK_HOME="${WASI_SDK_HOME:=/opt/wasi-sdk/}"

mkdir -p ${WASI_SDK_HOME}
curl -L ${WASI_SDK_URL} | tar -xz -C ${WASI_SDK_HOME} --strip-components 1

updaterc() {
    if [ "${UPDATE_RC}" = "true" ]; then
        echo "Updating /etc/bash.bashrc and /etc/zsh/zshrc..."
        if [[ "$(cat /etc/bash.bashrc)" != *"$1"* ]]; then
            echo -e "$1" >> /etc/bash.bashrc
        fi
        if [ -f "/etc/zsh/zshrc" ] && [[ "$(cat /etc/zsh/zshrc)" != *"$1"* ]]; then
            echo -e "$1" >> /etc/zsh/zshrc
        fi
    fi
}

# Add WASI-SDK bin to beginning of PATH in bashrc/zshrc files (unless disabled)
# so that WASI  clang and clang++ take precedence over the system ones.
updaterc "$(cat << EOF
if [[ "\${PATH}" != *"\${WASI_SDK_HOME}/bin"* ]]; then export PATH="\${WASI_SDK_HOME}/bin:\${PATH}"; fi
EOF
)"
