# Implementing the `power-of` Example in C

## Generating Bindings
For the C language, we’ll need to explicitly generate language bindings for our functions. We can do this using the [`wit-bindgen`](https://bytecodealliance.github.io/wit-bindgen/) program. In your work directory, run the following command:

```
wit-bindgen c --export power.wit
```

This will generate two files in your work directory:  `power.c` and `power.h`.

If you look at the contents of the `power.h` file, you’ll see a single prototype:

```cpp
int32_t power_power_of(int32_t base, int32_t exp);
```

The name looks odd because wit-bindgen concatenates the name of the WIT file with the name of the function. That’s ok; as we’ll see in a moment, the name that is actually exported will make more sense.

Aside from the function name, the signature is as expected:  take two 32-bit integers, and return one.

Next, open the `power.c` file in your editor. We’ll ignore the `canonical_abi_realloc` and `canonical_abi_free` functions for now and skip to the bottom where we will find a function called `__wasm_export_power_power_of`. This is wrapper code that handles passing values through the Wasm Canonical ABI (a trivial operation in this case). Looking at the body of this function, we can see that it calls the `power_power` function that was declared in the header file. We’ll need to provide the implementation for this.

Before we continue, though, notice the following line just above the this function’s definition:

```cpp
__attribute__((export_name("power-of")))
```

This line forces the name of this wrapper function be exported from the compiled module as power-of (the hyphen is the preferred word separator for function names in Wasm). Fortunately, consumers only need to invoke it by this name.

## Implementing and Compiling
Now, let’s implement the logic we need. To the bottom of the `power.c` file, add the following code (we can start by copying in the prototype from the `power.h` file):

```cpp
int32_t power_power_of(int32_t base, int32_t exp)
{
    int32_t res = 1;
    for (int32_t i = 0; i < exp; ++i)
    {
        res *= base;
    }
    return res;
}
```

Now let’s save the file and get back to our command line.

We can compile this program into a Wasm module by using the following command.

```bash
clang                            \
    --target=wasm32-unknown-wasi \
    -mexec-model=reactor         \
    -I.                          \
    -o power.wasm                \
    power.c
```

You should now see a `power.wasm` file in your directory.

Next, we'll do some [testing](Tutorial-Test-Power.md).

