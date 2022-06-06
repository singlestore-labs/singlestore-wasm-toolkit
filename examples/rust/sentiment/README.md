# Sentiment Rust

A simple sentiment analysis function using [VADER Sentiment Analysis](https://crates.io/crates/vader_sentiment)

## Build

```sh
cargo build --target wasm32-unknown-unknown
```

## Create Function

```sql
CREATE FUNCTION sentiment AS WASM FROM INFILE 'target/wasm32-unknown-unknown/debug/sentiment_rust.wasm' WITH WIT FROM INFILE 'sentiment.wit'
```

## Example Queries

```sql
SELECT sentiment('I love this!').positive;
SELECT sentiment('I hate this!').negative;
```