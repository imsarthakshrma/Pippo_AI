---
description: Run the full backtesting suite to validate strategy performance.
---

1. Build the project in release mode for performance
// turbo
cargo build --release

2. Run the backtesting simulation
// turbo
cargo run --release --bin pippo -- backtest --days 30
