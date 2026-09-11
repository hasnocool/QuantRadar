# CRATE_CANONICAL_regime-detector.md — quantaradar-regime-detector

## quantaradar-regime-detector

### Purpose
Standalone crate `quantaradar-regime-detector` in the QuantRadar workspace (`crates/standalone/regime-detector`).
# crates/standalone/regime-detector/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/regime-detector/codemap.md)
```text
# crates/standalone/regime-detector/codemap

## Responsibility
Regime detection classifier.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → regime-detector computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `enum Regime (lib.rs)`
- `struct RegimeDetector (lib.rs)`
- `fn new (lib.rs)`
- `fn detect (lib.rs)`
- `fn process (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-regime-detector"
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
- Workspace member: `crates/standalone/regime-detector` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-regime-detector` / `cargo doc -p quantaradar-regime-detector --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-regime-detector` passes
- [ ] `cargo doc -p quantaradar-regime-detector` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
