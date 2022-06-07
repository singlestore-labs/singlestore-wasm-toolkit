# Rust User Generator
The following rust wasm example uses the [fakeit](https://crates.io/crates/fakeit) crate to generate fake user profiles that can be inserted into SingleStore for generating user profile data models quickly.

## Setup
### Compiling the code for Wasm
Now that we have our Rust code, to compile we need one of two ways of compiling the code. Using wasm32-wasi or wasm32-unknown-unknown you should check out the [different compilers here](https://docs.wasmtime.dev/wasm-rust.html)

Check out how to use the development container provided in this repository [SingleStore Build Wasm Rust](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/docs/Quickstart-2-Build-WASM-Rust.md)
### Setting up SingleStore Wasm Function
1. Copy your wasm and wit file to your singlestore ciab
```
docker cp fprofile.wasm singlestore-ciab:/fprofile.wasm
docker cp fprofile.wit singlestore-ciab:/fprofile.wit
```
1. Connect to your SingleStore CIAB cluster
```
docker exec -it singlestore-ciab singlestore -p'YOUR_PASSWORD' --local-infile=ON
```
1. Create the schema, function and table
```
CREATE DATABASE IF NOT EXISTS TechCo;
USE TechCo;
CREATE TABLE IF NOT EXISTS users (
  `uid` VARCHAR(36),
  `created` DATETIME,
  `first_name` VARCHAR(255),
  `last_name` VARCHAR(255),
  `email` VARCHAR(255),
  `password` VARCHAR(255),
  SHARD KEY (uid),
  SORT KEY (created)
);

CREATE OR REPLACE FUNCTION `gen-users` RETURNS TABLE AS WASM FROM INFILE '/usergenerator.wasm' WITH WIT FROM INFILE '/usergenerator.wit';

-- Test your new function!
SELECT * FROM `gen-users`(100);

-- The following is just a simple example of using Wasm in a Stored Procedure
DELIMITER //
CREATE OR REPLACE PROCEDURE insert_fake_users(c INT) RETURNS void AS
BEGIN
INSERT INTO users SELECT * FROM `gen-users`(c);
END;
//

CALL insert_fake_users(1000);
SELECT * FROM users;
```

# FAQ
## Can you give me a rough break down of all of this and their terms?
- Rust: The programming language (think C, C++, JS, Python, etc)
- Cargo: Package manager for rust (think apt, yum, zypper, pip, npm, etc)
- Wasm: WebAssembly (Compiled applications for the web)
- Wit: WebAssembly Interface Type (See more below)
- Wit Bindgen: Bindgen provides the coupling of the Wit file and the Wasm file. Enabling you to quickly generate "bindings" for your rust application/function

## What is a .wit file?
The wit file is a "WebAssembly Interface Type" definition file. You can think of it as simply "what is in your wasm function and what should i expect back". Check out the links below for more information on the topic.
- https://github.com/bytecodealliance/wit-bindgen/blob/main/WIT.md
- https://github.com/bytecodealliance/wit-bindgen

## Where can i find pre-compiled cargo packages that will help?
- https://crates.io
- https://lib.rs