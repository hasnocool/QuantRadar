# CRATE_CANONICAL_hold-period.md — quantaradar-hold-period

## quantaradar-hold-period

### Purpose
Standalone crate `quantaradar-hold-period` in the QuantRadar workspace (`crates/standalone/hold-period`).
# crates/standalone/hold-period/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/hold-period/codemap.md)
```text
# crates/standalone/hold-period/codemap

## Responsibility
Hold-period and position-duration logic.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → hold-period computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct HoldPeriod (lib.rs)`
- `fn new (lib.rs)`
- `fn process (lib.rs)`
- `fn estimate (lib.rs)`
- `fn optimize (lib.rs)`
- `fn predict_horizon (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-hold-period"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/hold-period` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-hold-period` / `cargo doc -p quantaradar-hold-period --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-hold-period` passes
- [ ] `cargo doc -p quantaradar-hold-period` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
