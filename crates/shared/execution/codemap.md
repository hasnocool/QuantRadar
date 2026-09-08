# crates/execution/

## Responsibility
Risk controls + paper execution (execution orders, paper-state machine, reconciliation).

## Design
Paper-state machine with risk limits; order execution tracking.

## Flow
Signals → execution orders → paper state → reconciliation.

## Integration
Depends on portfolio, backtest; produces paper-trading state.
