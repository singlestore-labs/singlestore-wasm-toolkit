# Run WASM UDFs

This quickstart will get WASM UDFs running in a database as fast as possible!

TODO

## Load in an Existing `.wasm` File

TODO

```sql
create link gcs_wasm as gcs credentials '{}' description 'wasm and wit examples';

create function sentimentable returns table
as wasm from link gcs_wasm 'wasm-modules/sentimentable.wasm'
with wit from link gcs_wasm 'wasm-modules/sentimentable.wit';
```

## Execute Queries

```sql
--TODO
```

## Further Reading

Checkout the Build WASM Quickstart for C++ or Rust!