# crates/features/
## Responsibility
Deterministic technical features (FeatureRow, feature store, feature-engine pipeline).
## Design
Polars/duckdb-backed; deterministic calculations; no ML within this crate.
## Flow
Market data → feature computation → feature store → screening/backtest.
## Integration
Used by regime, screening, backtest crates; depends on core.
