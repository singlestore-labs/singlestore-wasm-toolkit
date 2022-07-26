changequote(`{{', `}}')
# Testing the `power-of` Example
include({{Tutorial-Test-Prologue.md}})

Let's make sure our `power-of` program is working correctly by doing a few spot-tests.  The examples below assume that the `power.wasm` file exists in the current directory.  If you are using a Rust build, your Wasm file will be located at `target/wasm32-wasi/debug/power.wasm`.

```
$ writ --wit power.wit ./power.wasm power-of 2 3
8

$ writ --wit power.wit ./power.wasm power-of 2 0
1

$ writ --wit power.wit ./power.wasm power-of 0 0
1

$ writ --wit power.wit ./power.wasm power-of 0 2
0

$ writ --wit power.wit ./power.wasm power-of 2 -3
1
```

Except for the last attempt, the function seems to work correctly.  To keep this example simple, we'll just assume that negative exponents won't be provided.

Now, we're ready to [deploy](Tutorial-Deploy-Power.md).

