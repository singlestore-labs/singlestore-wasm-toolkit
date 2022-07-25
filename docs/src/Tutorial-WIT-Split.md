<!-- GENERATED FILE; DO NOT CHANGE! -->
                                                          
# Creating a WIT Specification for the `split-str` Example
Before we do any coding, let’s first define our interface. WIT is an Interface Definition Language (IDL) used to describe Wasm modules.  It is provided in files with the `.wit` extension.
                                          

In this example, we’ll work with strings and nested types. Let’s create a function that takes a string, splits it at every occurrence of a delimiting string, and then returns each sub-string along with its starting indices. The output will be sent back as a list of records (aka structures).

To start, let’s create a new work directory, and inside of it we’ll make a new file called `split.wit`. The WIT IDL we need is below, so we can go ahead and paste that in and save it.

```wit
record subphrase {
  str: string,
  idx: s32
}
split-str: func(phrase: string, delim: string) -> list<subphrase>
```

Now we're ready to [write some code](Tutorial-Impl-CPP-Split.md).

