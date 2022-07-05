# Sentiment Rust

A simple sentiment analysis function using [VADER Sentiment Analysis](https://crates.io/crates/vader_sentiment)

## Build

```sh
cargo wasi build --lib
```

## Example Queries

```sql
SELECT sentiment('I love this!').positive;
SELECT sentiment('I hate this!').negative;
```
