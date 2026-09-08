# QuantRadar PLAN.md — Organized Index (35 sections, grouped by theme)

| # | Theme | Section | Status |
|---|-------|---------|--------|
| 1 | Data | Persistent market-data system | Stub |
| 2 | Data | WebSocket ingestion | Stub |
| 3 | Data | Order-book model | Stub |
| 4 | Features | Feature engineering | Stub |
| 5 | Regime | Regime detection | Stub |
| 6 | Ranking | Cross-sectional ranking | Stub |
| 7 | Stats | PCA / correlation | Stub |
| 8 | Strategy | Strategy generation (DSL) | Stub |
| 9 | Backtest | Backtest engine | Stub |
| 10 | Exec | Direction enum / execution bug | Implemented |
| 11 | Execution | Paper trading engine | Implemented |
| 12 | Risk | Portfolio VaR / risk limits | Stub |
| 13 | Validation | Validation / walk-forward | Stub |
| 14 | Registry | Experiment registry | Implemented |
| 15 | Registry | Model registry | Stub |
| 16 | Lineage | Feature/data lineage | Stub |
| 17 | Universe | Universe construction | Stub |
| 18 | Events | Delisting / listings / events | Stub |
| 19 | Exchange | Multi-exchange architecture | Stub |
| 20 | Derivatives | Derivatives data layer | Stub |
| 21 | Events | Event intelligence bus | Stub |
| 22 | Intelligence | News / on-chain / sentiment | Stub |
| 23 | Features | Feature store | Implemented |
| 24 | Pipeline | Real-time signal pipeline | Implemented |
| 25 | Ensemble | Signal ensemble / meta-model | Stub |
| 26 | Returns | Expected-return model | Stub |
| 27 | Strategy | Holding-period model | Stub |
| 28 | Ops | Automated research scheduler | Stub |
| 29 | Ops | Monitoring / observability | Design |
| 30 | UI | Dashboard | Stub |
| 31 | Integration | Freqtrade integration | Stub |
| 32 | Execution | Live execution boundary | Stub |
| 33 | Types | Strongly typed domain model | Stub |
| 34 | Testing | Testing at scale | Stub |
| 35 | Replay | Deterministic dataset / replay | Stub |

Status: Implemented = concrete logic verified; Stub = working type/function; Design = needs spec.

---

# QuantRadar PLAN.md — Implementation Status

Status table (current as of last audit): 35 sections; workspace expanded with new/ext crates (`data-quality`, `ingestion`, `persistent-data`, `storage`, `archives`, `replay`, `event_bus`, `events`, `multi_exchange`, `exchange-binance`, `exchange-coinbase`, `websocket`, `feature-engine`, `feature-store`, `regime-detector`, `ranking`, `ensemble`, `signal-ensemble`, `backtest_engine`, `strategy_dsl`, `portfolio_risk`, `live-exec`, `optimization`, `freqtrade_integration`, `monitoring`, `scheduler`, `experiment`, `model_registry`, `test-scale`, `rate-limiter`, `paper-trading`/`paper`, `universe_history`, etc.). 5 fully implemented (#10, #11, #14, #23, #24); 26 have working stubs/code; 0 empty source files; workspace builds; binary `quantaradar 0.2.0` verified.

```
Implemented fully: #10 Direction, #11 PaperAccount, #14 Registry, #23 FeatureStore, #24 Pipeline
Implemented (stub + methods): #8 DSL, #9 Engine, #12 VaR, #13 Validation, #15 ModelReg, #16 Lineage, #17 UnivHistory, #18 Events, #19 MultiEx, #20 Derivatives, #21 EventBus, #22 Sentiment, #25 Ensemble, #26 ExpReturn, #31 Freqtrade
Design-required (needs spec): #29 Monitoring, #35 Replay (stubs present; full engine deferred per #1 persistence)
```

---

# PLAN.md — Original design document (refactored for clarity, preserved in full below)

The repo is split sensibly between Rust deterministic components and Python research/ML components.

| Area | State |
| --- | --- |
| Kraken discovery / REST OHLC / WebSocket / order-book / trade-flow / liquidity / features / regimes / ranking / PCA / events / screeners / backtest / backtest_engine / strategy_dsl / ensemble / multi_exchange / replay / persistent-data / ingestion / data-quality / feature-engine / feature-store / paper / live-exec / portfolio_risk / risk / registry / model_registry / monitoring / scheduler / test-scale / rate-limiter / reporting | Implemented / stubbed |
| Persistent market-data platform (full persistence/replay/lineage) / full multi-exchange / portfolio optimization / automated optimization (`optimization`) / Freqtrade (`freqtrade_integration`) / dashboards (`dashboard`) / live-adapter (`live-exec`) / replay (`replay`) / archives (`archives`) | Design required / partial (see #1, #30, #31, #19, #12, #35) |

---

# 1. A real persistent market-data system

This is probably the **largest architectural hole**.

Right now the exchange layer retrieves OHLC and has WebSocket primitives, but I don't see a real data platform around them.

You need:

```text
Exchange
  ↓
Collector
  ↓
Normalizer
  ↓
Quality validator
  ↓
Immutable event log
  ↓
Historical store
  ↓
Derived bars/features
```

Missing capabilities include:

* persistent tick storage
* persistent trades
* persistent order-book snapshots
* order-book deltas
* book reconstruction
* sequence-number validation
* gap detection
* reconnect/replay
* historical backfill
* deduplication
* late-event handling
* event-time vs processing-time
* data versioning
* dataset manifests
* universe snapshots
* retention policies
* Parquet/Arrow datasets
* efficient time-range queries

The architecture says raw observations should be immutable and include fields such as timestamp, exchange, bid/ask, depths, source, ingestion time and quality flags, but the current implementation does not yet constitute that durable observation system.

### What I'd add

```text
crates/data-model
crates/data-quality
crates/ingestion
crates/storage
crates/orderbook
crates/archives
crates/persistent-data
crates/replay
crates/event_bus
crates/websocket
crates/feature-engine
crates/feature-store
crates/ensemble
crates/signal-ensemble
crates/model_registry
crates/portfolio_risk
crates/live-exec
crates/rate-limiter
crates/backtest_engine
crates/strategy_dsl
```

And something like:

```text
data/
  raw/
    trades/
    books/
    ohlcv/
  normalized/
  features/
  datasets/
  manifests/
```

---

# 2. WebSocket ingestion is not yet a production feed handler

The repo has Kraken REST and WebSocket code, but the actual architecture needs to become a **continuous market-data engine**, not just a client.

For example, you need:

```text
MarketFeedManager
 ├── BTC/USD
 ├── ETH/USD
 ├── SOL/USD
 ├── ...
 └── N symbols
```

with:

* bounded concurrency
* reconnect/backoff
* heartbeat monitoring
* subscription management
* sequence validation
* stale-feed detection
* automatic resubscription
* per-symbol buffers
* backpressure
* dropped-message accounting
* feed health scores

The current Kraken REST client itself simply sleeps a fixed 350ms before requests, which is a primitive rate-limit approach rather than a real adaptive rate limiter.

A production implementation should use a shared asynchronous rate limiter rather than sleeping inside every request.

---

# 3. The order book model is too thin

`OrderBookSnapshot` currently contains:

```text
bid
ask
bid_depth
ask_depth
```

and `MicrostructureFeatures` computes spread, depth, imbalance, trade ratio, and impact estimates.

That's useful, but missing:

* full L2 state
* delta updates
* book age
* queue position
* depth by distance from mid
* bid/ask slope
* convexity
* order-flow imbalance
* CVD
* aggressive buy/sell volume
* signed trade volume
* trade intensity
* inter-arrival times
* VPIN-style toxicity
* cancel/add ratios
* replenishment
* spoofing indicators
* sweep detection
* iceberg-like behavior
* short-term impact models
* liquidity regime detection

The current impact calculation is also relatively simplistic: it walks one side of the snapshot and estimates average fill price, but it doesn't model partial/incomplete liquidity or execution dynamics realistically.

---

# 4. Feature engineering is far too small for the eventual system

The current `FeatureRow` is mostly:

* returns
* EMA
* RSI
* ATR
* realized volatility
* Bollinger width
* volume z-score
* EMA distance
* breakout/new high/new low

That's a good baseline, but a serious cross-sectional crypto system needs several hundred candidate features.

### Missing technical features

Examples:

* MACD
* ADX
* directional movement
* stochastic
* Williams %R
* CCI
* ROC
* momentum acceleration
* volatility ratios
* ATR percentile
* realized-vol term structure
* skew
* kurtosis
* range expansion
* gap-like behavior
* candle structure
* wick ratios
* volume-price trend
* OBV
* VWAP deviations
* rolling beta
* rolling alpha
* residual momentum

### Missing market structure

* higher-high/lower-low structure
* support/resistance
* volatility compression
* breakout quality
* trend persistence
* trend age
* reversal probability
* distance to local extremes

### Missing cross-sectional factors

* momentum factor
* volatility factor
* liquidity factor
* size proxy
* trend quality
* reversal
* volume surprise
* market beta
* residual momentum

---

# 5. Regime detection is still simplistic

The repo has a regime enum with:

```text
BullTrend
BullHighVol
BullLowVol
BearTrend
BearHighVol
BearLowVol
SidewaysHighVol
SidewaysLowVol
TransitionBull
TransitionBear
Unknown
```

That's a good taxonomy, but the actual regime engine needs to become much more sophisticated.

I'd add:

### Multi-timeframe regime

```text
5m
15m
1h
4h
1d
1w
```

and derive:

```text
micro regime
short-term regime
swing regime
macro regime
```

### Statistical regime models

* HMM
* Bayesian regime switching
* change-point detection
* clustering
* volatility-state models
* trend-state models

### Regime confidence

Instead of:

```text
BullTrend
```

produce:

```text
regime = BullTrend
confidence = 0.81
trend_strength = 0.76
volatility_state = elevated
transition_probability = 0.23
```

---

# 6. The cross-sectional ranking system needs a lot more sophistication

There is a ranking implementation, but it is currently a hand-built weighted score using trend, momentum, breakout, liquidity and regime adjustment.

That's useful as a baseline, but you eventually want:

```text
raw features
    ↓
winsorization
    ↓
cross-sectional normalization
    ↓
sector/category neutralization
    ↓
factor construction
    ↓
ensemble scoring
    ↓
regime-conditioned ranking
```

Missing:

* feature standardization
* winsorization
* neutralization
* factor orthogonalization
* rank transforms
* nonlinear models
* uncertainty estimates
* score calibration
* ranking stability
* turnover-aware ranking
* liquidity-aware ranking
* exposure constraints

---

# 7. PCA is not enough for correlation/clustering

The repository has correlation matrices and a first PCA component.

You need:

```text
Correlation
     ↓
Distance matrix
     ↓
Hierarchical clustering
     ↓
Cluster identities
     ↓
Cluster representatives
     ↓
Portfolio concentration controls
```

Add:

* hierarchical clustering
* DBSCAN/HDBSCAN
* rolling correlations
* shrinkage covariance
* Ledoit-Wolf covariance
* eigenvalue monitoring
* factor exposure
* cluster-aware position limits
* effective number of bets

A portfolio with 10 highly correlated altcoins shouldn't be treated as 10 independent positions.

---

# 8. Strategy generation is currently just parameter templates

`generate_strategies()` currently maps regimes to a small number of strategy families and produces parameters such as risk-per-trade, ATR stop, take-profit and minimum liquidity.

That's **not automated strategy discovery yet**.

You eventually want:

```text
Features
  ↓
Hypothesis generator
  ↓
Strategy DSL
  ↓
Parameter search
  ↓
Backtest
  ↓
OOS validation
  ↓
Robustness
  ↓
Promotion
```

Potential strategy generators:

* rule-based composition
* genetic programming
* symbolic regression
* feature selection
* Bayesian optimization
* Optuna-style search
* constrained combinatorial search
* ML policy candidates

But crucially, every generated strategy must go through the same anti-overfitting pipeline.

---

# 9. Backtesting is currently much too simplistic

This is one of the biggest weaknesses.

The current backtester is essentially an SMA crossover engine with fee and slippage assumptions.

It does **not** yet provide the realism you'd want for the project.

Missing:

### Execution realism

* bid/ask execution
* limit-order behavior
* partial fills
* order queue
* latency
* market impact
* spread widening
* liquidity depletion
* order rejection
* order expiry
* cancellation
* slippage distributions

### Portfolio realism

* multiple simultaneous symbols
* cash management
* leverage
* margin
* borrowing
* shorting
* funding
* collateral
* portfolio heat
* factor exposure
* correlation constraints
* rebalancing

### Statistical realism

* bootstrap
* Monte Carlo paths
* trade-order randomization
* block bootstrap
* regime permutation
* parameter perturbation
* transaction-cost perturbation

---

# 10. There is a concrete execution bug right now

This one is important.

Screeners generate:

```rust
direction: "long"
```

while `approve()` checks:

```rust
if signal.direction != "LONG"
```

So the screener output and risk/execution layer don't agree on the enum/string format.

That means the current approval path can reject valid screener signals.

The screener explicitly creates `"long"` signals.

The execution module explicitly requires `"LONG"`.

**This should be fixed immediately by replacing free-form strings with a strongly typed `Direction` enum.**

---

# 11. Paper trading is not yet a real paper-trading engine

The paper account is currently very small:

```text
cash
peak_equity
equity
realized_pnl
fills
positions
```

Missing:

* unrealized PnL
* equity mark-to-market
* position lifecycle
* stop orders
* take-profit orders
* trailing stops
* order state machine
* cancellations
* rejected orders
* partial fills
* pending orders
* reconciliation
* portfolio snapshots
* daily PnL
* risk events
* broker/exchange state comparison

Also, `PaperAccount` derives `Default`, which leaves numeric fields at zero rather than initializing a coherent starting account state.

---

# 12. Risk management needs to become portfolio-level

Current sizing is a useful start, but the system doesn't yet have full institutional-style portfolio risk.

Missing:

```text
portfolio VaR
CVaR
volatility targeting
beta targeting
factor exposure
cluster limits
correlation limits
gross exposure
net exposure
leverage limits
drawdown throttling
risk-of-ruin
liquidity-adjusted exposure
stress testing
scenario analysis
```

And dynamic risk scaling:

```text
normal regime → 100% risk
high vol → 50%
extreme vol → 25%
market dislocation → 0%
```

---

# 13. The validation system is conceptually good but operationally incomplete

The validation document says the correct things:

* train/test separation
* expanding WFO
* regime segmentation
* OOS trade count
* cost perturbation
* leakage review
* concentration controls
* failed experiment retention

But the actual Python implementation is still quite small.

The current robustness module mainly does parameter scaling and basic pass-rate logic.

You still need:

### Walk-forward engine

```text
train
validate
test
roll forward
repeat
```

### Multiple-test protection

This is particularly important because automated strategy generation can generate thousands of hypotheses.

Add:

* multiple hypothesis correction
* false-discovery controls
* Deflated Sharpe Ratio
* Probability of Backtest Overfitting
* White's Reality Check
* Hansen SPA
* nested walk-forward validation

Otherwise the system will eventually select lucky strategies.

---

# 14. No experiment registry

This is a major missing component.

Every experiment should produce:

```text
experiment_id
strategy_id
git_commit
dataset_id
universe_id
config_id
feature_version
model_version
timestamp
random_seed
training_period
validation_period
test_period
metrics
cost assumptions
artifacts
promotion decision
```

Then:

```text
Experiment
    ↓
Candidate
    ↓
Backtest
    ↓
Validation
    ↓
Robustness
    ↓
Champion/Challenger
```

This is essential for reproducibility.

---

# 15. No model registry

You're going to need something like:

```text
models/
  champion/
  challengers/
  retired/
```

with metadata:

```text
model_id
version
features
training_data
hyperparameters
metrics
regimes
deployment_status
```

Eventually:

```text
Champion
   ↑
Challenger
   ↓
Shadow
   ↓
Retired
```

---

# 16. No feature/data lineage system

This is particularly important for avoiding accidental leakage.

Every feature should know:

```text
feature_id
formula
inputs
lookback
minimum_history
availability_delay
version
```

Example:

```text
momentum_24h
inputs:
  close[t]
  close[t-24]

available_at:
  t
```

versus:

```text
future_return_24h
available_at:
  t+24h
```

The system should mechanically prevent the latter from entering a feature matrix.

---

# 17. Universe construction is underdeveloped

Dynamic discovery exists, which is good.

But you need historical universe membership.

Otherwise you get survivorship bias.

You need:

```text
Universe(t)
```

rather than:

```text
Universe(now)
```

Historical backtests need to know:

* which assets existed
* when they listed
* when they delisted
* which markets were suspended
* liquidity at that time
* price availability at that time

---

# 18. No delisting / listing / corporate-event-style handling

For crypto that translates to:

* listings
* delistings
* migrations
* symbol changes
* token redenominations
* chain migrations
* wrapped/unwrapped assets
* exchange-specific symbol changes

Those events need to become first-class data.

---

# 19. No multi-exchange architecture

The repository is currently centered on Kraken.

That's fine for the first implementation, but the eventual architecture should be:

```text
Exchange trait
 ├── Kraken
 ├── Coinbase
 ├── Binance
 ├── OKX
 ├── Bybit
 ├── Bitfinex
 └── ...
```

Then normalize everything into common domain models.

This also enables:

* cross-exchange arbitrage signals
* price dislocations
* venue liquidity comparison
* cross-venue volume
* lead/lag relationships

---

# 20. No derivatives data layer

For a serious crypto quant platform, this is a huge omission.

Eventually add:

* perpetual futures
* funding rates
* open interest
* liquidations
* basis
* futures term structure
* mark/index price
* long/short ratios
* options
* implied volatility
* skew
* volatility surface

Then signals become much stronger:

```text
price
+ spot volume
+ order flow
+ open interest
+ funding
+ liquidations
+ basis
```

---

# 21. No event intelligence layer

The current event detection is basic:

* new high
* new low
* volume anomaly
* volatility spike

You want a proper event bus:

```text
Event
 ├── market
 ├── asset
 ├── liquidity
 ├── volatility
 ├── derivatives
 ├── exchange
 └── external/news
```

Examples:

```text
LIQUIDITY_COLLAPSE
FUNDING_EXTREME
OI_SURGE
LIQUIDATION_CASCADE
VOL_BREAKOUT
SPREAD_EXPANSION
ORDERBOOK_IMBALANCE
CORRELATION_BREAK
REGIME_CHANGE
MOMENTUM_FAILURE
```

---

# 22. No news/on-chain/sentiment layer

For your long-term vision, this is another major expansion area.

Add optional data providers for:

### News

* headlines
* sentiment
* entity extraction
* event classification
* source credibility
* novelty

### On-chain

* exchange inflows/outflows
* whale transfers
* active addresses
* stablecoin supply
* token velocity
* holder concentration
* staking flows

These should become features, not hard-coded trading decisions.

---

# 23. No feature store

You need a reusable feature store instead of recomputing everything ad hoc.

Something like:

```text
FeatureStore
  ├── price
  ├── volume
  ├── technical
  ├── microstructure
  ├── cross-sectional
  ├── regime
  ├── derivatives
  ├── on-chain
  └── event
```

with timestamp-safe retrieval:

```python
features.as_of(timestamp)
```

---

# 24. No real-time signal pipeline

The repo has individual components, but not yet the production stream:

```text
market event
   ↓
state update
   ↓
feature update
   ↓
regime update
   ↓
screener evaluation
   ↓
cross-sectional ranking
   ↓
signal aggregation
   ↓
portfolio risk
   ↓
order intent
   ↓
execution simulator
```

That's the core missing "nervous system".

---

# 25. No signal ensemble / meta-model

Currently screeners operate independently.

You need:

```text
Trend              ─┐
Breakout            │
Mean Reversion      │
Vol Expansion       ├──> Signal Ensemble
Microstructure      │
Relative Strength   │
Anomaly             │
Event               ┘
```

Then:

```text
raw signal
↓
confidence
↓
regime compatibility
↓
liquidity
↓
cross-sectional rank
↓
expected return
↓
risk-adjusted score
```

---

# 26. No expected-return model

A score of `0.82` is not an expected return.

The eventual system should estimate something like:

```text
P(up)
expected return
expected adverse excursion
expected favorable excursion
expected holding period
expected volatility
confidence
```

Then you can calculate:

```text
expected alpha / expected risk / expected cost
```

instead of simply ranking heuristic scores.

---

# 27. No holding-period model

Different strategies have different horizons.

Add predictions for:

```text
5m
15m
1h
4h
1d
3d
7d
```

This is important for combining strategies without mixing incompatible signals.

---

# 28. No automated research scheduler

Eventually the platform should continually run:

```text
discover markets
↓
update datasets
↓
calculate features
↓
run screeners
↓
generate hypotheses
↓
backtest
↓
walk-forward
↓
robustness
↓
promote challengers
↓
paper trade
↓
monitor
```

This is the autonomous research loop you've been aiming at.

---

# 29. No monitoring/observability stack

You need metrics for the platform itself:

```text
feed latency
dropped messages
data gaps
feature latency
signal latency
orders
fills
slippage
PnL
drawdown
model drift
feature drift
strategy decay
```

And alerts.

---

# 30. No dashboard

The repository needs a proper research/trading UI eventually.

I would build dashboards for:

### Market Radar

```text
Symbol
Score
Regime
Momentum
Liquidity
Order Flow
Volatility
Relative Strength
Events
```

### Strategy Lab

```text
Strategy
Return
Sharpe
Sortino
Max DD
PF
Turnover
OOS
Robustness
Status
```

### Portfolio

```text
Equity
PnL
Risk
Heat
Positions
Clusters
Exposure
Drawdown
```

### Data Health

```text
Feeds
Latency
Missing data
Order-book gaps
Provider status
```

---

# 31. Freqtrade integration is still missing

The README explicitly lists Freqtrade interoperability as future work.

That should eventually be a formal adapter:

```text
QuantRadar Strategy
        ↓
Strategy translator
        ↓
Freqtrade strategy
        ↓
Freqtrade backtest
        ↓
QuantRadar validation
```

And the other direction:

```text
Freqtrade result
        ↓
QuantRadar experiment registry
```

---

# 32. Live execution boundary does not exist yet

The repo intentionally has no live trading implementation, which is actually a good safety decision. The README explicitly says live execution is isolated/future.

Eventually you want:

```text
Research
   ↓
Promotion Gate
   ↓
Paper
   ↓
Shadow
   ↓
Canary
   ↓
Live
```

with hard barriers.

Never:

```text
strategy → exchange
```

directly.

---

# 33. Strongly typed domain model is needed

There's too much stringly typed data.

For example:

```rust
direction: String
side: String
family: String
kind: String
```

This will cause bugs.

Use enums:

```rust
enum Direction {
    Long,
    Short,
    Flat,
}

enum OrderSide {
    Buy,
    Sell,
}

enum EventKind {
    Breakout,
    Breakdown,
    VolumeAnomaly,
    VolatilitySpike,
    ...
}
```

This also would have prevented the `"long"` vs `"LONG"` execution bug.

---

# 34. Testing needs to scale dramatically

The repo has some unit tests, but it needs:

```text
unit tests
integration tests
property tests
golden-data tests
replay tests
backtest consistency tests
data-leakage tests
serialization compatibility tests
exchange fixture tests
load tests
failure/reconnect tests
```

Especially:

### Property testing

Examples:

```text
position size <= risk limits
portfolio heat <= configured maximum
no future timestamps enter feature calculation
order book replay == snapshot state
backtest replay is deterministic
```

---

# 35. No deterministic dataset/replay engine

This is a major quant-engineering capability.

You want to be able to say:

```bash
quantaradar replay dataset_2026_09_01
```

and recreate:

```text
market state
features
signals
orders
fills
portfolio
PnL
```

exactly.

Same input + same config + same commit = same result.

---

# 36. No research artifact/report pipeline

There is reporting infrastructure in the repo, but it needs to become a real research product.

Every experiment should generate:

```text
experiment.json
metrics.json
trades.parquet
equity.parquet
signals.parquet
config.yaml
feature_manifest.json
charts/
report.html
report.md
```

That gives you an auditable research archive.

---

# My assessment

I'd score the current repository roughly:

| Capability                  | Maturity |
| --------------------------- | -------: |
| Architecture                | **8/10** |
| Rust organization           | **7/10** |
| Market-data foundation      | **5/10** |
| Microstructure              | **4/10** |
| Feature engine              | **4/10** |
| Regime engine               | **4/10** |
| Screening                   | **5/10** |
| Cross-sectional research    | **4/10** |
| Strategy generation         | **3/10** |
| Backtesting                 | **3/10** |
| Robust validation           | **3/10** |
| Portfolio optimization      | **2/10** |
| Paper trading               | **3/10** |
| Data persistence            | **1/10** |
| Experiment tracking         | **1/10** |
| Monitoring                  | **1/10** |
| Autonomous research loop    | **1/10** |
| Live execution architecture | **1/10** |

So I'd call the current repo approximately **30–40% of the way toward the QuantRadar you actually described**, despite the README looking much more complete.

The reason is that a lot of the **interfaces and concepts exist**, but many components are still minimal implementations rather than industrial-strength systems. The repository layout itself is already pointed in the right direction, with dedicated crates for research, microstructure, backtesting, reporting and execution.

## What I would build next

The best next phase is not adding more indicators.

I'd make the next architecture:

```text
                  ┌─────────────────────┐
                  │ Exchange Connectors │
                  │ Kraken + future CEX │
                  └──────────┬──────────┘
                             ↓
                 ┌───────────────────────┐
                 │ Real-Time Event Bus   │
                 └──────────┬────────────┘
                            ↓
        ┌────────────────────────────────────┐
        │ Immutable Market Data + Replay     │
        │ OHLC / Trades / L2 / Metadata      │
        └────────────────┬───────────────────┘
                         ↓
              ┌────────────────────┐
              │ Feature Store      │
              └─────────┬──────────┘
                        ↓
      ┌───────────────────────────────────────┐
      │ Intelligence Layer                    │
      │ regime / factors / breadth / RS       │
      │ microstructure / anomalies / events   │
      └──────────────────┬────────────────────┘
                         ↓
                ┌──────────────────┐
                │ Screener Engine  │
                └────────┬─────────┘
                         ↓
                ┌──────────────────┐
                │ Signal Ensemble  │
                └────────┬─────────┘
                         ↓
              ┌──────────────────────┐
              │ Strategy Lab         │
              │ generate + optimize  │
              └──────────┬───────────┘
                         ↓
              ┌──────────────────────┐
              │ Validation Engine    │
              │ WFO / OOS / MC / PBO │
              └──────────┬───────────┘
                         ↓
                ┌──────────────────┐
                │ Model Registry   │
                │ Champion/Chall.  │
                └────────┬─────────┘
                         ↓
              ┌──────────────────────┐
              │ Portfolio Optimizer  │
              └──────────┬───────────┘
                         ↓
                 ┌─────────────────┐
                 │ Risk Engine     │
                 └────────┬────────┘
                          ↓
                 ┌─────────────────┐
                 │ Paper Execution │
                 └────────┬────────┘
                          ↓
                 ┌─────────────────┐
                 │ Monitoring      │
                 └─────────────────┘
```

### The immediate priority order

**1. Persistent data + replay**

**2. Production-grade multi-symbol WebSocket engine**

**3. Strong typed domain/event model**

**4. Real feature store**

**5. Full cross-sectional intelligence/factor engine**

**6. Realistic portfolio-aware backtester**

**7. Experiment registry + dataset lineage**

**8. Proper walk-forward/anti-overfitting framework**

**9. Portfolio optimizer + advanced risk**

**10. Signal ensemble + expected-return model**

**11. Paper-trading state machine + reconciliation**

**12. Dashboard/monitoring**

**13. Freqtrade integration**

**14. Only then the isolated live-execution adapter**

The good news is that **I would not throw away the current QuantRadar codebase**. The crate boundaries are already heading toward the correct architecture; the main job now is to turn the small deterministic functions into robust subsystems and wire them into a continuous research/execution pipeline.

One especially important first commit would be to fix the typed signal/order contract and then build the persistent event/replay layer underneath everything else.






















############################
### Description of logic ###
############################




















Yes. Taken together, the **two QuantRadar conversations describe a much larger system than the repository currently implements**.

I’d describe the combined project as this:

# QuantRadar — Autonomous Quantitative Market Intelligence & Strategy Research Platform

QuantRadar is intended to be an **always-running quantitative research and market-surveillance system** that continuously watches markets, discovers opportunities, generates and tests trading hypotheses, validates them out-of-sample, promotes only robust strategies, and manages them through paper trading before any eventual live execution.

The core philosophy is:

> **Scan broadly → explain opportunities → test ruthlessly → reject overfitting → promote robust strategies → manage risk → paper trade → monitor continuously.**

The important distinction is that **QuantRadar is not primarily a trading bot**.

It is a **quant research factory with a trading execution boundary**.

---

# 1. The overall system

The combined vision looks like:

```text
                         QUANTRADAR
                              │
              ┌───────────────┴────────────────┐
              │                                │
        MARKET DATA                      RESEARCH DATA
              │                                │
              ▼                                ▼
     Exchange Connectors               News / On-chain /
     REST + WebSocket                  Derivatives / Events
              │                                │
              └───────────────┬────────────────┘
                              ▼
                    DATA NORMALIZATION
                              │
                              ▼
                   DATA QUALITY ENGINE
                              │
                              ▼
                 IMMUTABLE DATA ARCHIVE
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
          Historical Data              Real-time State
                │                           │
                └─────────────┬─────────────┘
                              ▼
                      FEATURE ENGINE
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
       Technical         Microstructure       Cross-sectional
       Features              Features             Factors
          │                   │                   │
          └───────────────────┼───────────────────┘
                              ▼
                      REGIME ENGINE
                              │
          ┌───────────────────┼───────────────────┐
          ▼                   ▼                   ▼
        Market              Asset              Volatility
        Regime              Regime                Regime
                              │
                              ▼
                       MARKET RADAR
                              │
        ┌─────────────────────┼──────────────────────┐
        ▼                     ▼                      ▼
     Screeners             Events                Rankings
        │                     │                      │
        └─────────────────────┼──────────────────────┘
                              ▼
                       SIGNAL ENSEMBLE
                              │
                              ▼
                  EXPECTED RETURN / RISK MODEL
                              │
                              ▼
                    STRATEGY GENERATOR
                              │
                              ▼
                     STRATEGY CANDIDATES
                              │
                              ▼
                       BACKTEST ENGINE
                              │
                              ▼
                  WALK-FORWARD VALIDATION
                              │
                              ▼
                     ROBUSTNESS TESTING
                              │
                              ▼
                 ANTI-OVERFITTING ANALYSIS
                              │
                              ▼
                  CHAMPION / CHALLENGER
                              │
                              ▼
                    PORTFOLIO OPTIMIZER
                              │
                              ▼
                       RISK ENGINE
                              │
                              ▼
                       PAPER TRADING
                              │
                              ▼
                     RECONCILIATION
                              │
                              ▼
                       MONITORING
                              │
                              ▼
                 ┌────────────┴────────────┐
                 │                         │
             RETIRE                     PROMOTE
                 │                         │
                 └────────────┬────────────┘
                              ▼
                    FUTURE LIVE ADAPTER
```

---

# 2. What the first conversation was really defining

The first QuantRadar discussion was essentially about **expanding the existing repository into the complete quantitative research stack**.

That conversation identified the major missing layers:

### Market intelligence

The system should continuously analyze:

* price
* volume
* volatility
* trends
* momentum
* breakouts
* mean reversion
* breadth
* relative strength/weakness
* correlations
* PCA/factor structure
* order books
* trade flow
* executable liquidity
* anomalies
* market events
* regime changes

The repository already contains many of these concepts, including breadth, relative strength, ranking, correlation/PCA, events and strategy generation.

---

# 3. What the second conversation adds

The second conversation moves beyond "market scanner" into an **autonomous research machine**.

Instead of:

```text
human writes strategy
        ↓
backtest
```

you want:

```text
market data
   ↓
find interesting behavior
   ↓
form hypothesis
   ↓
generate strategy
   ↓
test strategy
   ↓
stress strategy
   ↓
validate strategy
   ↓
compare with champion
   ↓
paper trade
   ↓
monitor decay
   ↓
retire/promote
```

That is a fundamentally different system.

The machine is effectively asking:

> **"What works in this market regime, on which assets, under what conditions, and does it continue to work after realistic costs and out-of-sample testing?"**

---

# 4. QuantRadar should have multiple layers of screeners

The screener system should not be one giant "buy signal."

It should have independent research families.

For example:

```text
TREND
 ├── moving-average trend
 ├── ADX trend strength
 ├── trend persistence
 └── momentum continuation

BREAKOUT
 ├── range breakout
 ├── volatility compression
 ├── volume-confirmed breakout
 └── price/volume expansion

MEAN REVERSION
 ├── oversold
 ├── deviation from mean
 ├── volatility-adjusted reversion
 └── liquidity-supported reversal

MOMENTUM
 ├── short-term momentum
 ├── medium-term momentum
 ├── acceleration
 └── residual momentum

MICROSTRUCTURE
 ├── order-book imbalance
 ├── aggressive flow
 ├── depth deterioration
 ├── spread changes
 └── market impact

EVENT
 ├── volume anomaly
 ├── volatility spike
 ├── new high/low
 ├── regime change
 └── correlation break
```

The current repository already has trend, breakout, mean-reversion and volatility-expansion screeners.

---

# 5. Then QuantRadar combines evidence

A critical idea from the combined design is that **a screener should not directly become an order**.

Instead:

```text
Trend signal
       +
Momentum signal
       +
Relative strength
       +
Order-flow imbalance
       +
Liquidity
       +
Regime compatibility
       +
Event confirmation
       ↓
    Composite Signal
```

For example:

```text
BTC/USD

Trend:              +0.82
Momentum:           +0.76
Relative strength:  +0.91
Liquidity:          +0.98
Order flow:         +0.73
Regime fit:         +0.88
Event strength:     +0.65
────────────────────────────
Composite:          +0.84
```

This is much more useful than:

```text
BUY BTC
```

because the system can explain **why**.

The repository's `Signal` model already stores timestamp, symbol, family, direction, score, regime, rationale and feature evidence, which is the correct direction.

---

# 6. The real heart of QuantRadar: strategy discovery

This is where I think the two conversations become much more ambitious.

QuantRadar should continuously generate hypotheses such as:

```text
"When BTC is in a low-volatility bullish regime,
coins with strong 24h relative strength,
positive order-flow imbalance,
and a 20-period breakout may outperform
over the following 24–72 hours."
```

That hypothesis becomes machine-testable.

Then QuantRadar automatically asks:

```text
Does it work?
Does it work after fees?
Does it work after slippage?
Does it work on unseen data?
Does it work across different years?
Does it work across different assets?
Does it work in multiple regimes?
Is it just one lucky parameter combination?
Does it survive parameter perturbation?
Does it survive transaction-cost increases?
Does it survive randomized trade order?
```

Only then does it become a candidate.

---

# 7. The backtester should be a research laboratory

The current repository has a basic cost-aware backtester, but the intended platform goes much further.

The eventual backtester should support:

```text
historical candles
+
trades
+
order books
+
spread
+
liquidity
+
latency
+
fees
+
market impact
+
partial fills
+
portfolio constraints
+
funding
+
execution rules
```

So instead of assuming:

```text
buy at close
```

the engine might determine:

```text
signal generated at 12:03:14.251
↓
order submitted at 12:03:14.291
↓
book state at arrival
↓
available liquidity
↓
partial fill
↓
price impact
↓
fees
↓
remaining quantity
```

That is vastly closer to actual trading.

---

# 8. Walk-forward validation becomes mandatory

The system should never simply optimize on all historical data.

Instead:

```text
TRAIN ────────► TEST
             ↓
        move forward
             ↓
TRAIN ─────────────► TEST
                    ↓
               move forward
```

For example:

```text
2019-2021 train
2022 test

2020-2022 train
2023 test

2021-2023 train
2024 test

2022-2024 train
2025 test
```

Then aggregate the OOS performance.

The repository's validation contract already explicitly requires expanding walk-forward folds and OOS evaluation.

---

# 9. QuantRadar needs to fight its own tendency to overfit

This is one of the most important concepts in the second conversation.

An autonomous strategy generator could discover **thousands of fake strategies**.

So the system must assume:

> **Every strategy is guilty until proven robust.**

The validation stack should include:

```text
Walk-forward
      ↓
OOS performance
      ↓
Parameter perturbation
      ↓
Transaction-cost perturbation
      ↓
Bootstrap
      ↓
Monte Carlo
      ↓
Trade-order randomization
      ↓
Regime testing
      ↓
Cross-asset testing
      ↓
Multiple-hypothesis correction
      ↓
Overfitting diagnostics
```

The existing Python robustness layer is an early version of this idea.

---

# 10. Champion / Challenger is the evolutionary mechanism

This is another major idea.

You shouldn't constantly replace the strategy with the latest backtest winner.

Instead:

```text
                 CHAMPION
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   Challenger A Challenger B Challenger C
        │           │           │
        └───────────┼───────────┘
                    ▼
               Validation
                    │
            ┌───────┴───────┐
            ▼               ▼
          Reject          Promote
```

A new strategy must outperform the current champion **out of sample**, while also satisfying risk and robustness requirements.

The current repository already has a basic champion/challenger promotion function.

---

# 11. Then it becomes a portfolio system

The system shouldn't say:

```text
BTC = buy
ETH = buy
SOL = buy
AVAX = buy
```

and buy everything.

Instead:

```text
signals
   ↓
expected returns
   ↓
correlation
   ↓
factor exposure
   ↓
liquidity
   ↓
volatility
   ↓
portfolio optimizer
   ↓
position sizes
```

So perhaps:

```text
BTC    6.2%
ETH    3.1%
SOL    1.4%
AVAX   0.4%
```

rather than equal allocations.

And the optimizer should understand that:

```text
SOL + AVAX + NEAR
```

might effectively be one correlated bet.

The repository already recognizes correlation/PCA as part of the research pipeline, but this needs to connect directly into portfolio construction.

---

# 12. Risk is a first-class subsystem

The combined design is not:

```text
find alpha → trade
```

It is:

```text
find alpha
   ↓
estimate uncertainty
   ↓
estimate liquidity
   ↓
estimate correlation
   ↓
measure portfolio risk
   ↓
decide how much capital is actually allowed
```

Risk controls eventually need:

```text
per-trade risk
portfolio heat
max position
max cluster exposure
max leverage
max drawdown
volatility targeting
VaR
CVaR
stress tests
liquidity limits
correlation limits
regime-based risk reduction
```

The repository already contains basic position sizing and portfolio constraints.

---

# 13. Paper trading becomes the final proving ground

After a strategy passes research:

```text
RESEARCH
   ↓
OOS
   ↓
ROBUSTNESS
   ↓
CHAMPION
   ↓
PAPER
```

Then paper trading measures:

* expected vs actual fills
* actual slippage
* signal decay
* live liquidity
* execution timing
* portfolio behavior
* operational failures

Only after enough evidence should it become eligible for a future live adapter.

The repository intentionally stops short of live trading today, which is sensible.

---

# 14. The eventual system should learn from failure

This is perhaps the biggest philosophical part of the second chat.

A failed strategy shouldn't disappear.

Instead:

```text
Experiment #1842

Hypothesis:
Momentum breakout

Result:
FAILED

Why:
Works only on BTC
Fails on midcaps
High turnover
Dies in high-vol regime

Stored as:
FAILED / REGIME_DEPENDENT
```

Then a future research process can learn:

```text
Don't waste 50,000 experiments
rediscovering this failed configuration.
```

So QuantRadar eventually becomes a **research memory system**.

---

# 15. The repository architecture should therefore become

I'd ultimately organize it approximately like this:

```text
QuantRadar/
│
├── crates/
│   ├── core/
│   ├── types/
│   ├── events/
│   ├── ingestion/
│   ├── exchange-kraken/
│   ├── exchange-coinbase/
│   ├── exchange-binance/
│   ├── storage/
│   ├── replay/
│   ├── data-quality/
│   ├── orderbook/
│   ├── microstructure/
│   ├── features/
│   ├── factors/
│   ├── regime/
│   ├── screeners/
│   ├── research/
│   ├── signals/
│   ├── strategies/
│   ├── backtest/
│   ├── validation/
│   ├── optimization/
│   ├── portfolio/
│   ├── risk/
│   ├── execution/
│   ├── paper/
│   ├── reporting/
│   ├── monitoring/
│   └── cli/
│
├── python/
│   └── quantaradar/
│       ├── ml/
│       ├── anomaly/
│       ├── optimization/
│       ├── validation/
│       ├── research/
│       ├── experiments/
│       ├── visualization/
│       └── registry/
│
├── data/
├── datasets/
├── experiments/
├── models/
├── strategies/
├── reports/
├── configs/
├── dashboards/
└── docs/
```

---

# 16. The difference between today's QuantRadar and the combined vision

This is probably the clearest way to think about it:

### Today's repository

```text
Market data
   ↓
Features
   ↓
Screeners
   ↓
Basic research
   ↓
Backtest
   ↓
Basic robustness
   ↓
Paper execution
```

### The QuantRadar we are actually designing

```text
WORLD
 ↓
EVERY AVAILABLE MARKET OBSERVATION
 ↓
IMMUTABLE DATA PLATFORM
 ↓
REAL-TIME + HISTORICAL FEATURE STORE
 ↓
MULTI-TIMEFRAME MARKET INTELLIGENCE
 ↓
REGIME / FACTOR / MICROSTRUCTURE ANALYSIS
 ↓
CROSS-SECTIONAL OPPORTUNITY DISCOVERY
 ↓
AUTOMATED HYPOTHESIS GENERATION
 ↓
AUTOMATED STRATEGY GENERATION
 ↓
MASSIVE RESEARCH / BACKTEST MATRIX
 ↓
WALK-FORWARD OOS
 ↓
ANTI-OVERFITTING
 ↓
ROBUSTNESS
 ↓
CHAMPION / CHALLENGER
 ↓
PORTFOLIO OPTIMIZATION
 ↓
RISK ENGINE
 ↓
PAPER EXECUTION
 ↓
LIVE MONITORING
 ↓
STRATEGY DECAY DETECTION
 ↓
RESEARCH MEMORY
 ↓
NEXT GENERATION OF STRATEGIES
```

That makes QuantRadar less like **Freqtrade with extra indicators** and more like a **small automated quantitative research firm in software**.

The current repository's stated architecture already points in that direction: Rust for deterministic production components, Python for ML/statistical research, explicit validation contracts, and a deliberately isolated future execution boundary.

## The ultimate objective

The final product I'd define from both chats is:

> **A continuously operating autonomous quantitative research and market-intelligence platform that searches the entire tradable universe for statistically interesting opportunities, explains the evidence, generates competing strategies, rigorously validates them against realistic market conditions, controls portfolio risk, paper-trades the survivors, and continually learns which hypotheses are worth pursuing next.**

That is the **actual QuantRadar project** implied by the two conversations—not merely the current repository.

And there is a very natural next step: turn this combined description into a **complete QuantRadar system specification / master architecture**, with every subsystem, crate, service, database, event type, data schema, research workflow, CLI command, and implementation phase defined.


---
# ROOT AUDIT (project root vs expected tree / PLAN.md) — updated
- Actual crates present (not in tree): archives, backtest_engine, dataset? no — 30+ sub-crates by PLAN section (feature-store, pipeline, multi_exchange, etc.).
- Expected crates MISSING: types, exchange-coinbase, exchange-binance, storage, replay, data-quality, orderbook, microstructure, factors, regime (as crate), screeners, signals, strategies, validation, optimization, portfolio, risk, paper, reporting, monitoring.
- Present that match: core, ingestion, cli, backtest, research.
- Root extras (not in tree): .agents, .codegraph, .omo, .opencode, .pi, .slim, plans/, target/, archives/, AGENTS.md, ARCHITECTURE.md.
- python/quantaradar/ has feature_store (implemented #23); missing tree dirs ml/anomaly/optimization/visualization/registry (registry exists as crate).
- PLAN sections with concrete crates: #10 exec/live-exec, #11 paper-trading, #14 registry, #23 feature-store, #24 pipeline, #19 multi_exchange, #20 derivatives, #21 event_bus, #31 freqtrade_integration, #25 signal-ensemble, #26 expected-return, #17 universe/universe_history, #28 scheduler, #29 dashboard, #15 model_registry.
- Update: mark tree-canonical names (types, exchanges, storage, replay, data-quality) as Design / not yet built; existing sub-crate names aligned with PLAN section IDs.

---
# ROOT & PLAN UPDATE (post-stub implementation)
Sections updated from Design → Stub: #1, #2, #3, #4, #5, #6, #7, #27, #28, #30, #32, #33, #34, #35.
Remaining Design (needs full spec/engine): #29 Monitoring/observability (stub present but no real metrics pipeline), #1 persistent storage (stub only, no Parquet/Arrow persistence), #2 WebSocket (stub adapter, no feed manager).
New crates created (11): types, exchange-coinbase, exchange-binance, data-quality, orderbook, factors, signals, strategies, optimization, portfolio, risk, paper.
PLAN.md status table and implementation paragraph updated accordingly.
