# CRATE_CANONICAL_reporting.md — quantaradar-reporting

## quantaradar-reporting

### Purpose
Shared crate `quantaradar-reporting` in the QuantRadar workspace (`crates/shared/reporting`).
# crates/reporting/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/reporting/codemap.md)
```text
# crates/reporting/

## Responsibility
Machine-readable reports (backtest.json, markets.json, scan.json, screen.json).

## Design
JSON schema aligned with backtest/screen outputs.

## Flow
Backtest results → machine-readable reports → storage/review.

## Integration
Used by CLI, dashboard; consumes backtest/screen outputs.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `fn write_json (lib.rs)`
- `fn write_yaml (lib.rs)`
- `struct ExperimentReport (lib.rs)`
- `struct ExperimentConfig (lib.rs)`
- `struct RiskParams (lib.rs)`
- `struct DataConfig (lib.rs)`
- `struct ExperimentMetrics (lib.rs)`
- `struct LineageRecord (lib.rs)`
- `struct ParameterChange (lib.rs)`
- `struct TradeRecord (lib.rs)`
- `struct EquityPoint (lib.rs)`
- `struct SignalRecord (lib.rs)`
- `struct FeatureManifest (lib.rs)`
- `fn generate_experiment_artifacts (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar report serialization
[package]
name = "quantaradar-reporting"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
quantaradar-core = { path = "../core" }
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
serde_yaml = "0.9"
chrono.workspace = true
uuid = { version = "1", features = ["v4", "serde"] }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`, `serde_yaml`, `uuid`

### Integration points
- Workspace member: `crates/shared/reporting` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-reporting` / `cargo doc -p quantaradar-reporting --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-reporting` passes
- [ ] `cargo doc -p quantaradar-reporting` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
