# QuantRadar Architecture

## Data flow

```text
Exchange APIs/WebSockets
        ↓
Ingestion + normalization
        ↓
Raw immutable observations
        ↓
Data quality + liquidity
        ↓
Feature engine
   ├── market regime
   ├── cross-sectional ranking
   └── anomaly detection
        ↓
Independent screener families
        ↓
Evidence-backed signals
        ↓
Strategy candidates
        ↓
Walk-forward + fees/slippage + robustness
        ↓
Champion / challenger registry
        ↓
Portfolio + risk
        ↓
Paper trading
        ↓
Future execution boundary
```

## Rust / Python boundary

Rust is the deterministic production boundary: ingestion, normalization, features, screeners, regimes and backtesting. Python is the research layer: statistics, ML, optimization, walk-forward studies and visualization.

## Required market observation fields

`timestamp, exchange, symbol, base, quote, OHLCV, trade_count, bid, ask, bid_depth, ask_depth, source, ingested_at, quality_flags`.

## Signal contract

Every signal retains timestamp, symbol, family, direction, score, regime, rationale, feature evidence and strategy/config version. This prevents opaque scores from becoming the only evidence available to the portfolio layer.

## Design goals

- Dynamic asset discovery instead of hard-coded universes.
- Bounded asynchronous concurrency.
- No data leakage.
- Explicit trading costs.
- Regime-conditional research.
- Failed experiments retained for future learning.
- Liquidity as a hard constraint.
- Live execution isolated from research until promotion gates pass.
