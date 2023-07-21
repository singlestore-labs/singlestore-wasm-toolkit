# Templating Functions

The templating functions in this example allow you to apply an
XML, HTML, JSON, or Yaml document / object to a
[Tera](https://tera.netlify.app) template. In the case of XML 
and HTML, the documents can be queried using 
[XPath](https://www.w3.org/TR/xpath-31/) expressions. For JSON
and Yaml, [JSONPath](https://datatracker.ietf.org/wg/jsonpath/about/)
expressions can be used.  The output data format can be anything
supported by Tera templates, which can generate pretty much any
text-based format.

What this means is that you can transform XML, HTML, JSON, or Yaml
strings into any other format including new forms of XML, HTML, JSON,
or Yaml. For example, if you had the following XML structure:
```
<book category="web">
    <title lang="en">XQuery Kick Start</title>
    <author>James McGovern</author>
    <author>Per Bothner</author>
    <author>Kurt Cagle</author>
    <author>James Linn</author>
    <author>Vaidyanathan Nagarajan</author>
    <year>2003</year>
    <price>49.99</price>
</book>
```

You could convert it to a JSON structure like this:
```
{
  "book": {
    "title": "XQuery Kick Start",
    "authors": [
      "James McGovern",
      "Per Bothner",
      "Kurt Cagle",
      "James Linn",
      "Vaidyanathan Nagarajan"
    ],
    "published": {
      "year": 2003
    },
    "listing": {
      "price": 49.99
    }
  }
}
```

Using the `render_xml` function included in this package and the
following template:
```
{
  "book": {
    "title": {{ q(path="/book/title") | get(key="text") | json_encode | safe }},
    "authors": [{% for item in q(path="/book/author") %}
                    {{ item | get(key="text") | json_encode | safe }}{% if not loop.last %}, {% endif %}
                {% endfor %}],
    "published": { "year": {{ q(path="/book/year") | get(key="text") | int }} },
    "listing": { "price": {{ q(path="/book/price") | get(key="text") | float }} }
  }
}
```

## The `q` function

In addition to being able to traverse the objects in the parsed document using
the Tera syntax, a `q` function has also been added. It has the following
signature:
```
q(path="...") -> string
```

The `path` parameter when using XML or HTML input is an XPath query. For
JSON and Yaml input, the path is a JSONPath query. This allows you to use 
more powerful queries to extract pieces of your input document than what
Tera can do by default.

## XML / HTML object structure

While JSON and Yaml are fairly straight-forward data structures that both map to
JSON structures entirely using maps and arrays, XML and HTML are a bit more
complex. There is no standard map or array type in XML / HTML. In order to make
it possible to traverse XML / HTML objects in a Tera template, those documents
are converted to a JSON-like object using the following mappings:
```
<book category="web">
    <title lang="en">XQuery Kick Start</title>
    <author>James McGovern</author>
    <author>Per Bothner</author>
    <author>Kurt Cagle</author>
    <author>James Linn</author>
    <author>Vaidyanathan Nagarajan</author>
    <year>2003</year>
    <price>49.99</price>
</book>
```

The JSON object for the above XML looks like:
```
{
  "_": {
    "name": "book",
    "attributes": {"category": "web"},
    "children": [
      {
        "name": "title",
        "attributes": {"lang": "en"},
        "text": "XQuery Kick Start",
        "children": []
      },
      {
        "name": "author",
        "attributes": {},
        "text": "James McGovern",
        "children": []
      },
      ...
      {
        "name": "year",
        "attributes": {},
        "text": "2003",
        "children": []
      },
      {
        "name": "price",
        "attributes": {},
        "text": "49.99",
        "children": []
      },
    ]
  }
}
```

As you can see traversing an XML document using Tera's syntax or
even JSONPath is rather complicated. Using the `q(path="...")` function
to get nodes is much simpler. Even in an XML document, the `q` function
returns a JSON object that can be traversed from that point. For example,
to get the price of a book using the `q` function could be done as follows:
```
q(path="/book/price")
```

This will return an object of the following form:
```
{
  "name": "price",
  "attributes": {},
  "text": "49.99",
  "children": []
}
```

You can then extract the `text` attribute using Tera's `get` function:
```
q(path="/book/price") | get(key="text")
```

The above expression will return "49.99".

## UDFs

The functions included in this package are as follows:

```
render_json(json-string, template-string) -> string
render_xml(xml-string, template-string) -> string
render_yaml(yaml-string, template-string) -> string
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
pushwasm udf --conn mysql://user:@127.0.0.1:3306/dbname --wit templates.wit \
             --wasm target/wasm32-wasi/release/templates.wasm --name render_json
pushwasm udf --conn mysql://user:@127.0.0.1:3306/dbname --wit templates.wit \
             --wasm target/wasm32-wasi/release/templates.wasm --name render_xml
pushwasm udf --conn mysql://user:@127.0.0.1:3306/dbname --wit templates.wit \
             --wasm target/wasm32-wasi/release/templates.wasm --name render_yaml
```

## Using the functions

The `test.py` file contains a Python program that demonstrates the use
of each of the functions.
