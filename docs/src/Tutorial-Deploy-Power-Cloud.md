# Deploying `power-of` From Cloud Storage

You'll need the following information to do this:

- The path to the compiled Wasm module (we'll use `./power.wasm` below, but for the Rust example, you should use located in `target/wasm32-wasi/debug/power.wasm` instead)
- The path to the WIT file
- Your cloud storage credentials and path (we'll assume an S3 bucket called `wasm-modules`).

From your work directory, you'll need to upload your WIT (`./power.wit`) and compiled Wasm (`.wasm`) files to a bucket to the `wasm-modules` S3 bucket.

Now, connect to the database from your SQL client and run the following statement to pull the module from S3 into the `wasm_tutorial` database:

```sql
CREATE FUNCTION power_of AS WASM 
FROM S3 'wasm-modules/power.wasm'
    CREDENTIALS '{
        "aws_access_key_id": "ASIAZPIKLSJ3HM7FKAUB",
        "aws_secret_access_key": FwoGZXIvYXdzEL3fv [...]"
    }'
    CONFIG '{"region": "us-east-1"}'
WITH WIT FROM S3 'wasm-modules/power.wit'
    CREDENTIALS '{
        "aws_access_key_id": "ASIAZPIKLSJ3HM7FKAUB", 
        "aws_secret_access_key": FwoGZXIvYXdzEL3fv [...]"
    }' 
    CONFIG '{"region": "us-east-1"}';
```

If the UDF has been created successfully, you will see something like:

```console
Query OK, 1 row affected (0.029 sec)
```

Finally, we're ready to [run the UDF](Tutorial-Running-Power.md)!

