<!-- GENERATED FILE; DO NOT CHANGE! -->

# Testing the `split-str` Example
The `writ` utility provides a convenient way for us to test our Wasm function in isolation before we load it into the database.  It accepts JSON-formatted arguments on the command-line, casts them to the actual types defined in the Wasm function, and then passes them in.


Let's make sure our `split-str` program is working correctly by doing a few spot-tests.  The examples below assume that the `split.wasm` file exists in the current directory.  If you are using a Rust build, your Wasm file will be located at `target/wasm32-wasi/debug/split.wasm`.

```
$ writ --wit split.wit ./split.wasm split-str 'hello_you_fine_folks' '_'
[
  {
    "str": "hello",
    "idx": 0
  },
  {
    "str": "you",
    "idx": 6
  },
  {
    "str": "fine",
    "idx": 10
  },
  {
    "str": "folks",
    "idx": 15
  }
] 
```

Let's try a couple of edge cases as well.
```
$ writ --wit split.wit ./split.wasm split-str 'hello' '_'
[
  {
    "str": "hello",
    "idx": 0
  }
]

$ writ --wit split.wit split.wasm split-str 'hello--there-' '-'
[
  {
    "str": "hello",
    "idx": 0
  },
  {
    "str": "",
    "idx": 6
  },
  {
    "str": "there",
    "idx": 7
  },
  {
    "str": "",
    "idx": 13
  }
]
```

Looks good!

Now, we're ready to [deploy](Tutorial-Deploy-Split.md).

