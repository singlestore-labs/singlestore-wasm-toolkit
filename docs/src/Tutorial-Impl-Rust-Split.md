# Developing a More Complex UDF in Rust

# Initialize Your Source Tree

Start by creating a new work directory and initializing it using `cargo init --vcs none --lib`.

We'll need the `split.wit` file we created [earlier](Tutorial-WIT-Split.md).  Copy it into this directory if it is not already there.

### Implementing and Compiling

Much of this will be similar to the techniques we used in the [`power-of` example](Tutorial-Impl-Rust-Power.md).

Edit the `Cargo.toml` file so it looks like this:

```toml
[package]
name = "split"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git", rev = "60e3c5b41e616fee239304d92128e117dd9be0a7" }

[lib]
crate-type = ["cdylib"]
```

And, for the implementation, edit the `src/lib.rs` file and replace its contents with this:

```rust,noplayground
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

Fortunately, with Rust, we have some nice language features that help make our code concise. Unlike in [`C/C++`](Tutorial-Impl-CPP-Split.md), we don’t need to explicitly free the parameters. Due to Rust’s move semantics, they will be freed implicitly when the function ends. Allocation for the output vector is also managed “under the hood” by Rust’s robust data structures and runtime. Finally, the declaration of the Subphrase struct happens via the `wit_bindgen_rust::export macro`, so we don’t need to do it.

Let’s compile the Wasm module now:

```bash
cargo wasi build --lib
```

The new Wasm file should be written to `target/wasm32-wasi/debug/split.wasm`.

Next, we'll do some [testing](Tutorial-Test-Split.md).

