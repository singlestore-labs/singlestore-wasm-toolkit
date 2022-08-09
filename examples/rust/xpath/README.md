# XPath functions

The XPath functions in this example allow you to query XML documents using 
[XPath](https://www.w3.org/TR/xpath-31/) expressions. There is one function for
use as a UVF and one for use as a TVF. The UDF can be used to pull out specific
elements / attributes of XML for use in projections or filtering. The TVF is
typically used to expand the number of rows in the data using an XPath expression
that returns multiple values for a single row of data. 

The function definitions are:

## UDF
```
eval_xpath(xml-string, xpath-string) -> string
```

## TVF
```
eval_xpaths(xml-string, xpath-string) -> list(string)
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
pushwasm --prompt mysql://user@host.com:3306/dbname --wit xpath.wit \
         target/wasm32-wasi/release/xpath.wasm eval_xpath
pushwasm --tvf --prompt mysql://user@host.com:3306/dbname --wit xpath.wit \
         target/wasm32-wasi/release/xpath.wasm eval_xpaths
```

## Using the functions

For the case of the examples below, the `bookxml` column has entries of the
following form:
```
<book category=\"web\">
  <title lang=\"en\">XQuery Kick Start</title>
  <author>James McGovern</author>
  <author>Per Bothner</author>
  <author>Kurt Cagle</author>
  <author>James Linn</author>
  <author>Vaidyanathan Nagarajan</author>
  <year>2003</year>
  <price>49.99</price>
</book>
```

An example using the UDF is as follows.
```
select eval_xpath(bookxml, '/book/title'), eval_xpath(bookxml, '/book/year') from booktable;
```

Since the entries can have multiple authors, it's possible to expand the number 
of rows to include a row for each author using the TVF.
```
select * from booktable t, eval_xpaths(bookxml, '/book/author');
```
