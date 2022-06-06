# SingleStore Wasm UDF Tutorial
<!-- The original tutorial -->

## Overview

This tutorial will teach you how to write some simple WebAssembly (Wasm) UDFs in C and in Rust, and then load and run them in the database.

Pre-written code can be found under the [examples](examples) directory.

## Setup￼

For each example in this tutorial, it will be useful to create yourself a separate work directory in which to put your code and compiled artifacts.  This will help keep your files for each example organized.

In addition to a SQL client and SingleStore database, you’ll need a few other things to get your environment set up properly.  For this, you have a couple of options.

### VS Code DevContainer Setup￼

The easiest way to get started is to download the GitHub repository for this tutorial.  This repo provides a VS Code DevContainer that will give you all of the tools you'll need to work through the examples here.  It also includes fully-written versions of the examples themselves in case you just want to cut to the chase.

You can install VS Code here:
https://code.visualstudio.com/download

Once you have VS Code, follow these steps:

1. Run git clone git@github.com:singlestore-labs/wasm-udf-tutorial.git

1. Install the Remote - Containers VS Code Extension if you do not already have it.

1. In VS Code, type F1 and search for “Open Folder in Container”.

1. Navigate to the directory where you cloned the project in step 1, and click Open.

1. VS Code will now build the container.  This may take a few minutes.

When the container build completes, you are ready to go.  Be sure to execute your commands inside the VS Code terminal window so that the required command-line tools are available.

*Important Note*:  In each example of this tutorial, there will be a section where we load our compiled Wasm module into the SingleStore database.  For this, you will need a SQL client – which you will need to run on your local system, outside of the container.  To make the compiled artifacts you build in the container available to the SQL client, be sure to create your work directories under the Git project.  When the tutorial references a file in /workdir directory, replace this with the actual local (not the container-relative) path.

### Manual Setup￼

If you’d prefer to set up your development environment manually, you’ll need to do the following:

- Download the WASI SDK and decompress it somewhere.  For the purposes of this tutorial, it might be convenient to create a shell alias to the version of clang in this tarball.  For example, if you uncompressed the tarball under /opt, then you can run the following:

  - `alias clang=/opt/wasi-sdk-14.0/bin/clang`

  - `alias clang++=/opt/wasi-sdk-14.0/bin/clang++`

- Download and install the Rust toolchain:

  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- Download and install the wit-bindgen program:

  - `cargo install --git https://github.com/bytecodealliance/wit-bindgen wit-bindgen-cli`

## Developing a Simple Example￼

### Creating the WIT Specification￼

Before we do any coding, let’s first define our interface. WIT is an Interface Definition Language (IDL) used for describing WASM modules in `*.wit` files. We'll create a .wit specification for our new function. In a new work directory, open a new file called power.wit in your text editor.

Let’s say we want to develop a program that simply computes x^y (that is, x to the power of y).  The interface for this is simple; here is the WIT IDL for it:
￼
```
power-of: function(base: s32, exp: s32) -> s32
```

This function will take two signed 32-bit integers as arguments (the base and the exponent) and return a single signed 32-bit integer.

Copy and paste the above code into power.wit and save it.  Now we’re ready to write some code.

### In C/C++￼

We’ll start by generating the C language bindings for our functions.  We can do this using the wit-bindgen program (demo).  In your work directory, run the following command:

```bash
wit-bindgen c --export power.wit
```

This will generate two files in your work directory:  `power.c` and `power.h`.

If you look at the contents of the `power.h` file, you’ll see a single prototype:

```cpp
int32_t power_power_of(int32_t base, int32_t exp);
```

The name looks odd because wit-bindgen concatenates the name of the WIT file with the name of the function.  That’s ok; as we’ll see in a moment, the name that is actually exported will make more sense.

Aside from the function name, the signature is as expected:  take two 32-bit integers, and return one.

Next, open the `power.c` file in your editor.  We’ll ignore the `canonical_abi_realloc` and `canonical_abi_free` functions for now and skip to the bottom where we will find a function called `__wasm_export_power_power_of`.  This is wrapper code that handles passing values through the Wasm Canonical ABI (a trivial operation in this case).  Looking at the body of this function, we can see that it calls the `power_power` function that was declared in the header file.  We’ll need to provide the implementation for this.

Before we continue, though, notice the following line just above the this function’s definition:

```cpp
__attribute__((export_name("power-of")))
```

This line forces the name of this wrapper function be exported from the compiled module as power-of (the hyphen is the preferred word separator for function names in Wasm).  Fortunately, consumers only need to invoke it by this name.

Now, let’s implement the logic we need.  To the bottom of the `power.c` file, add the following code (we can start by copying in the prototype from the `power.h` file):

```cpp￼
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
￼
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

All that’s left to do is load it into the database.  Fire up your favorite SingleStore instance and connect a SQL client to it.  You’ll need to make sure the “local infile” feature is enabled; to do this in the mysql CLI, pass the `--local-infile=ON` argument.

In your SQL client, issue the following commands.  For this tutorial, we’ll just create a scratch database called `wasm_tutorial`.  In the last line, be sure to replace `/workdir` with the path to your work directory (if you are using a DevContainer, please see the note in the Set Up section above).  

```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `power-of` AS WASM FROM INFILE '/workdir/power.wasm' WITH WIT FROM INFILE '/workdir/power.wit';
```

If the UDF has been created successfully, you will see something like:

```
Query OK, 1 row affected (0.029 sec)
```

Now our UDF is ready to run!  To do this, just run the following command:

```sql
SELECT `power-of`(2, 8);
```

... which should return a single-column row with the value 256, of course.

### In Rust￼
Now, let’s learn how to do the same thing using a Rust program.  Rust requires a little more infrastructure to set up first, but has some convenient integration points with WebAssembly.

First, from within a new work directory, run `cargo init --vcs none --lib`.  This will set up a skeletal Rust source tree.

Next, edit the file called `Cargo.toml` so that it looks like the following:
￼
```
[package]
name = "power"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git" }

[lib]
crate-type = ["cdylib"]
```

We’ll also need the power.wit file we used above.  Either recreate it or copy it into your work directory.

Now we’re almost ready to roll.  Edit the file `src/lib.rs` and replace its content with this:
￼
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

The syntax at the top of the code is boiler-plate.  The export macro generates code that declares a trait named after our WIT file (and some other things).  So, our main job is here is to implement this trait.  If you are curious what the macro actually generates, you can run cargo expand and it will show you the fully expanded source code.

The WIT IDL is heavily inspired by the Rust language syntax, so it was pretty easy to derive the Rust function signature we needed from the IDL:

- Replaced hyphens with underscores

- Changed s32 types to i32

Now we can compile the program into a wasm module using this command:

```bash
cargo build --target wasm32-unknown-unknown
```

We can now load the module into the database using the same procedure we discussed above.  The Wasm module is written to `target/wasm32-unknown-unknown/debug/power.wasm`, so we need to make sure the Wasm *infile* path is pointing there instead of the work tree’s root.
￼
```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `power-of` AS WASM FROM INFILE '/workdir/target/wasm32-unknown-unknown/debug/power.wasm' WITH WIT FROM INFILE '/workdir/power.wit';
```

And, once again, running the following SQL gives us back the expected result of 256.

```sql
SELECT `power-of`(2, 8);
```

## A More Complex Example￼

### Creating the WIT Specification￼

In this example, we’ll work with strings and nested types.  Let’s create a function that takes a string, splits it at the first occurrence of a delimiting string, and then returns the two sub-strings along with their starting indices.   The output will be sent back as a list of records (aka structures).

To start, let’s create a new work directory, and inside of it we’ll make a new file called `split.wit`.  The WIT IDL we need is below, so we can go ahead and paste that in and save it.

```￼
record subphrase {
  str: string,
  idx: s32
}
split-str: function(phrase: string, delim: string) -> list<subphrase>
```

### In C++￼

With this example, we’ll use C++ so that we can leverage the STL’s higher-level data structures and keep our implementation focused on the big picture as much as possible.

As before, we’ll start by generating the C language bindings.  Run the *wit-bindgen* command:

```bash
wit-bindgen c --export split.wit
```

There should now be a `split.c` and `split.h` file in your work directory.  Since we’ll be using C++, rename `split.c` to `split.cpp`:

```bash
mv split.c split.cpp
```

Now let’s take a look at `split.h`.  As we might expect, there are a few more definitions in here than before.  The wit-bindgen program has generated a struct definition for us to use when passing our strings, as well as ones for the subphrase record and its enclosing list.  At the bottom is prototype for the function we are going to implement:

```cpp
void split_split_str(split_string_t *phrase, split_string_t *delim, split_list_subphrase_t *ret0);
```

This, too, looks a bit different than before.  For one thing, the function doesn’t return a value.  Since this function will be returning a list, for which we’ll need to dynamically allocate memory, the result is passed as an argument pointer instead.

Now let’s open up the `split.cpp` file.  Once again, we are going to add our implementation here.  At the bottom of the file, we can find the generated wrapper function, called `__wasm_export_split_split_str`.  This wrapper delegates to our function, and we also can see that it  is doing work required for lifting and lowering the data types on either side of the function call.

We’ll now add our code.  Let’s first update the top of `split.cpp` as follows:
￼
```cpp
#include <memory>
#include <stdlib.h>
#include <string.h>
#include <string>
#include <vector>
#include <split.h>
```

Since C++ has a more strict compiler than C, we’ll also need to make a small change to the generated code in this file at line 38.  Go ahead and change the following line:

￼
```cpp
ret->ptr = canonical_abi_realloc(NULL, 0, 1, ret->len);
```

... to this:

```￼
ret->ptr = reinterpret_cast<char *>(canonical_abi_realloc(NULL, 0, 1, ret->len));
```

And finally, to the bottom of the file, we’ll add this chunk of code:

```￼
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

    // If success, assign the result.  Else, clean up and return an empty list.
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

The Wasm Canonical ABI requires that any dynamic memory passed from the host to the guest or vice-versa transfers the ownership to the receiver.  This explains the last two lines of our `split_split_str` function, where we free the pointers to the strings we have been passed as arguments.  This memory will have been allocated by the host using our guest module’s exported `canonical_abi_realloc` function, which takes it from the Wasm instance’s linear memory at runtime.  Conversely, you may also notice that we pass dynamically allocated memory out of the function; the host is expected to free it by calling our module's `canonical_abi_free` routine, which will return it to the linear memory.

Now let’s save the file and build the module.  We will use a similar approach as we did in the simple example above, but with a couple of tweaks.  First, we’ll use clang++, since this is C++ code.  And second, we’ll need to include the option `-fno-exceptions` because Wasm doesn’t yet support exception handling (there is a proposal, however).

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

Hooray!  It’s time to load it into the database and try it out.  Another difference between the simple example and this one is that, while we previously returned a scalar value from our UDF, we are now returning a list of records.  This aspect makes the function suitable for use as a TVF.  And, we only need to make a small change to the SQL syntax to do it (by adding the `RETURNS TABLE` clause).

Start your database client up, again making sure you enable the “local infile” feature (`--local-infile=ON` for mysql), and issue the following statements.  Don’t forget to replace `/workdir`.

```sql￼
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `split-str` RETURNS TABLE AS WASM FROM INFILE '/workdir/split.wasm' WITH WIT FROM INFILE '/workdir/split.wit';
```

Now we can run our Wasm function as a TVF like this:
￼
```sql
SELECT * FROM `split-str`('wasm_rocks_the_house', '_');
```
... which will produce the following output:
￼
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

### In Rust￼

For our last trick, we’ll split strings in Wasm using a Rust-based implementation.  Much of this will be similar to the techniques we used in the simple example.

Start by creating a new work directory and initializing it using `cargo init --vcs none --lib`.

Now, edit the `Cargo.toml` file so it looks like this:
￼
```
[package]
name = "split"
version = "0.1.0"
edition = "2018"

[dependencies]
wit-bindgen-rust = { git = "https://github.com/bytecodealliance/wit-bindgen.git" }

[lib]
crate-type = ["cdylib"]
```

Next, let’s copy or recreate the split.wit file in our work directory.

And, for the implementation, edit the `src/lib.rs` file and replace its contents with this:

```rust￼
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

Fortunately, with Rust, we have some nice language features that help make our code concise.  Notice how, unlike in C++, we don’t need to explicitly free the parameters.  Due to Rust’s move semantics, they will be freed implicitly when the function ends.  Allocation for the output vector is also managed “under the hood” by Rust’s robust data structures and runtime.  Finally, the declaration of the Subphrase struct happens via the `wit_bindgen_rust::export macro`, so we don’t need to do it.

Let’s compile the Wasm module now:

```bash
cargo build --target wasm32-unknown-unknown
```

And, we’ll finish up by loading the module into the database as TVF, just as we did with the simple example.  Note again that our Wasm module is down in the target/debug directory.

￼
```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
CREATE FUNCTION `split-str` RETURNS TABLE AS WASM FROM INFILE '/workdir/target/wasm32-unknown-unknown/debug/split.wasm' WITH WIT FROM INFILE '/workdir/split.wit';
```

Then:
￼
```sql
SELECT * FROM `split-str`('wasm_rocks_the_house', '_');
```

## Wrap-Up￼

Well, this concludes our little tutorial.  Using both the C/C++ and Rust programming languages we were able to turn simple and not-quite-so-simple use cases into WebAssembly programs.  We also learned how to load them into the SingleStore database in form of UDFs and TVFs, and then run them.

Hopefully, this helps you kickstart your own Wasm UDFs.  Thanks for tuning in!



