changequote(`{{', `}}')
# Developing the `power-of` example in Rust

include({{Tutorial-Impl-Rust-Prologue.md}})
## Copy the WIT File

We’ll need the `power.wit` file we created [earlier](Tutorial-WIT-Power.md). Copy it into this directory if it is not already there.

## Implementing and Compiling

Now, edit the file called `Cargo.toml` so that it looks like the following:

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

Now we’re almost ready to roll. Edit the file `src/lib.rs` and replace its content with this:

```rust,noplayground
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

The new Wasm file should be written to `target/wasm32-wasi/debug/power.wasm`.

Next, we'll do some [testing](Tutorial-Test-Power.md).

