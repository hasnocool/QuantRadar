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

## Crate modules (current workspace state)

New/ext crates added to extend functionality (grouped by theme):

**Data / persistence / quality** — `data-model`, `data-quality`, `persistent-data`, `ingestion`, `storage`, `archives`, `websocket`, `replay`, `event_bus`, `events`
**Exchanges / multi-source** — `exchange-kraken` (REST + WebSocket v2), `exchange-binance`, `exchange-coinbase`, `multi_exchange`
**Features / engine / store** — `features`, `feature-engine`, `feature-store`
**Regime / ranking / stats** — `regime`, `regime-detector`, `ranking`, `pca`, `research`
**Screening / ensemble / signals** — `screeners`, `ensemble`, `signal-ensemble`, `signals`, `sentiment`
**Strategy / DSL / backtest / engine** — `strategies`, `strategy_dsl`, `backtest`, `backtest_engine`
**Execution / paper / live / risk / portfolio** — `execution`, `paper-trading`, `paper`, `live-exec`, `portfolio`, `portfolio_risk`, `risk`
**Operations / registry / monitoring / pipeline** — `pipeline`, `registry`, `model_registry`, `monitoring`, `scheduler`, `test-scale`, `validation`, `experiment`
**Domain / types / lineages / universe** — `core`, `domain-model`, `types`, `lineage`, `universe`, `universe_history`, `derivatives`, `expected-return` / `expected_return`

These crates extend the architecture from ingestion through persistent storage, multi-exchange support, feature stores, regime detection, signal ensemble, backtest engines, portfolio risk, replay, rate limiting (`rate-limiter`), and isolated live-execution (`live-exec`) boundaries.
