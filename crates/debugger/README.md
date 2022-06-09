# Wasm Remote Debugger Service

This crate implements an [external functions][extfns] compatible http service which hosts wasm functions at specific endpoints.

## Usage from VSCode

This repo is already setup to ensure a seamless debugging experience.

First, you need to make sure your wasm module is annotated with the debugger macro. We will use the following rust code as a starting point for adding debugger support:

**Cargo.toml**
```toml
[package]
name = "echo"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git" }

[lib]
crate-type = ["cdylib"]
```

**echo.wit**
```wit
echo: function(phrase: string) -> string
```

**lib.rs**
```rust
wit_bindgen_rust::export!("echo.wit");
struct Echo;
impl echo::Echo for Echo {
    fn echo(phrase: String) -> String {
        format!("{} {}", phrase, phrase)
    }
}
```

**In order to enable the debugger you need to make 2 changes.**

The first is to add a dependency on the debugger-macro crate. You need to add this line to the `[dependencies]` section of `Cargo.toml`:

```toml
debugger-macro = { path = "../../../crates/debugger-macro" }
```

Make sure the relative path is correct based on where your `Cargo.toml` is located.

The second is to use the debugger macro in your code. This macro needs to annotate the `impl echo::Echo for Echo` like so:

**lib.rs**
```rust
wit_bindgen_rust::export!("echo.wit");
struct Echo;

#[debugger_macro::export_debug_handler]
impl echo::Echo for Echo {
    fn echo(phrase: String) -> String {
        format!("{} {}", phrase, phrase)
    }
}
```

Once you have done this simply press `F5` while you have your lib.rs open. The debugger will automatically build your crate targetting wasm32-wasi, and then start hosting it at `localhost:3000/echo`.

**Note:** The path in the url needs to match the name of the function you want to call. For example, if the function in the code above was called "tell_joke" then the web service would host the function at `localhost:3000/tell_joke`. This does allow you to host multiple functions from the same Wasm module.

## Calling your remote function from the command line

For easy testing, you can now use curl (or any other http client) to test your Wasm code. Here is how I would use curl to test the function above:

```bash
$ curl -s -XPOST localhost:3000/echo -d '{"data":[[1,"hello"]]}' | jq -r '.'
{
  "data": [
    [
      1,
      "hello hello"
    ]
  ]
}
```

For documentation on the input/output format please see the [external function][extfns] documentation.

## Calling your remote function from SingleStore

Now that you have your Wasm code hosted behind an external functions compatible web service, you can easily call your code from SingleStore by defining an external udf or tvf. Full [documentation on doing this is here][extfns].

As an example, lets test the echo function we defined above. Note, I am running the VSCode dev container in this repository, and my dev container has ip address 172.17.0.3. I also have SingleStore running on my machine in another docker container. So, in order to call the external function from SingleStore I will need to tell SingleStore how to connect to 172.17.0.3:3000. I can do this like so:

```sql
MySQL [x]> create or replace external function echo (phrase text) returns text as remote service '172.17.0.3:3000/echo' format json;
Query OK, 1 row affected (0.014 sec)

MySQL [x]> select echo("hi");
+------------+
| echo("hi") |
+------------+
| hi hi      |
+------------+
1 row in set (0.049 sec)
```

## Using breakpoints

Now that you have your wasm code hosted in the remote debugger, you can do some pretty magical things with it. The first thing you can do is use breakpoints. To continue with the example from above, let's open `lib.rs` and put a breakpoint at the line containing `format!("{} {}", phrase, phrase)`.

Once the breakpoint is set make sure the debugger is running (press `F5` if it's not) and trigger your function from an HTTP client or SingleStore. As soon as you do, the breakpoint you set should be hit.

Note - currently debugger support for Wasm is a bit thin. You will be able to step through your code and get nice back traces on failure, however you won't be able to inspect local variables yet. Hopefully that will be resolved in the future as debugger support increases for Wasm modules.

## Logging

Since you can't inspect variables in the debugger, how can you see what is going on? For now, the best answer is good ol `print` style debugging. Using the example above, let's add some logs to our echo function:

**lib.rs**
```rust
#[debugger_macro::export_debug_handler]
impl echo::Echo for Echo {
    fn echo(phrase: String) -> String {
        dbg!("hello from wasm!", &phrase);
        format!("{} {}", phrase, phrase)
    }
}
```

When you run the debugger, you will see it's output in one of the VSCode terminals. Look for the phrase `tide::server Server listening on http://0.0.0.0:3000`. That's where your logs will go!

Now send another request to the debugger from an HTTP client or SingleStore, and you should see something like the following output:

```
tide::log::middleware <-- Request received
    method POST
    path /echo
[examples/rust/echo/src/lib.rs:7] "hello from wasm!" = "hello from wasm!"
[examples/rust/echo/src/lib.rs:7] &phrase = "hi"
tide::log::middleware --> Response sent
    method POST
    path /echo
    status 200 - OK
    duration 8.712664ms
```

Pretty cool right? Well, hopefully this gets you started! See the FAQ below if you run into any issues. Otherwise, enjoy!

# FAQ

## A panic occurred! VSCode opened up some weird assembly code

If VSCode opens a file that looks something like this:
```asm
; No Symbol Info
; Source location: /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/panic_abort/src/lib.rs:84
7F30BF4235F0: 0F 0B                      ud2    
7F30BF4235F2: 55                         pushq  %rbp
7F30BF4235F3: 48 89 E5                   movq   %rsp, %rbp
7F30BF4235F6: 8B F2                      movl   %edx, %esi
7F30BF4235F8: 48 8B 87 88 00 00 00       movq   0x88(%rdi), %rax
7F30BF4235FF: 48 0F B7 74 30 00          movzwq (%rax,%rsi), %rsi
7F30BF423605: 48 89 F0                   movq   %rsi, %rax
```

That means you have most likely panicked. Don't fret!

Check the call stack first. If you see your wasm function somewhere in the call stack click that to see where in your code the panic happened. Hopefully you can determine why and fix the issue.

If you **don't see your wasm function in the call stack** then you most likely hit the wrong endpoint on the debugger. Check that the url you are requesting ends with the precise name of the function defined in your code. For example, in the `examples/rust/power` example project, the function in the code is called `power_of` so the debugger endpoint needs to be `IP_ADDRESS:3000/power_of` for it to work.

## the trait bound `XXX: Serialize` is not satisfied

If you get a Rust compilation error that looks something like this, it means you are using custom types in your Wasm code.

```
error[E0277]: the trait bound `PolarityScores: Serialize` is not satisfied
    --> examples/rust/sentiment/src/lib.rs:18:1
     |
18   | #[debugger_macro::export_debug_handler]
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Serialize` is not implemented for `PolarityScores`
```

Since external functions uses JSON, you will need to provide an implementation of the `serde::Serialize` trait for each of your custom types. For example, here is the implementation of `Serialize` for the PolarityScores type in the `examples/rust/sentiment` crate:

```rust
impl Serialize for sentiment::PolarityScores {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("compound", &self.compound)?;
        map.serialize_entry("pos", &self.positive)?;
        map.serialize_entry("neg", &self.negative)?;
        map.serialize_entry("neu", &self.neutral)?;
        map.end()
    }
}
```

Once you add a valid Serialize implementation for your custom type(s) the compilation error should go away.

<!-- links -->

[extfns]: https://docs.singlestore.com/managed-service/en/reference/sql-reference/procedural-sql-reference/create--or-replace--external-function.html