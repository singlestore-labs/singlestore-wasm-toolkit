# Deploying `split-str` From Cloud Storage

You'll need the following information to do this:

- The path to the compiled Wasm module (we'll use `./split.wasm` below, but for the Rust example, you should use located in `target/wasm32-wasi/debug/split.wasm` instead)
- The path to the WIT file
- Your cloud storage credentials and path (we'll assume an S3 bucket called `wasm-modules`).

From your work directory, you'll need to upload your WIT (`./split.wit`) and compiled Wasm (`./split.wasm`) files to a bucket to the `wasm-modules` S3 bucket.

Now, connect to the database from your SQL client and run the following statement to pull the module from S3 into the `wasm_tutorial` database.  Note that unlike the [`power-of`](Tutorial-Deploy-Power-Cloud.md) example, we'll be deploying this `split-str` function as a Table-Valued Function (TVF).  This will require us to include the `RETURNS TABLE` clause.

```sql
CREATE FUNCTION split_str RETURNS TABLE AS WASM 
FROM S3 'wasm-modules/split.wasm'
    CREDENTIALS '{
        "aws_access_key_id": "ASIAZPIKLSJ3HM7FKAUB",
        "aws_secret_access_key": FwoGZXIvYXdzEL3fv [...]"
    }'
    CONFIG '{"region": "us-east-1"}'
WITH WIT FROM S3 'wasm-modules/split.wit'
    CREDENTIALS '{
        "aws_access_key_id": "ASIAZPIKLSJ3HM7FKAUB", 
        "aws_secret_access_key": FwoGZXIvYXdzEL3fv [...]"
    }' 
    CONFIG '{"region": "us-east-1"}';
```

If the TVF has been created successfully, you will see something like:

```console
Query OK, 1 row affected (0.029 sec)
```

Finally, we're ready to [run the TVF](Tutorial-Running-Split.md)!

