# crates/core/

## Responsibility
Shared domain models: Direction, OrderSide, EventKind, timestamp normalization, and immutable market-data contracts used by ingestion, features, regime, screening, and backtest crates.

## Design
Strict Rust types (`thiserror`/`anyhow`); serde for serialization; no business logic — pure data/contracts. Key enums: Direction, CliMode, RegimeState.

## Flow
Exchange ingestion → core contracts → feature/regime/screening consumers → backtest.

## Integration
Used by `quantaradar-core` (lib) and all downstream crates via `crates/shared/core/src/lib.rs`.
