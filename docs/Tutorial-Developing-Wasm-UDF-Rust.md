# SingleStore Wasm UDF Tutorial
<!-- The original tutorial -->

## Overview

This tutorial will teach you how to write some simple WebAssembly (Wasm) UDFs in in Rust, load them into the database, and evaluate them in queries.

## Setup

For each example in this tutorial, it will be useful to create yourself a separate work directory in which to put your code and compiled artifacts. This will help keep your files for each example organized.

In addition to a SQL client and SingleStore database, you’ll need a few other things to get your environment set up properly. For this, you have a couple of options.

### VS Code DevContainer Setup

The easiest way to get started is to download the [SingleStore WASM Toolkit](https://github.com/singlestore-labs/singlestore-wasm-toolkit) repository.
This repo provides a VS Code DevContainer with all of the tools you'll need to work with WASM UDFs and complete this tutorial. It also includes fully-written versions of the tutorial code.

You can install VS Code [here](https://code.visualstudio.com/download) then follow these steps:

1. Run git clone git@github.com:singlestore-labs/singlestore-wasm-toolkit.git

1. Install the Remote - Containers VS Code Extension if you do not already have it.

1. In VS Code, type F1 and search for “Open Folder in Container”.

1. Navigate to the directory where you cloned the project in step 1, and click Open. The container will build, which may take a few minutes.

When the container build completes, you are ready to go. Execute your commands inside the VS Code terminal window so that the required command-line tools are available.

> **Note**
>  Make sure to run your SQL client on your local system, outside of the container.

> **Note**
> Create your folders inside the repository structure so they are available within the container.
<!-- These use a special GitHub-flavored .md feature -->

### Manual Setup

If you’d prefer to set up your development environment manually, you’ll need to do the following:

- Download and install the Rust toolchain and WASM targets:

  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  - `source $HOME/.cargo/env` (or restart your shell)

  - `rustup target add wasm32-wasi`

- Download and install the wit-bindgen program:

  - `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli`

## Developing a Simple Example

### Creating the WIT Specification

Before we do any coding, let’s first define our interface. WIT is an Interface Definition Language (IDL) used for describing WASM modules in `*.wit` files. We'll create a .wit specification for our new function. In a new work directory, open a new file called power.wit in your text editor.

Let’s say we want to develop a program that simply computes $x^y$. The interface for this is simple; here is the WIT IDL for it:

```wit
power-of: function(base: s32, exp: s32) -> s32
```

This function will take two signed 32-bit integers as arguments (the base and the exponent) and return a single signed 32-bit integer.

Copy and paste the above code into power.wit and save it. Now we’re ready to write some code.

### Using Bindgen
To implement this interface in Rust, we're going to use wit-bindgen. First, from within a new work directory, run `cargo init --vcs none --lib`. This will set up a skeletal Rust source tree.

Next, edit the file called `Cargo.toml` so that it looks like the following:

```toml
[package]
name = "power"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git", rev = "60e3c5b41e616fee239304d92128e117dd9be0a7" }

[lib]
crate-type = ["cdylib"]
```

We’ll also need the power.wit file we used above. Either recreate it or copy it into your work directory.

Now we’re almost ready to roll. Edit the file `src/lib.rs` and replace its content with this:

```rust
wit_bindgen_rust::export!("power.wit");
struct Power;

impl power::Power for Power {
    fn power_of(base: i32, exp: i32) -> i32 {
        let mut res = 1;
        for _i in 0..exp {
            res *= base;
        }
        res
    }
}
```

The syntax at the top of the code is boiler-plate. The export macro generates code that declares a trait named after our WIT file (and some other things). So, our main job is here is to implement this trait. If you are curious what the macro actually generates, you can run cargo expand and it will show you the fully expanded source code.

The WIT IDL is heavily inspired by the Rust language syntax, so it was pretty easy to derive the Rust function signature we needed from the IDL:

- Replaced hyphens with underscores

- Changed s32 types to i32

Now we can compile the program into a wasm module using this command:

```bash
cargo wasi build --lib
```

We can now load the module into the database using the same procedure we discussed above. The Wasm module is written to `target/wasm32-wasi/debug/power.wasm`, so we need to make sure the Wasm *infile* path is pointing there instead of the work tree’s root.

```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `power-of` AS WASM FROM LOCAL INFILE '/workdir/target/wasm32-wasi/debug/power.wasm' WITH WIT FROM LOCAL INFILE '/workdir/power.wit';
```

And, once again, running the following SQL gives us back the expected result of 256.

```sql
SELECT `power-of`(2, 8);
```

Before moving on to the next example, drop your `wasm_tutorial` database.
```sql
DROP DATABASE wasm_tutorial
```

## A More Complex Example

### Creating the WIT Specification

In this example, we’ll work with strings and nested types. Let’s create a function that takes a string, splits it at every occurrence of a delimiting string, and then returns each sub-string along with its starting indices. The output will be sent back as a list of records (aka structures).

To start, let’s create a new work directory, and inside of it we’ll make a new file called `split.wit`. The WIT IDL we need is below, so we can go ahead and paste that in and save it.

```wit
record subphrase {
  str: string,
  idx: s32
}
split-str: function(phrase: string, delim: string) -> list<subphrase>
```

### Implementing

For our last trick, we’ll split strings in Wasm using a Rust-based implementation.
Much of this will be similar to the techniques we used in the simple example.

Start by creating a new work directory and initializing it using `cargo init --vcs none --lib`.

Now, edit the `Cargo.toml` file so it looks like this:

```toml
[package]
name = "split"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git, rev = "60e3c5b41e616fee239304d92128e117dd9be0a7" }

[lib]
crate-type = ["cdylib"]
```

Next, let’s copy or recreate the split.wit file in our work directory.

And, for the implementation, edit the `src/lib.rs` file and replace its contents with this:

```rust
wit_bindgen_rust::export!("split.wit");
struct Split;
use crate::split::Subphrase;

impl split::Split for Split {

    fn split_str(phrase: String, delim: String) -> Vec<Subphrase> {
        phrase
            .split(&delim)
            .scan(0, |idx, s| {
                let current = Subphrase {
                    str: s.to_string(),
                    idx: *idx as i32
                };
                *idx += (s.len() + delim.len()) as i32;
                Some(current)
            })
            .collect()
    }
} 
```

Fortunately, with Rust, we have some nice language features that help make our code concise. Notice how, unlike in C++, we don’t need to explicitly free the parameters. Due to Rust’s move semantics, they will be freed implicitly when the function ends. Allocation for the output vector is also managed “under the hood” by Rust’s robust data structures and runtime. Finally, the declaration of the Subphrase struct happens via the `wit_bindgen_rust::export macro`, so we don’t need to do it.

Let’s compile the Wasm module now:

```bash
cargo wasi build --lib
```

And, we’ll finish up by loading the module into the database as TVF, just as we did with the simple example. Note again that our Wasm module is down in the target/debug directory.


```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `split-str` RETURNS TABLE AS WASM FROM LOCAL INFILE '/workdir/target/wasm32-wasi/debug/split.wasm' WITH WIT FROM LOCAL INFILE '/workdir/split.wit';
```

Then:

```sql
SELECT * FROM `split-str`('wasm_rocks_the_house', '_');
```

## Wrap-Up

Well, this concludes our little tutorial. Using both the C/C++ and Rust programming languages we were able to turn simple and not-quite-so-simple use cases into WebAssembly programs. We also learned how to load them into the SingleStore database in form of UDFs and TVFs, and then run them.

Hopefully, this helps you kickstart your own Wasm UDFs. Thanks for tuning in!



