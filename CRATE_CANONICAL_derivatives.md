# CRATE_CANONICAL_derivatives.md — quantaradar-derivatives

## quantaradar-derivatives

### Purpose
Standalone crate `quantaradar-derivatives` in the QuantRadar workspace (`crates/standalone/derivatives`).
# crates/standalone/derivatives/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/derivatives/codemap.md)
```text
# crates/standalone/derivatives/codemap

## Responsibility
Derivatives/option analytics.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → derivatives computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct DerivativeContract (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-derivatives"
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
- Workspace member: `crates/standalone/derivatives` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-derivatives` / `cargo doc -p quantaradar-derivatives --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-derivatives` passes
- [ ] `cargo doc -p quantaradar-derivatives` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
