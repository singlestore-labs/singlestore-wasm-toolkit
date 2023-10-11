# Simd Rust

A simple simd example using [core::arch::wasm32](https://doc.rust-lang.org/core/arch/wasm32/index.html#simd)

## Build

```sh
cargo build --target wasm32-wasi
```

## Create Function

```sql
CREATE FUNCTION `u64x2-dot` AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/simd.wasm' WITH WIT FROM INFILE 'simd.wit'
CREATE FUNCTION `u64x2-inner` RETURNS TABLE AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/simd.wasm' WITH WIT FROM INFILE 'simd.wit'
```

## Example Queries

```sql
SELECT * FROM `u64x2-dot`([1,2,3], [0,5,6]);
SELECT * FROM `u64x2-inner`([1,2,3], [3,4,5]);
```
