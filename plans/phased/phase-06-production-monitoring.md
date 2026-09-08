# Phase 6 — Experiment Registry, Monitoring & Production Pipeline

Working version of experiment tracking, feature/data lineage, monitoring, and autonomous research loop.

## 6.1 Experiment Registry (minimum)
Every experiment produces:
```text
experiment_id, strategy_id, git_commit, dataset_id, universe_id,
config_id, feature_version, model_version, timestamp, random_seed,
training_period, validation_period, test_period, metrics,
cost_assumptions, artifacts, promotion_decision
```
Failed experiments are retained (not deleted) with `FAILED / REGIME_DEPENDENT` tags.

## 6.2 Feature / Data Lineage
Every feature knows:
```text
feature_id, formula, inputs, lookback, minimum_history,
availability_delay, version
```
Mechanical prevention: `future_return_24h` (available at `t+24h`) is blocked from entering feature matrix at `t`.

## 6.3 Monitoring / Observability (minimum metrics)
```text
feed_latency, dropped_messages, data_gaps, feature_latency,
signal_latency, orders, fills, slippage, PnL, drawdown,
model_drift, feature_drift, strategy_decay
```
Alerts on: feed gap > 5s, feature latency > 500ms, drawdown > max_limit, strategy decay > 20%.

## 6.4 Autonomous Research Loop (minimum working cycle)
```text
discover markets → update datasets → calculate features →
run screeners → generate hypotheses → backtest →
walk-forward → robustness → promote challengers →
paper trade → monitor → retire / promote
```
This is the core "nervous system" — runs continuously, not on-demand.

## 6.5 Dashboard (minimum)
- Market Radar: symbol, score, regime, momentum, liquidity, order flow, volatility, events.
- Strategy Lab: strategy, return, Sharpe, Sortino, max DD, PF, turnover, OOS, robustness, status.
- Portfolio: equity, PnL, risk, heat, positions, clusters, exposure, drawdown.
- Data Health: feeds, latency, missing data, order-book gaps, provider status.

## 6.6 Testing Scale (minimum)
- Unit tests for every crate.
- Integration tests for ingestion → feature → signal pipeline.
- Property tests: position_size <= risk_limit, portfolio_heat <= max, no future timestamps in features, replay deterministic.
- Golden-data tests: fixed dataset produces fixed metrics.

## Working version complete when:
- [ ] Experiment registry stores all experiments with artifacts.
- [ ] Feature lineage prevents future leakage mechanically.
- [ ] Monitoring reports feed latency, gaps, PnL, drawdown, decay.
- [ ] Autonomous loop runs continuously with promotion/retirement.
- [ ] Dashboard displays market radar, strategy lab, portfolio, data health.
