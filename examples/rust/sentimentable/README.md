# Sentiment Rust

A simple sentiment analysis function using [VADER Sentiment Analysis](https://crates.io/crates/vader_sentiment)

## Build

```sh
cargo wasi build --lib
```

## Testing with Writ

```sh
writ --wit sentimentable.wit target/wasm32-wasi/debug/sentimentable.wasm sentimentable 'Wasm is an exciting new technology that we love.'

[
  {
    "compound": 0.812604508328942,
    "positive": 0.513888888888889,
    "negative": 0.0,
    "neutral": 0.4861111111111111
  }
]
```

## Example Queries

```sql
SELECT sentimentable('I love this!').positive;
SELECT sentimentable('I hate this!').negative;
```
