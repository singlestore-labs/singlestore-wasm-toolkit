# JSONPath functions

The JSONPath functions in this example allow you to query JSON objects using 
[JSONPath](https://www.ietf.org/id/draft-ietf-jsonpath-base-05.html) expressions.
There is one function for use as a UVF and one for use as a TVF. The UDF can be used
to pull out specific elements JSON for use in projections or filtering. The TVF is
typically used to expand the number of rows in the data using an JSONPath expression
that returns multiple values for a single row of data. 

The function definitions are:

## UDF
```
eval_jsonpath(json-string, jsonpath-string) -> json-string
```

## TVF
```
eval_jsonpaths(json-string, jsonpath-string) -> list(json-string)
```

## Compiling

To compile the functions in the example, use the following command.

```
cargo wasi build --lib --release
```


## Load functions into the database

Once you have compiled the functions, they can be loaded into the database
using the `pushwasm` command.

```
pushwasm --prompt mysql://user@host.com:3306/dbname --wit jsonpath.wit \
         target/wasm32-wasi/release/xpath.wasm eval_jsonpath
pushwasm --tvf --prompt mysql://user@host.com:3306/dbname --wit jsonpath.wit \
         target/wasm32-wasi/release/xpath.wasm eval_jsonpaths
```

## Using the functions

For the case of the examples below, the `bookjson` column has entries of the
following form:
```
{
    "category": "web",
    "language": "en",
    "title": "XQuery Kick Start",
    "author": [
        "James McGovern",
        "Per Bothner",
        "Kurt Cagle",
        "James Linn",
        "Vaidyanathon Nagarajan"    
    ],
    "year": 2003,
    "price": 49.99
}
```

An example using the UDF is as follows.
```
select eval_jsonpath(bookjson, '$.title'), eval_jsonpath(bookjson, '$.year') from booktable;
```

Since the entries can have multiple authors, it's possible to expand the number 
of rows to include a row for each author using the TVF.
```
select * from booktable t, eval_jsonpaths(bookjson, '$.author[*]');
```
