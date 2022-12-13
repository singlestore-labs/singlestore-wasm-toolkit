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
TODO: change this for regex
writ --wit sentimentable.wit target/wasm32-wasi/debug/sentimentable.wasm sentimentable 'Wasm is an exciting new technology that we love.'

[
  {
    "compound": 0.812604508328942,
    "positive": 0.513888888888889,
    "negative": 0.0,
    "neutral": 0.4861111111111111
  }
]
```

