# CRATE_CANONICAL_lineage.md — quantaradar-lineage

## quantaradar-lineage

### Purpose
Standalone crate `quantaradar-lineage` in the QuantRadar workspace (`crates/standalone/lineage`).
# crates/standalone/lineage/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/lineage/codemap.md)
```text
# crates/standalone/lineage/codemap

## Responsibility
Dataset lineage and provenance.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → lineage computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct LineageRecord (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-lineage"
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
- Workspace member: `crates/standalone/lineage` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-lineage` / `cargo doc -p quantaradar-lineage --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-lineage` passes
- [ ] `cargo doc -p quantaradar-lineage` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
