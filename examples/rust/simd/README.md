# Simd Rust

A simple simd example using [core::arch::wasm32](https://doc.rust-lang.org/core/arch/wasm32/index.html#simd)

## Build

```sh
cargo build --target wasm32-unknown-unknown
```

## Create Function

```sql
CREATE FUNCTION mul AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/simd.wasm' WITH WIT FROM INFILE 'simd.wit'
CREATE FUNCTION dot AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/simd.wasm' WITH WIT FROM INFILE 'simd.wit'
CREATE FUNCTION `inner` RETURNS TABLE AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/simd.wasm' WITH WIT FROM INFILE 'simd.wit'
```

## Example Queries

```sql
SELECT mul(3,4);
SELECT dot([1,2,3], [0,5,6]);
SELECT * FROM `inner`([1,2,3], [3,4,5]);
```
