# Phase 2 — Market Intelligence Engine

Working version of feature engineering, regime detection, microstructure, and cross-sectional ranking.

## 2.1 Feature Engine (minimum working set)
struct FeatureRow {
  timestamp: u64,
  symbol: String,
  returns_1h: f64,
  ema_20: f64,
  rsi_14: f64,
  atr_14: f64,
  realized_vol_24h: f64,
  bollinger_width: f64,
  volume_zscore: f64,
  ema_distance: f64,
  breakout_flag: bool,
  new_high_24h: bool,
  new_low_24h: bool,
}

Every feature knows its `lookback`, `minimum_history`, and `available_at` timestamp. No future leakage.

## 2.2 Feature Lineage & Leakage Prevention
Each feature carries lineage metadata:
- feature_id
- formula
- inputs
- lookback
- minimum_history
- availability_delay
- version

Mechanical future-leakage prevention: a feature such as `future_return_24h` is only available at `t+24h` and is blocked at time `t`. No future information may enter a feature value.

## 2.3 Regime Engine (minimum taxonomy)
enum Regime {
  BullTrend, BullHighVol, BullLowVol,
  BearTrend, BearHighVol, BearLowVol,
  SidewaysHighVol, SidewaysLowVol,
  TransitionBull, TransitionBear, Unknown,
}

Output includes `confidence: f64`, `trend_strength: f64`, `volatility_state: String`, `transition_probability: f64`. Multi-timeframe: 5m, 15m, 1h, 4h, 1d, 1w. Statistical models: HMM, Bayesian switching, change-point detection.

## 2.4 Microstructure Analytics
From `OrderBookSnapshot` (bid, ask, bid_depth, ask_depth):
- spread = ask - bid
- depth_imbalance = (bid_depth - ask_depth) / (bid_depth + ask_depth)
- executable_impact_1k, 10k, 100k (walk one side, average fill price)
- liquidity_score = f(spread, depth, impact)

## 2.5 Cross-Sectional Ranking (minimum)
raw features → winsorization (5% / 95%) → z-score normalization → sector neutralization → factor construction → ensemble scoring → regime-conditioned rank → top-N selection

Score = weighted sum of trend, momentum, breakout, liquidity, regime fit. Regime fit penalizes signals that conflict with current regime.

## 2.6 Breadth & Relative Strength
- Positive assets count / total universe.
- EMA participation rate.
- New high / new low counts.
- Relative strength vs benchmark (e.g., BTC/USD as benchmark for altcoins).

## 2.7 Correlation / PCA (minimum)
- Rolling correlation matrix (30-day window).
- First PCA component = market factor.
- Cluster-aware position limits: highly correlated assets share exposure budget.

## 2.8 Feature Store
Timestamp-safe retrieval: `features.as_of(timestamp)` returns the feature snapshot valid at that time, preventing look-ahead bias. Categories: price, volume, technical, microstructure, cross-sectional, regime, derivatives, on-chain, event.

## 2.9 Derivatives Data Layer
Fields: funding rates, open interest, liquidations, basis, futures term structure, mark/index price, long/short ratios, options, implied volatility, skew, volatility surface. Used as signal enhancement, not hard-coded decisions.

## 2.10 Design Goals
- Goal 1: Dynamic asset discovery instead of hard-coded universes.
- Goal 3: No data leakage.
- Goal 5: Regime-conditional research.

## Working version complete when:
- [x] FeatureRow computes deterministically with no future leakage.
- [x] Regime engine outputs confidence + transition probability.
- [x] Microstructure computes spread, imbalance, impact, liquidity score.
- [x] Ranking produces regime-conditioned scores.
- [x] PCA first component explains >30% variance on crypto universe.