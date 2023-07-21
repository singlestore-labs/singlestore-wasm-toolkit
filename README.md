# SingleStore Wasm Toolkit, Tutorial, And Examples

This repository provides utilities and documentation intended to help you streamline development of Wasm UDFs and TVFs for SingleStoreDB.  It consists of development containers, a tutorial, and a collection of example Wasm programs.

This README describes the technical details on how to get started using this toolkit.  Please also check out our [tutorial](https://singlestore-labs.github.io/singlestore-wasm-toolkit/html/Tutorial-Overview.html), which is great place to start once you are ready to write your first function, and will walk you through specific examples.

# Tools

To use the tools in this repo, you will need to have Docker installed on your system.  Most of these tools can be installed locally as well.

In this repo, you will find two development containers -- one standalone and one designed specifically for VS Code.  These containers attempt to strike a balance between providing a fairly comprehensive set of Wasm-oriented development tools, while still being mindful of image sizes.  Still, the container sizes currently range between 2-3 GB.

The following tools are available within each container:

## [clang](https://clang.llvm.org)
The Clang compiler and toolchain.  The exact compiler version may differ between containers; see below for specifics.

## [gcc/g++](https://gcc.gnu.org)
The GCC compiler and toolchain.  The exact compiler version may differ between containers; see below for specifics.

## [rust/cargo](https://www.rust-lang.org)
The Rust compiler with the *stable* toolchain.  It has been configured with compilation targets for the default platform, *wasm32-wasi*, and *wasm32-unknown-unknown*.  The exact compiler version may differ between containers; see below for specifics.

## [wasmtime](https://wasmtime.dev)
A popular Wasm compiler and runtime.

## [WASI SDK](https://github.com/WebAssembly/wasi-sdk)
Utilities to support the WASI toolchain.

## [wit-bindgen](https://github.com/WebAssembly/wasi-sdk)
A language binding generator for the WIT IDL.

## [writ](https://github.com/singlestore-labs/writ)
A utility to help test Wasm functions locally without the need to create a separate driver program.  Please see its dedicated [Git Repository](https://github.com/singlestore-labs/writ) for more information.

## [pushwasm](https://github.com/singlestore-labs/pushwasm)
A utility that allows you to easily import your locally-built Wasm function into SingleStoreDB as a UDF or TVF.  Please see its dedicated [Git Repository](https://github.com/singlestore-labs/pushwasm) for more information.

## Remote Debugging Tool (VS Code Only)
The VS Code container includes an experimental remote debugging tool that can be run as an external function from SingleStoreDB.  For instructions on how to use this utility, please see [this document](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/crates/debugger/README.md).

## Sudo
Finally, both containers give you access to password-less `sudo` so that you can further customize instances of them if you prefer.

# Suggested Workflow
This section suggests a possible workflow for developing a Wasm UDF or TVF for use in SingleStoreDB.
 
Before you start, you'll need to clone this repository.  For the purposes of example, we'll assume you are cloning to `$HOME/singlestore-wasm-toolkit`.
```sh
git clone https://github.com/singlestore-labs/singlestore-wasm-toolkit.git
```

## Getting Started With VS Code

*One-Time Setup*

1. Install [VS Code](https://code.visualstudio.com/download).

1. In VS Code, press *Ctrl-Shift-X*.  This will take you to the *Extensions Marketplace*.  Type `Remote - Containers` in the search field and click the *Install* button.

1. In VS Code, press *F1* and type `Remote Containers: Install Docker`.

1. By default, VS Code will mount just this repo in the container file system.  This is only really useful if you want to work with the examples.  If you'd like to work on other directories in this environment, then you will also need to do the following:
    1. Edit the `.devcontainer/devcontainer.json` file.
    1. Search for the `mounts` section and uncomment it.
    1. Change the value after `source=` to the local directory you want to mount.
    1. Change the value after `target=` to the path where you want the directory mounted in the container.  If you intend to develop in this directory, things will flow more smoothly if it is a child of the `/workspaces` directory. For example: `target=/workspaces/mycode`.
    1. You can add a new entry for each additional path you want mounted.

*Beginning Development*

1. Start VS Code.

1. Press `F1` and search for `Remote-Containers: Open Folder in Container`.

1. Navigate to the `$HOME/singlestore-wasm-toolkit` (or the place where you cloned this repo).  Click *Open*.

1. If this is your first time opening the container, it will now be built.  This can take quite a while, so please be patient -- it only needs to happen once.

1. If you intend to work on a project outside of this repo, then you will add this code to the current project.  Here's how:
    1. First make sure you've completed Step 4 in the *Preparation* section, above.
    1. Press *F1* and search for `Workspaces: Add Folder to Workspace`.
    1. In the drop-down list, if you followed the convention suggested in Step 4 of *Preparation*, you should see the name of the folder you mounted into the `/workspaces` directory.  If you chose a different location, then navigate to that folder instead.  When you are ready, make sure the proper folder is selected and click the *OK* button.
    1. The project will reload automatically.
    1. Your new folder can be found in the project navigator window.  If you want to save this workspace configuration for later, then press *F1*, search for `Workspaces: Save Workspace As`, and pick a name.

1. Edit the source code as necessary.

1. When you are ready to compile the Wasm module, press *Ctrl-\`* (that's the backtick character) to open a console window in the IDE.  You should see a prompt prefixed with `dev-shell`.  Be sure to change (`cd`) to the directory of the code you intend to compile.

## Getting Started With the Standalone Container

1. From this repo, run the script `scripts/dev-shell`.  It takes a single argument with the location of the local source directory where you intend to work.  For example:  `scripts/dev-shell /home/pete/mycoolproject`.

1. If this is your first time running this container, it will be downloaded from the GitHub Docker registry.  This may take a few minutes, depending on your internet connection.

1. You should now see a prompt prefixed with `dev-shell`.  The directory you supplied in the argument will be mounted to `~/src` in the container, and will be the current working directory when the prompt appears.

1. Edit your source code locally, using whatever tools your prefer.  You only need to use the dev-shell when you are ready to compile, test, or deploy.

## Iterative Development

For both container types, iterative development proceeds the same.  We'll use the `split` example included in this repo as a case study to describe a possible workflow.

1. To start:
    1. In the container, ensure that you are in the root directory of the code you intend to compile.  For our demonstration, we will be in `~/src/examples/rust/split`.
    1. Ensure that you have created a WIT file to describe the common interface of your Wasm function and UDF/TVF.  The WIT syntax is described [here](https://github.com/bytecodealliance/wit-bindgen/blob/main/WIT.md).  For the `split` example, we have placed this file at the root of the example's source tree.

1. Compile your program using the appropriate compiler.  Clang/GCC and Rust are both available in the container.  The process of compiling is described in more detail for C/C++ [here](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/docs/Tutorial-Developing-Wasm-UDF-CPP.md), and for Rust [here](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/docs/Tutorial-Developing-Wasm-UDF-Rust.md).
    <br>
    To compile `split` for Rust in optimized mode, we would use this command:
    <br>
    ```console
    cargo wasi build --lib --release
    ```

1. Now, test your program using the `writ` tool.  It should already be included in your `$PATH`.  Specific instructions on its usage are available in its [repository](https://github.com/singlestore-labs/writ).  As an example, we might use it to test one input of `split` like this:
    <br>
    ```console
    % writ --wit split.wit target/wasm32-wasi/release/split.wasm split_str 'hello_you_fine_folks' '_'
    [
      {
        "str": "hello",
        "idx": 0
      },
      {
        "str": "you",
        "idx": 6
      },
      {
        "str": "fine",
        "idx": 10
      },
      {
        "str": "folks",
        "idx": 15
      }
    ]
    
    ```

1. When you are satisfied with your program, you can import it into SingleStoreDB using the `pushwasm` tool, also included in your default `$PATH`.  Usage instructions can be found in its source [repository](https://github.com/singlestore-labs/pushwasm).  For example, we could use it like this to create a Wasm TVF from our `split` module:
    <br>
    ```console
    % pushwasm --tvf --prompt mysql://admin@svc-0e3c0e37-dml.singlestore.com:3306/testing --wit split.wit target/wasm32-wasi/release/split.wasm split_str
    Wasm TVF 'split_str' was created successfully.
    ```

1. The last step is verify that the Wasm UDF or TVF works in your database.  Using the SQL Editor in the SingleStore Customer Portal, you can check that the function has been created using the `SHOW FUNCTIONS` statement.  Then, you might try running using your UDF or TVF on a table.  Here's an example using the `split` TVF:
    <br>
    ```sql
    SELECT * from example_table t, split_str(t.name, ' ');
    ```

# Additional Information

To learn about the process of developing a Wasm UDF or TVF in more detail, please have a look at our [tutorial](https://singlestore-labs.github.io/singlestore-wasm-toolkit/html/Tutorial-Overview.html).

Information about setting up the remote debugging tool can be found [here](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/crates/debugger/README.md).

For instructions on using the `writ` tool, please go [here](https://github.com/singlestore-labs/writ).

For instructions on using the `pushwasm` tool, please go [here](https://github.com/singlestore-labs/pushwasm).

Information about the Rust examples can be found [here](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/examples/rust/README.md).

The SingleStoreDB Wasm UDF/TVF documentation is [here](https://docs.singlestore.com/managed-service/en/reference/code-engine---powered-by-wasm.html).

# Resources

* [Documentation](https://docs.singlestore.com)
* [Twitter](https://twitter.com/SingleStoreDevs)
* [SingleStore forums](https://www.singlestore.com/forum)

