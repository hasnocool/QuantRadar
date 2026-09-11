# CRATE_CANONICAL_experiment.md — quantaradar-experiment

## quantaradar-experiment

### Purpose
Shared crate `quantaradar-experiment` in the QuantRadar workspace (`crates/shared/experiment`).
# crates/shared/experiment/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/experiment/codemap.md)
```text
# crates/shared/experiment/codemap

## Responsibility
Experiment registry and lineage tracking.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → experiment computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Experiment (lib.rs)`
- `struct ExperimentConfig (lib.rs)`
- `struct RiskParams (lib.rs)`
- `struct DataConfig (lib.rs)`
- `enum ExperimentStatus (lib.rs)`
- `struct ExperimentMetrics (lib.rs)`
- `struct LineageRecord (lib.rs)`
- `struct ParameterChange (lib.rs)`
- `struct PromotionRule (lib.rs)`
- `struct ExperimentRegistry (lib.rs)`
- `fn new (lib.rs)`
- `fn create (lib.rs)`
- `fn update_metrics (lib.rs)`
- `fn evaluate_promotion (lib.rs)`
- `fn promote (lib.rs)`
- `fn retire (lib.rs)`
- `fn get (lib.rs)`
- `fn list (lib.rs)`
- `fn promoted (lib.rs)`
- `fn retired (lib.rs)`
- `struct PromotionDecision (lib.rs)`
- `struct MonitoringSnapshot (lib.rs)`
- `fn snapshot (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar experiment registry and lineage tracking
[package]
name = "quantaradar-experiment"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
chrono.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
anyhow.workspace = true
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/experiment` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-experiment` / `cargo doc -p quantaradar-experiment --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-experiment` passes
- [ ] `cargo doc -p quantaradar-experiment` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
