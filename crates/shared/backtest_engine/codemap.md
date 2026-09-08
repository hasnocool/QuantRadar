# crates/shared/backtest_engine/codemap

## Responsibility
Backtest engine execution layer (cost-aware simulation).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → backtest_engine computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
