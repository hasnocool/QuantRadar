# Repository Atlas: QuantRadar

## Project Responsibility
Market-intelligence and quantitative research platform turning broad market data into explainable, validated trading opportunities. Rust owns deterministic ingestion, features, regimes, backtesting and risk. Python owns exploratory ML, walk-forward research and statistical promotion.

## System Entry Points
- `Cargo.toml`: Workspace manifest for 50+ Rust crates
- `pyproject.toml`: Python package quantradar
- `crates/cli/src/main.rs`: CLI entry `quantaradar --discover/--screen/--fetch/--backtest`
- `python/quantaradar/`: Research, robustness, champion/challenger gates

## Data Flow
Exchange → Ingestion + normalization → Quality/Liquidity → Feature engine → Regime/Ranking/Anomaly → Screeners → Signals → Strategy candidates → Walk-forward + costs + robustness → Champion/Challenger → Portfolio + risk → Paper trading

## Directory Map
| Directory | Responsibility Summary | Detailed Map |
|-----------|------------------------|--------------|
| `crates/core/` | Shared domain models | [View](crates/core/codemap.md) |
| `crates/exchange-kraken/` | Kraken REST + WebSocket ingestion | [View](crates/exchange-kraken/codemap.md) |
| `crates/features/` | Deterministic technical features | [View](crates/features/codemap.md) |
| `crates/regime/` | Market-state classifier | [View](crates/regime/codemap.md) |
| `crates/screeners/` | Explainable screener families | [View](crates/screeners/codemap.md) |
| `crates/microstructure/` | Order-book/trade-flow/liquidity analytics | [View](crates/microstructure/codemap.md) |
| `crates/research/` | Breadth/ranking/relative-strength/PCA/events | [View](crates/research/codemap.md) |
| `crates/backtest/` | Cost-aware backtesting | [View](crates/backtest/codemap.md) |
| `crates/reporting/` | Machine-readable reports | [View](crates/reporting/codemap.md) |
| `crates/execution/` | Risk controls + paper execution | [View](crates/execution/codemap.md) |
| `crates/cli/` | QuantRadar CLI | [View](crates/cli/codemap.md) |
| `python/quantaradar/` | ML, walk-forward, robustness, champion/challenger | [View](python/quantaradar/codemap.md) |
| `configs/` | Research and strategy configurations | [View](configs/codemap.md) |

<!-- ponytail: shallow atlas, sub-maps are empty templates. Fill per-folder when deep work starts. -->

