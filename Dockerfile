FROM debian as builder

RUN apt-get clean
RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Get Python
RUN apt-get install python3 -y
RUN apt-get install python3-pip -y
RUN pip install wasmtime 

COPY .devcontainer/library-scripts/*.sh /tmp/library-scripts/
RUN bash /tmp/library-scripts/wasmtime_wasi.sh \
    && bash /tmp/library-scripts/finalize.sh \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts
