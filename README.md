# QuantRadar

QuantRadar is a market-intelligence and quantitative research platform designed to turn broad market data into explainable, validated trading opportunities.

> **Research broadly, validate honestly, execute conservatively.**

## Architecture

Rust owns deterministic market-data ingestion, microstructure, features, screeners, regimes, backtesting, risk and paper-execution primitives. Python owns exploratory ML, anomaly detection, optimization, walk-forward research and statistical promotion workflows.

```text
Kraken REST + WebSocket
        ↓
Raw observations → normalization → quality controls
        ↓
OHLC + trades + order books
        ↓
Features → regime → breadth → relative strength/weakness
        ↓
Liquidity/depth/slippage → correlation/PCA → anomalies/events
        ↓
Cross-sectional ranking → independent screeners
        ↓
Strategy generation → backtest → robustness → walk-forward OOS
        ↓
Champion / Challenger gate
        ↓
Portfolio sizing → risk limits → paper orders → fills/reconciliation
        ↓
[future isolated live-execution adapter]
```

## Current implementation

### Market data and microstructure
- Dynamic Kraken spot-market discovery.
- Async bounded-concurrency REST OHLC retrieval.
- Kraken WebSocket v2 ingestion primitives for public book and trade streams.
- Order-book spread, bid/ask depth, imbalance and executable $1K/$10K/$100K impact estimates.
- Liquidity scoring that penalizes wide spreads and poor executable depth.

### Market intelligence
- EMA, RSI, ATR, realized-volatility, Bollinger-width, volume-z and ATR-normalized distance features.
- Bull/bear/sideways/transition regimes.
- Breadth: positive assets, EMA participation, new-high/new-low counts.
- Relative-strength / relative-weakness measurements against a benchmark and cross-section.
- Cross-sectional ranking with regime-aware scoring.
- Correlation matrices and PCA first-component extraction for concentration detection.
- Event detection for breakouts, breakdowns, volume anomalies and volatility spikes.

### Strategy research
- Trend, breakout, mean-reversion and volatility-expansion screeners.
- Programmatic regime-specific strategy specification generation.
- Cost-aware backtesting with fees/slippage and drawdown accounting.
- Expanding walk-forward folds and performance metrics in Python.
- Isolation Forest anomaly scoring.
- Parameter perturbation / robustness testing.
- Champion/challenger promotion gates requiring OOS improvement plus drawdown and profit-factor constraints.

### Extended crate landscape
- New/ext crates cover data quality (`data-quality`), ingestion (`ingestion`), persistent storage (`persistent-data`, `storage`, `archives`), replay (`replay`), multi-exchange (`multi_exchange`, `exchange-binance`, `exchange-coinbase`), event bus (`event_bus`, `events`), feature engine/store (`feature-engine`, `feature-store`), regime detector (`regime-detector`), ranking (`ranking`), signal ensemble (`ensemble`, `signal-ensemble`), backtest engine (`backtest_engine`), DSL (`strategy_dsl`), portfolio risk (`portfolio_risk`), paper execution (`paper`, `paper-trading`), live-exec (`live-exec`), optimization (`optimization`), Freqtrade interop (`freqtrade_integration`), monitoring/scheduler (`monitoring`, `scheduler`), experiment (`experiment`), registry (`registry`, `model_registry`), replay/archives (`replay`, `archives`), and scale testing (`test-scale`).
- `rate-limiter` added for bounded concurrency controls.

### Portfolio and paper trading
- Position-risk sizing from equity, entry and stop distance.
- Per-position size caps, portfolio heat and concurrent-position limits.
- Minimum executable-liquidity gate.
- Paper order intents and simulated book fills with fees.
- Paper-account cash, positions, fills and realized-PnL accounting.
- No live trading credentials or live order submission in this repository.

## Repository layout

```text
crates/
  core/                   shared domain models
  domain-model/           domain types
  data-model/             observation / bar models
  data-quality/           quality flags / validation
  ingestion/              data ingestion pipeline
  persistent-data/        durable storage layer
  storage/ / archives/     historical datasets
  exchange-kraken/        Kraken REST + WebSocket market data
  exchange-binance/       Binance data adapter
  exchange-coinbase/      Coinbase adapter
  multi_exchange/         multi-source coordination
  websocket/              generic WebSocket primitives
  replay/                 deterministic replay
  event_bus/ / events/     event publishing
  features/               deterministic technical features
  feature-engine/         feature computation
  feature-store/          persistent feature storage
  regime/                 market-state classifier
  regime-detector/        regime detection
  screeners/              explainable screener families
  ranking/                cross-sectional ranking
  microstructure/         order-book/trade-flow/liquidity analytics
  research/               breadth / ranking / relative-strength / PCA / events
  backtest/               cost-aware backtesting
  backtest_engine/        backtest execution engine
  pipeline/               signal pipeline
  ensemble/ / signal-ensemble/  meta-model / combined signals
  strategies/             strategy definitions
  strategy_dsl/           DSL generation
  registry/ / model_registry/  experiment / model registry
  execution/              risk controls + paper execution
  paper-trading/ / paper/ paper execution primitives
  live-exec/              isolated live-execution adapter (disabled)
  portfolio/ / portfolio_risk/  sizing + portfolio risk
  risk/                   risk limits / VaR
  reporting/              machine-readable reports
  cli/                    quantaradar CLI
  experiment/             experiment tracking
  optimization/           optimization layer
  freqtrade_integration/  Freqtrade interop
  validation/             validation / walk-forward
  scheduler/ / monitoring/  ops / observability
  test-scale/             scale testing
python/quantaradar/
  robustness.py           robustness + champion/challenger gates
configs/                  research and strategy configurations
scripts/                  deterministic project checks
reports/                  generated research outputs
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

## Research contract

1. No future information may enter a feature, signal or training fold.
2. Every strategy change is a challenger against a fixed baseline.
3. Backtests must include fees, spread/slippage assumptions and turnover where applicable.
4. Walk-forward OOS validation is mandatory for promotion.
5. Results must be segmented by market regime.
6. Liquidity must be evaluated at executable trade size, not just daily volume.
7. Highly correlated assets must not be counted as independent diversification.
8. Failed experiments remain recorded as research data rather than silently discarded.

## Roadmap

The foundation is now in place. The next production layers are persistent market-state storage, full multi-symbol WebSocket fan-out, historical order-book/trade archives, advanced anomaly models, automated portfolio optimization, Freqtrade/backtest interoperability, paper-trading reconciliation dashboards, and a deliberately isolated live-execution adapter that remains disabled by default.
