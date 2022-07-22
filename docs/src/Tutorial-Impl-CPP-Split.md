# Implementing the `split-str` example in C++

## Generating Bindings
In this example, we’ll use C++ so that we can leverage the STL’s higher-level data structures and keep our implementation focused on the big picture as much as possible.

As before, we’ll start by generating the C language bindings. Run the *wit-bindgen* command:

```bash
wit-bindgen c --export split.wit
```

There should now be a `split.c` and `split.h` file in your work directory. Since we’ll be using C++, rename `split.c` to `split.cpp`:

## Implementing and Compiling
Since we’ll be using C++, rename `split.c` to `split.cpp`:

```bash
mv split.c split.cpp
```

Now let’s take a look at `split.h`. As we might expect, there are a few more definitions in here than in the simpler `power-of` example. The wit-bindgen program has generated a struct definition for us to use when passing our strings, as well as ones for the subphrase record and its enclosing list. At the bottom is prototype for the function we are going to implement:

```cpp
void split_split_str(split_string_t *phrase, split_string_t *delim, split_list_subphrase_t *ret0);
```

This, too, looks a bit different than in the `power-of` example. For one thing, the function doesn’t return a value. Since this function will be returning a list, for which we’ll need to dynamically allocate memory, the result is passed as an argument pointer instead.

Now let’s open up the `split.cpp` file. Once again, we are going to add our implementation here. At the bottom of the file, we can find the generated wrapper function, called `__wasm_export_split_split_str`. This wrapper delegates to the function we will be implementing, and we also can see that it is doing work required for lifting and lowering the data types on either side of the function call.

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

There is much more work going on here than in the [`power-of`](Tutorial-Impl-CPP-Power.md) example; a fair amount of it deals with memory management.

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

Next, we'll do some [testing](Tutorial-Test-Split.md).
