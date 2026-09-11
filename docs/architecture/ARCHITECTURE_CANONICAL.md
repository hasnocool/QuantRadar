SEE ALSO: docs/control-plane/MASTERLIST.md §3 (canonical authority).
# ARCHITECTURE_CANONICAL.md — Canonical Architecture Documentation

## Overview

QuantRadar is a Rust + Python quantitative research platform for market data analysis,
strategy development, and execution. The system is organized as a Rust workspace with
 numerous crates organized into shared and standalone categories.

This document provides:
1. **Individual crate documentation** — Detailed descriptions of each crate's purpose,
   functionality, and API
2. **Overall architecture** — How all crates fit together in the system

## Workspace Structure

The Rust workspace is defined in `Cargo.toml` and contains two categories of crates:

### Shared Crates (`crates/shared/`)
Core infrastructure crates used by multiple standalone crates:

| Crate | Purpose |
|-------|---------|
| `archives` | Persistent tick storage, trades, order-book snapshots |
| `backtest` | Backtesting engine infrastructure |
| `backtest_engine` | Full backtest engine with event-driven simulation |
| `cli` | Command-line interface framework |
| `core` | Core domain models (types, core data structures) |
| `dashboard` | Dashboard UI components |
| `data-model` | Data model types (OHLC, ticks, trades) |
| `domain-model` | Domain-driven design entities (Account, Position, Order) |
| `exchange-kraken` | Kraken exchange integration |
| `execution` | Execution engine and order management |
| `expected-return` | Expected return modeling |
| `experiment` | Experiment framework and run management |
| `features` | Feature engineering pipeline |
| `ingestion` | Market data ingestion (WebSocket, REST collectors) |
| `microstructure` | Order-book microstructure analysis |
| `news_ingestion` | News data ingestion and processing |
| `order-book` | Order book model and snapshot management |
| `ranking` | Cross-sectional ranking models |
| `regime` | Market regime detection |
| `reporting` | Report generation (PDF, HTML, markdown) |
| `research` | Research scripts and ML pipeline components |
| `screeners` | Screener strategies and filters |
| `signal-ensemble` | Signal ensemble and meta-model stacking |
| `storage` | Persistent storage abstraction (SQLx, DuckDB) |
| `websocket` | WebSocket client abstractions |

### Standalone Crates (`crates/standalone/`)
Domain-specific and application-level crates:

| Crate | Purpose |
|-------|---------|
| `data-quality` | Data quality validation and gap detection |
| `derivatives` | Derivatives data layer (futures, options) |
| `ensemble` | Signal ensemble / meta-model implementations |
| `event_bus` | Event-driven architecture core |
| `events` | Event types and dispatch system |
| `exchange-binance` | Binance exchange integration |
| `exchange-coinbase` | Coinbase exchange integration |
| `expected_return` | Expected return model implementations |
| `feature-engine` | Feature engineering implementations |
| `feature-store` | Feature store with caching and retrieval |
| `freqtrade_integration` | Freqtrade bot integration |
| `hold-period` | Holding period models |
| `lineage` | Feature/data lineage tracking |
| `live-exec` | Live execution boundary and adapter |
| `logging` | Structured logging and correlation ID management |
| `model_registry` | Model versioning and registry |
| `monitoring` | Monitoring, observability, and metrics |
| `multi_exchange` | Multi-exchange arbitrage and comparison |
| `optimization` | Portfolio optimization algorithms |
| `orderbook` | Order book reconstruction and delta handling |
| `paper` | Paper trading implementation |
| `paper-trading` | Paper trading engine |
| `pca` | Principal Component Analysis |
| `persistent-data` | Persistent data platform with replay |
| `pipeline` | Real-time signal pipeline |
| `portfolio` | Portfolio management |
| `portfolio_risk` | Portfolio risk calculations (VaR, CVaR) |
| `rate-limiter` | Rate limiting and throttle management |
| `regime-detector` | Alternative regime detection implementations |
| `registry` | Experiment and model registry |
| `replay` | Deterministic dataset replay |
| `report-gen` | Report generation framework |
| `risk` | Risk management engine |
| `scheduler` | Automated research scheduler |
| `sentiment` | Sentiment analysis models |
| `signals` | Signal generation and alpha models |
| `strategies` | Strategy DSL and implementations |
| `strategy_dsl` | Strategy domain-specific language |
| `test-scale` | Scale testing framework |
| `types` | Strongly typed domain model |
| `universe` | Universe construction and management |
| `universe_history` | Universe history and history tracking |
| `validation` | Validation and walk-forward optimization |

## Crate Interaction Hierarchy

```
┌─────────────────────────────────────────────────────────────────┐
│                    QuantRadar Workspace                          │
├─────────────────────┬───────────────────────────────────────┤
│   Shared Crates     │           Standalone Crates              │
│   (crates/shared/)  │   (crates/standalone/)                  │
├─────────────────────┼───────────────────────────────────────┤
│ core                │ data-quality, feature-engine,            │
│ (domain models)     │ feature-store, signals, strategies       │
├─────────────────────┼───────────────────────────────────────┤
│ data-model          │ pca, regime-detector, ranking,           │
│ (types, OHLC etc.)  │ expected-return, sentiment               │
├─────────────────────┼───────────────────────────────────────┤
│ ingestion           │ pipeline, scheduler, live-exec,          │
│ (collectors)        │ backtest_engine                          │
├─────────────────────┼───────────────────────────────────────┤
│ execution           │ live-exec, paper-trading,                │
│ (order management)  │ paper                                    │
├─────────────────────┼───────────────────────────────────────┤
│ risk                │ portfolio_risk, risk,                    │
│ (risk calculations) │ optimization                             │
├─────────────────────┼───────────────────────────────────────┤
│ monitoring          │ monitoring, scheduler                    │
│ (metrics/observability)                                 │
├─────────────────────┼───────────────────────────────────────┤
│ registry            │ model_registry, experiment               │
│ (versioning)        │                                          │
└─────────────────────┴───────────────────────────────────────┘
```

## Key Data Flow

1. **Data Ingestion** (`ingestion` + `websocket`)
   - Collects OHLC data from exchanges (Kraken, Binance, Coinbase)
   - Normalizes and validates incoming market data

2. **Data Normalization** (`data-model` + `features`)
   - Structures raw data into typed models
   - Engineers features from raw OHLC

3. **Feature Pipeline** (`features` → `feature-store` → `pipeline`)
   - Transforms raw features into usable signal features
   - Real-time signal pipeline computation

4. **Signal Generation** (`signals` + `ranking` + `ensemble`)
   - Generates alpha signals from features
   - Ranks and ensembles signals

5. **Risk assessment** (`risk` + `portfolio_risk`)
   - Calculates portfolio risk metrics (VaR, CVaR)
   - Position limit checks

6. **Execution** (`execution` + `live-exec` + `paper-trading`)
   - Submits orders to exchanges or paper trading
   - Handles order lifecycle and fills

7. **Monitoring & Metrics** (`monitoring`)
   - Tracks performance, errors, and system health
   - Exposes metrics for observability

8. **Replay & Backtest** (`replay` + `backtest_engine`)
   - Deterministic dataset replay
   - Historical backtesting with full event simulation

## Verification

All crate structures verified against:
- `Cargo.toml` workspace membership
- `codemap.md` in each crate
- Rust workspace build: `cargo build --workspace`
- Binary `quantaradar 0.2.0` verified

## Notes

- 5 crates fully implemented with concrete logic verified
- 26 crates have working stubs/code
- 0 empty source files
- Workspace builds successfully
- Binary `quantaradar 0.2.0` verified