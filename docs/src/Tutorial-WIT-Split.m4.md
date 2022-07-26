changequote(`{{', `}}')                                                          
# Creating a WIT Specification for the `split-str` Example
include({{Tutorial-WIT-Prologue.md}})                                          

In this example, we’ll work with strings and nested types. Let’s create a function that takes a string, splits it at every occurrence of a delimiting string, and then returns each sub-string along with its starting indices. The output will be sent back as a list of records (aka structures).

To start, let’s create a new work directory, and inside of it we’ll make a new file called `split.wit`. The WIT IDL we need is below, so we can go ahead and paste that in and save it.

```wit
record subphrase {
  str: string,
  idx: s32
}
split-str: func(phrase: string, delim: string) -> list<subphrase>
```

Now we’re ready to write some code.                                                
                                                                                 
* If you'd like to learn about implementing this example in C++, please look [here](Tutorial-Impl-CPP-Split.md).
* If you'd like to learn about implementing this example in Rust, please look [here](Tutorial-Impl-Rust-Split.md).

