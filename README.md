# Penguin

Live Binance order book ingestion with self-healing sync, plus a minimal strategy/risk/execution loop.

## What it does
- Streams depth updates over WebSocket
- Applies deltas with sequence checks and auto resync
- Emits mid-price only when it changes
- Runs a simple strategy with risk checks and paper execution

## Run
```zsh
cargo run
```

## Strategy / Risk defaults
- Strategy: `SimpleMidStrategy` with a 0.5 mid threshold
- Risk: max position 1.0, max order size 0.1
- Execution: paper fills at mid

Update these in `src/main.rs`.
