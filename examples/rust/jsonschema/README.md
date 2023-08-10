# ss_jsonschema: JSON Schema support for SingleStoreDB

**ss_jsonschema** adds [JSON schema](https://json-schema.org/) validation support for json data types. Since this is a Wasm extension, this will take `string` as input and parse as Json in the validation process. See examples below.

## Contents

### `match_schema (schema: string, instance: string) -> bool`
Returns true if the given instance matches the schema provided

### `is_valid_schema (schema: string) -> bool`
Returns true if the given schema is valid

## Building
```
# Install the WASI cargo extension.
cargo install cargo-wasi

# Compile the Wasm module.
cargo wasi build --release
```
## Deployment to SingleStoreDB

To install these functions using the MySQL client, use the following commands.  This command assumes you have built the Wasm module and your current directory is the root of this Git repo.  Replace `$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.

```bash
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE FUNCTION match_schema AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/extension.wasm" WITH WIT FROM LOCAL INFILE "extension.wit";
CREATE FUNCTION is_valid_schema AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/extension.wasm" WITH WIT FROM LOCAL INFILE "extension.wit";
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines.  As above, be sure to substitute the environment variables with values of your own.
```bash
pushwasm udf --force --prompt --name match_schema \
    --wasm target/wasm32-wasi/release/extension.wasm \
    --wit extension.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name is_valid_schema \
    --wasm target/wasm32-wasi/release/extension.wasm \
    --wit extension.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

## Clean
```
cargo clean
```

## Examples

### Simple example
```sql
SELECT match_schema('{"type": "number"}' , '123');
```

Output:
```
+--------------------------------------------+
| match_schema('{"type": "number"}' , '123') |
+--------------------------------------------+
|                                          1 |
+--------------------------------------------+
```

```sql
SELECT match_schema('{"type": "number"}' , '"hi"');
```

Output:
```
+---------------------------------------------+
| match_schema('{"type": "number"}' , '"hi"') |
+---------------------------------------------+
|                                           0 |
+---------------------------------------------+
```


```sql
SELECT is_valid_schema('{"type" : "integer"}');
```

Output:
```
+-----------------------------------------+
| is_valid_schema('{"type" : "integer"}') |
+-----------------------------------------+
|                                       1 |
+-----------------------------------------+
```

```sql
SELECT is_valid_schema('{"type" : "int"}');
```

Output:
```
+-------------------------------------+
| is_valid_schema('{"type" : "int"}') |
+-------------------------------------+
|                                   0 |
+-------------------------------------+
```

### More complex example
```sql
set @schema='{
         "type": "object",
         "properties": {
          "foo": {
           "type": "string"
          }
         },
         "required": ["foo"],
         "additionalProperties": false
      }';

SELECT match_schema(@schema, '{"foo" : "bar"}');
```

Output:
```
+------------------------------------------+
| match_schema(@schema, '{"foo" : "bar"}') |
+------------------------------------------+
|                                        1 |
+------------------------------------------+
```

```sql
SELECT match_schema(@schema, '{"notfoo" : "bar"}');
```

Output:
```
+---------------------------------------------+
| match_schema(@schema, '{"notfoo" : "bar"}') |
+---------------------------------------------+
|                                           0 |
+---------------------------------------------+
```

```sql
SELECT match_schema(@schema, '{"foo" : "bar", "morefoo": "morebar"}');
```

Output:
```
+----------------------------------------------------------------+
| match_schema(@schema, '{"foo" : "bar", "morefoo": "morebar"}') |
+----------------------------------------------------------------+
|                                                              0 |
+----------------------------------------------------------------+
```

## Acknowledgement
Rust [boon](https://crates.io/crates/boon) library and [JSON Schema](https://json-schema.org/)
