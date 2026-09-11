# CRATE_CANONICAL_regime.md — quantaradar-regime

## quantaradar-regime

### Purpose
Shared crate `quantaradar-regime` in the QuantRadar workspace (`crates/shared/regime`).
# crates/regime/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/regime/codemap.md)
```text
# crates/regime/

## Responsibility
Market-state classifier (regime confidence, regime detector, RegimeThresholds).

## Design
Classifier with typed confidence scores; integrates with feature engine outputs.

## Flow
Features → regime classification → confidence → regime-scaled screening/backtest.

## Integration
Used by screeners, portfolio, backtest; depends on core, features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct RegimeThresholds (lib.rs)`
- `enum Confidence (lib.rs)`
- `enum TrendStrength (lib.rs)`
- `enum VolatilityState (lib.rs)`
- `struct RegimeClassification (lib.rs)`
- `fn classify (lib.rs)`
- `fn classify_with_confidence (lib.rs)`
- `fn regime_transition (lib.rs)`
- `fn multi_timeframe_regime (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar market regime classifier
[package]
name = "quantaradar-regime"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
serde.workspace = true
quantaradar-core = { path = "../core" }

[dev-dependencies]
quantaradar-features = { path = "../features" }
chrono = { workspace = true, features = ["serde"] }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `quantaradar-features`, `chrono`

### Integration points
- Workspace member: `crates/shared/regime` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-regime` / `cargo doc -p quantaradar-regime --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-regime` passes
- [ ] `cargo doc -p quantaradar-regime` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
