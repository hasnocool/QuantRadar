# CRATE_CANONICAL_freqtrade_integration.md — quantaradar-freqtrade_integration

## quantaradar-freqtrade_integration

### Purpose
Standalone crate `quantaradar-freqtrade_integration` in the QuantRadar workspace (`crates/standalone/freqtrade_integration`).
# crates/standalone/freqtrade_integration/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/freqtrade_integration/codemap.md)
```text
# crates/standalone/freqtrade_integration/codemap

## Responsibility
Freqtrade integration adapter.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → freqtrade_integration computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct FreqAdapter (lib.rs)`
- `fn connect (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-freqtrade_integration"
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
- Workspace member: `crates/standalone/freqtrade_integration` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-freqtrade_integration` / `cargo doc -p quantaradar-freqtrade_integration --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-freqtrade_integration` passes
- [ ] `cargo doc -p quantaradar-freqtrade_integration` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
