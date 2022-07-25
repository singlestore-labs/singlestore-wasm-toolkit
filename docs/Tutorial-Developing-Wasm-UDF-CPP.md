# SingleStore Wasm UDF Tutorial
<!-- The original tutorial -->

## Overview

This tutorial will teach you how to write some simple WebAssembly (Wasm) UDFs in in C/C++, load them into the database, and evaluate them in queries.

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

- Download the [WASI SDK](https://github.com/WebAssembly/wasi-sdk/releases) and decompress it somewhere. For the purposes of this tutorial, it might be convenient to create a shell alias to the version of clang in this tarball. For example, if you uncompressed the tarball under /opt, then you can run the following:

  - `alias clang=/opt/wasi-sdk-14.0/bin/clang`

  - `alias clang++=/opt/wasi-sdk-14.0/bin/clang++`

- Download and install the Rust toolchain:

  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  - `source $HOME/.cargo/env` (or restart your shell)

- Download and install the wit-bindgen program:

  - `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli`

## Developing a Simple Example

### Creating the WIT Specification

Before we do any coding, let’s first define our interface. WIT is an Interface Definition Language (IDL) used for describing WASM modules in `*.wit` files. We'll create a .wit specification for our new function. In a new work directory, open a new file called power.wit in your text editor.

Let’s say we want to develop a program that simply computes x^y (that is, x to the power of y). The interface for this is simple; here is the WIT IDL for it:

```wit
power-of: func(base: s32, exp: s32) -> s32
```

This function will take two signed 32-bit integers as arguments (the base and the exponent) and return a single signed 32-bit integer.

Copy and paste the above code into power.wit and save it. Now we’re ready to write some code.

### Using Bindgen

We’ll start by generating the C language bindings for our functions. We can do this using the wit-bindgen program (demo). In your work directory, run the following command:

```bash
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

### Implementing and Running
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
    -s                           \
    -I.                          \
    -o power.wasm                \
    power.c
```

You should now see a `power.wasm` file in your directory.

All that’s left to do is load it into the database. Fire up your favorite SingleStore instance and connect a SQL client to it. You’ll need to make sure the “local infile” feature is enabled; to do this in the mysql CLI, pass the `--local-infile=ON` argument.

In your SQL client, issue the following commands. For this tutorial, we’ll just create a scratch database called `wasm_tutorial`.
You may need to adjust the paths to your files if your SQL client is not in the same directory as them or does not support relative paths.

```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `power_of` AS WASM FROM LOCAL INFILE 'power.wasm' WITH WIT FROM LOCAL INFILE 'power.wit';
```

If the UDF has been created successfully, you will see something like:

```
Query OK, 1 row affected (0.029 sec)
```

Now our UDF is ready to run!  To do this, just run the following command:

```sql
SELECT `power_of`(2, 8);
```

... which should return a single-column row with the value 256, of course.

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
split-str: func(phrase: string, delim: string) -> list<subphrase>
```

### Implementing the Specification

With this example, we’ll use C++ so that we can leverage the STL’s higher-level data structures and keep our implementation focused on the big picture as much as possible.

As before, we’ll start by generating the C language bindings. Run the *wit-bindgen* command:

```bash
wit-bindgen c --export split.wit
```

There should now be a `split.c` and `split.h` file in your work directory. Since we’ll be using C++, rename `split.c` to `split.cpp`:

```bash
mv split.c split.cpp
```

Now let’s take a look at `split.h`. As we might expect, there are a few more definitions in here than before. The wit-bindgen program has generated a struct definition for us to use when passing our strings, as well as ones for the subphrase record and its enclosing list. At the bottom is prototype for the function we are going to implement:

```cpp
void split_split_str(split_string_t *phrase, split_string_t *delim, split_list_subphrase_t *ret0);
```

This, too, looks a bit different than before. For one thing, the function doesn’t return a value. Since this function will be returning a list, for which we’ll need to dynamically allocate memory, the result is passed as an argument pointer instead.

Now let’s open up the `split.cpp` file. Once again, we are going to add our implementation here. At the bottom of the file, we can find the generated wrapper function, called `__wasm_export_split_split_str`. This wrapper delegates to our function, and we also can see that it  is doing work required for lifting and lowering the data types on either side of the function call.

We’ll now add our code. Let’s first update the top of `split.cpp` as follows:

```cpp
#include <memory>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <vector>
#include <split.h>
```

Since C++ has a more strict compiler than C, we’ll also need to make a small change to the generated code in this file at line 38. Go ahead and change the following line:


```cpp
ret->ptr = canonical_abi_realloc(NULL, 0, 1, ret->len);
```

... to this:

```
ret->ptr = reinterpret_cast<char *>(canonical_abi_realloc(NULL, 0, 1, ret->len));
```

And finally, to the bottom of the file, we’ll add this chunk of code:

```cpp
void split_split_str(split_string_t *phrase, split_string_t *delim, split_list_subphrase_t *ret0)
{
    // Clear the result.
    memset(ret0, 0, sizeof(split_list_subphrase_t));

    // Parse the tokens.
    std::string phr(phrase->ptr, phrase->len);
    std::string dlm(delim->ptr, delim->len);
    std::string tok;
    std::vector<std::pair<std::string, size_t>> subs;
    size_t start = 0, end = 0;
    if (delim->len)
    {
        while ((end = phr.find(dlm, start)) != std::string::npos)
        {
            tok = phr.substr(start, end - start);
            subs.push_back(std::pair<std::string, size_t>(tok, start));
            start = end + dlm.length();
        }
    }
    subs.push_back(std::pair<std::string, size_t>(phr.substr(start), start));

    // Populate the result.
    bool err = false;
    auto res = (split_subphrase_t *) malloc(phr.size() * sizeof(split_subphrase_t));
    for (int i = 0; !err && i < subs.size(); ++i)
    {
        auto& sub = subs[i].first;
        res[i].idx = static_cast<int32_t>(subs[i].second);
        res[i].str.len = sub.length();
        res[i].str.ptr = strdup(sub.c_str());
        if (!res[i].str.ptr)
            err = true;
    }

    // If success, assign the result. Else, clean up and return an empty list.
    if (!err)
    {
        // Success; assign the result.
        ret0->ptr = res;
        ret0->len = subs.size();
    }
    else
    {
        if (res)
        {
            for (int i = 0; i < subs.size(); ++i)
                if (res[i].str.ptr)
                    free(res[i].str.ptr);
            free(res);
        }
    }

    // Per the Canonical ABI contract, free the input pointers.
    free(phrase->ptr);
    free(delim->ptr);
}
```

Obviously, there is much more work going on here than in our first example; a fair amount of it deals with memory management.

The Wasm Canonical ABI requires that any dynamic memory passed from the host to the guest or vice-versa transfers the ownership to the receiver. This explains the last two lines of our `split_split_str` function, where we free the pointers to the strings we have been passed as arguments. This memory will have been allocated by the host using our guest module’s exported `canonical_abi_realloc` function, which takes it from the Wasm instance’s linear memory at runtime. Conversely, you may also notice that we pass dynamically allocated memory out of the function; the host is expected to free it by calling our module's `canonical_abi_free` routine, which will return it to the linear memory.

Now let’s save the file and build the module. We will use a similar approach as we did in the simple example above, but with a couple of tweaks. First, we’ll use clang++, since this is C++ code. And second, we’ll need to include the option `-fno-exceptions` because Wasm doesn’t yet support exception handling (there is a proposal, however).

```bash
clang++                          \
    -fno-exceptions              \
    --target=wasm32-unknown-wasi \
    -mexec-model=reactor         \
    -s                           \
    -I.                          \
    -o split.wasm                \
    split.cpp
```

As expected, there should now be a `split.wasm` file in your work directory.

### Running the New UDF
Hooray!  It’s time to load it into the database and try it out. Another difference between the simple example and this one is that, while we previously returned a scalar value from our UDF, we are now returning a list of records. This aspect makes the function suitable for use as a TVF. And, we only need to make a small change to the SQL syntax to do it (by adding the `RETURNS TABLE` clause).

Start your database client up, again making sure you enable the “local infile” feature (`--local-infile=ON` for mysql), and issue the following statements. 
You may need to adjust the paths to your files if your SQL client is not in the same directory as them or does not support relative paths.

```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `split_str` RETURNS TABLE AS WASM FROM LOCAL INFILE 'split.wasm' WITH WIT FROM LOCAL INFILE 'split.wit';
```

Now we can run our Wasm function as a TVF like this:

```sql
SELECT * FROM `split_str`('wasm_rocks_the_house', '_');
```
... which will produce the following output:

```
+-------+-----+
| str   | idx |
+-------+-----+
| wasm  |   0 |
| rocks |   5 |
| the   |  11 |
| house |  15 |
+-------+-----+
94 rows in set (0.001 sec)
```

Awesome!

## Wrap-Up

Well, this concludes our little tutorial. Using both the C/C++ we were able to turn simple and not-quite-so-simple use cases into WebAssembly programs. We also learned how to load them into the SingleStore database in form of UDFs and TVFs, and then run them.

Hopefully, this helps you kickstart your own Wasm UDFs. Thanks for tuning in!
