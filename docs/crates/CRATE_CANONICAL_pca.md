# CRATE_CANONICAL_pca.md — quantaradar-pca

## quantaradar-pca

### Purpose
Standalone crate `quantaradar-pca` in the QuantRadar workspace (`crates/standalone/pca`).
# crates/standalone/pca/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/pca/codemap.md)
```text
# crates/standalone/pca/codemap

## Responsibility
Principal component analysis engine.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → pca computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct PcaResult (lib.rs)`
- `fn compute (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-pca"
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
- Workspace member: `crates/standalone/pca` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-pca` / `cargo doc -p quantaradar-pca --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-pca` passes
- [ ] `cargo doc -p quantaradar-pca` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
