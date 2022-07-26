# Deploying `power-of` Using `pushwasm`

The `pushwasm` tool will upload our WIT file and compiled Wasm module into the database.  To use it, we'll need the following information.  Since this depends highly on your specific environment, we'll just make some generic assumptions about their values.

- The hostname of the SingleStoreDB server (we'll call this `myserver`)
- The destination database name (we'll call this `wasm_tutorial`)
- The user ID and password of the database user (we'll call this user `admin`)
- The path to the compiled Wasm module (we'll use `./power.wasm` below, but for the Rust example, you should use located in `target/wasm32-wasi/debug/power.wasm` instead)
- The path to the WIT file

Now, run the following command from within your work directory:

```bash
pushwasm --prompt --wit ./power.wit mysql://admin@myserver/wasm_tutorial ./power.wasm power_of
```

The `--prompt` option will cause a prompt to appear, where you can enter your database user's password.

When the deployment has completed, you should see the following:

```console
Wasm UDF 'power_of' was created successfully.
```

Finally, we're ready to [run the UDF](Tutorial-Running-Power.md)!

