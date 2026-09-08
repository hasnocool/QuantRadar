# QuantRadar Validation Protocol

A strategy is a hypothesis until it survives out-of-sample evaluation.

## Required evidence

- Exact strategy/config version.
- Immutable data snapshot and universe definition.
- Fees, spread and slippage assumptions.
- Train/test separation.
- Expanding walk-forward folds.
- Aggregate and per-regime performance.
- OOS trade count and turnover.
- Robustness to reasonable cost perturbations.
- Leakage review.

## Promotion gate

A challenger may replace a champion only when OOS expectancy is positive, drawdown remains acceptable, performance improves across multiple folds rather than a single lucky period, OOS trade count is adequate, cost sensitivity is acceptable, and there is no obvious data leakage or concentration failure.

Failed experiments are retained as research data.
