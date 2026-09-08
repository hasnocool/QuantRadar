# QuantRadar

QuantRadar is a market-intelligence and quantitative research platform designed to turn broad market data into explainable, validated trading opportunities.

> **Research broadly, validate honestly, execute conservatively.**

## Architecture

QuantRadar uses a Rust production core and a Python research/ML layer. Rust owns deterministic market-data processing, features, screeners, regimes and backtesting. Python owns exploratory research, ML, optimization and walk-forward analysis.

```text
Kraken/API data
      ↓
Normalization → raw observations → quality/liquidity
      ↓
Feature engine → regime → cross-sectional ranks → anomalies
      ↓
Independent screeners → evidence-backed signals
      ↓
Strategy candidates → walk-forward validation → champion/challenger
      ↓
Portfolio/risk → paper trading → future execution boundary
```

## Implemented foundation

- Dynamic Kraken spot-market discovery.
- Async bounded-concurrency OHLC retrieval.
- EMA, RSI, ATR, volatility, Bollinger-width, volume-z and ATR-normalized distance features.
- Bull/bear/sideways and volatility regime classification.
- Trend, breakout, mean-reversion and volatility-expansion screeners.
- Cost-aware moving-average backtester with fees, slippage and drawdown tracking.
- JSON report writer.
- Python walk-forward fold generation and performance metrics.
- Python Isolation Forest anomaly scoring.

## Repository layout

```text
crates/                  Rust production/research engine
  core/                  shared domain models
  exchange-kraken/       public Kraken REST adapter
  features/              deterministic technical features
  regime/                market-state classifier
  screeners/             explainable screener families
  backtest/              cost-aware backtesting
  reporting/             machine-readable reports
  cli/                    `quantaradar` CLI
python/quantaradar/       research + ML
configs/                  research/strategy configuration
scripts/                  project checks
```

## Quick start

```bash
cargo check --workspace
cargo test --workspace
cargo run -p quantaradar -- discover
cargo run -p quantaradar -- screen --pair BTC/USD --interval 1440
cargo run -p quantaradar -- fetch BTC/USD --interval 1440
cargo run -p quantaradar -- backtest data/raw/BTC_USD.json
```

Python:

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -e '.[dev,ml]'
```

## Research rules

1. Features must be timestamp-safe; no future information may enter a decision.
2. Every strategy change is a challenger and must be benchmarked against a fixed baseline.
3. Backtests must include explicit fees, spread/slippage assumptions and turnover.
4. Validate out-of-sample with expanding walk-forward folds.
5. Evaluate performance by market regime, not only aggregate performance.
6. Retain failed experiments as data for future research.
7. A signal without executable liquidity is not an opportunity.

## Planned expansion

The next major layer is the full market-intelligence engine: order books, trade flow, depth/slippage, relative strength and weakness, breadth, correlation/PCA clustering, event detection, cross-sectional ranking, automated strategy generation, robustness testing, champion/challenger promotion, and paper-trading reconciliation.

Live trading is intentionally not enabled by this initial implementation.
