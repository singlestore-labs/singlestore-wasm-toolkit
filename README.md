# SingleStore WASM Examples

For more info on getting started using WASM UDFs with SingleStore try our [tutorial](https://github.com/singlestore-labs/wasm-udf-tutorial/)!

## Development Containers

This repo provides two development containers.  Choose the one that best matches your preferred workflow.

### VS Code

TODO
VS Code instructions here

### Standalone

Alternatively, you can use a standalone development container that does not integrate with VS Code.  To do this, run the following script:

```
scripts/dev-shell PATH
```

The path specifies the root of the source directory for the project you want to build.  This directory will be mapped into the container under `/home/$USER/src'.

*Note*: The first time you run this script, it will build a Docker container with your UID/GID information.  This is necessary so that you can access your local filesystem within the container.  This build may take several minutes, depending on your bandwidth and CPU, and the resulting container will be about 3GB.

## Usage

1. [Sign up](https://www.singlestore.com/try-free/) for a free SingleStore license. This allows you
   to run up to 4 nodes up to 32 gigs each for free. Grab your license key from
   [SingleStore portal](https://portal.singlestore.com/?utm_medium=osm&utm_source=github) and set it as an environment
   variable.

   ```bash
   export SINGLESTORE_LICENSE="singlestore license"
   ```
1. Try the Run WASM Quickstart [here](./docs/Quickstart-1-Run-WASM.md)
1. Try the Build WASM Quickstart for your preferred language
   * [C/C++](./docs/Quickstart-2-Build-WASM-CPP.md)
   * [Rust](./docs/Quickstart-2-Build-WASM-Rust.md)

## Resources

* [Documentation](https://docs.singlestore.com)
* [Twitter](https://twitter.com/SingleStoreDevs)
* [SingleStore forums](https://www.singlestore.com/forum)
