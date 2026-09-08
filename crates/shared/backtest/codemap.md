# crates/backtest/

## Responsibility
Cost-aware backtesting (backtest engine, walk-forward, robustness, promotion gates).

## Design
Realistic execution with cost modeling; expanding WFO framework.

## Flow
Strategy candidates → backtest with costs → robustness → promotion.

## Integration
Used by portfolio, paper trading; depends on core, features, regime, screeners.
