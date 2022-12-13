# Regex Capture

This example shows how to create a Wasm UDF that will capture the first regex
match encountered.  A hash map is used to cache compiled regexes over multiple
calls so that repeated regexes need not be recompiled every time.

## Build

```sh
cargo wasi build --lib
```

## Testing with Writ

```sh
writ --wit s2regex.wit target/wasm32-wasi/debug/s2regex.wasm capture "aabaaabbb" "(b{2,})"

"bbb"
```

