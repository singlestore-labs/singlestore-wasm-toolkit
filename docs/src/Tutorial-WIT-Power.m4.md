changequote(`{{', `}}')
# Creating a WIT Specification for the `power-of` Example
include({{Tutorial-WIT-Prologue.md}})

This example will compute x^y (that is, "x to the power of y").

In a new work directory, create a new file called `power.wit` in your text editor, and add this content:

```wit
power-of: func(b: s32, exp: s32) -> s32
```

This function will take two signed 32-bit integers as arguments (the base `b` and the exponent `exp`) and return a single, signed 32-bit integer.

Go ahead and save the file.  

Now we’re ready to write some code.

* If you'd like to learn about implementing this example in C, please look [here](Tutorial-Impl-CPP-Power.md).
* If you'd like to learn about implementing this example in Rust, please look [here](Tutorial-Impl-Rust-Power.md).

