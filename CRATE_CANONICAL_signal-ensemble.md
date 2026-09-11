# CRATE_CANONICAL_signal-ensemble.md — quantaradar-signal-ensemble

## quantaradar-signal-ensemble

### Purpose
Shared crate `quantaradar-signal-ensemble` in the QuantRadar workspace (`crates/shared/signal-ensemble`).
# crates/shared/signal-ensemble/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/signal-ensemble/codemap.md)
```text
# crates/shared/signal-ensemble/codemap

## Responsibility
Signal ensemble and composite scoring.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → signal-ensemble computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Signal (lib.rs)`
- `struct EnsembleSignal (lib.rs)`
- `struct ComponentScores (lib.rs)`
- `fn new (lib.rs)`
- `struct SignalEnsembleConfig (lib.rs)`
- `fn should_trade (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-signal-ensemble"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
serde.workspace = true
serde_json.workspace = true
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/signal-ensemble` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-signal-ensemble` / `cargo doc -p quantaradar-signal-ensemble --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-signal-ensemble` passes
- [ ] `cargo doc -p quantaradar-signal-ensemble` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
