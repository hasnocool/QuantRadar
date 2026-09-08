# Phase 6 — Experiment Registry, Monitoring & Production Pipeline

Working version of experiment tracking, feature/data lineage, monitoring, and autonomous research loop.

## 6.1 Experiment Registry (minimum)
Every experiment produces:
experiment_id, strategy_id, git_commit, dataset_id, universe_id,
config_id, feature_version, model_version, timestamp, random_seed,
training_period, validation_period, test_period, metrics,
cost_assumptions, artifacts, promotion_decision

Failed experiments are retained (not deleted) with `FAILED / REGIME_DEPENDENT` tags.

## 6.2 Reproducibility Contract
Every experiment output records the 9 reproducibility fields:
- code_commit
- config_hash
- dataset_id
- feature_versions
- strategy_version
- model_version
- random_seed
- execution_model_version

## 6.3 Feature / Data Lineage
Every feature knows:
feature_id, formula, inputs, lookback, minimum_history,
availability_delay, version

Mechanical prevention: `future_return_24h` (available at `t+24h`) is blocked from entering feature matrix at `t`.

## 6.4 Monitoring / Observability (minimum metrics)
feed_latency, dropped_messages, data_gaps, feature_latency,
signal_latency, orders, fills, slippage, PnL, drawdown,
model_drift, feature_drift, strategy_decay

Alerts on: feed gap > 5s, feature latency > 500ms, drawdown > max_limit, strategy decay > 20%.

## 6.5 Autonomous Research Loop (minimum working cycle)
discover markets → update datasets → calculate features →
run screeners → generate hypotheses → backtest →
walk-forward → robustness → promote challengers →
paper trade → monitor → retire / promote

This is the core "nervous system" — runs continuously, not on-demand.

## 6.6 Dashboard (minimum)
- Market Radar: symbol, score, regime, momentum, liquidity, order flow, volatility, events.
- Strategy Lab: strategy, return, Sharpe, Sortino, max DD, PF, turnover, OOS, robustness, status.
- Portfolio: equity, PnL, risk, heat, positions, clusters, exposure, drawdown.
- Data Health: feeds, latency, missing data, order-book gaps, provider status.

## 6.7 Testing Scale (minimum)
- Unit tests for every crate.
- Integration tests for ingestion → feature → signal pipeline.
- Property tests: position_size <= risk_limit, portfolio_heat <= max, no future timestamps in features, replay deterministic.
- Golden-data tests: fixed dataset produces fixed metrics.

## 6.8 Design Goals
- Goal 1: Dynamic asset discovery instead of hard-coded universes.
- Goal 3: No data leakage.
- Goal 6: Failed experiments retained for future learning.
- Goal 8: Live execution isolated from research until promotion gates pass.

## Working version complete when:
- [x] Experiment registry stores all experiments with artifacts.
- [x] Feature lineage prevents future leakage mechanically.
- [x] Monitoring reports feed latency, gaps, PnL, drawdown, decay.
- [x] Autonomous loop runs continuously with promotion/retirement.
- [x] Dashboard displays market radar, strategy lab, portfolio, data health.