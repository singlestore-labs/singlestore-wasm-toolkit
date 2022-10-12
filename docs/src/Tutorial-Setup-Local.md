# Local Setup

If you do not wish to use a development container and/or you’d prefer to set up your development environment on your local system, you’ll need to do the following:

- Download the [WASI SDK](https://github.com/WebAssembly/wasi-sdk/releases) and decompress it somewhere.  Ensure that your `$PATH` variable is prefixed with this location when you are running the build commands suggested in this tutorial.  For example, assuming you installed the WASI SDK in `/opt/wasi-sdk`, at the command prompt, you can type (assuming your are using `bash`/`zsh`):

  - `export PATH=/opt/wasi-sdk/bin:$PATH`

- Download and install the Rust toolchain:

  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  - `source $HOME/.cargo/env`

- Download and install the wit-bindgen program:

  - `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli`

- Download and install the cargo-wasi plugin:

  - `cargo install cargo-wasi`

- *(Optional)* If you would like to use the testing tool [`writ`](https://github.com/singlestore-labs/writ) and the deployment tool [`pushwasm`](https://github.com/singlestore-labs/pushwasm), please follow the installation instructions in their respective repositories and ensure they are in your `$PATH`.

For a more detailed walkthrough on setting up for local development, also consider checking out David Lee's blog entry [here](https://www.singlestore.com/blog/locally-create-wsm-udfs-rust-singlestoredb).

Next, let's pick an [example](Tutorial-Examples.md) to work through.

