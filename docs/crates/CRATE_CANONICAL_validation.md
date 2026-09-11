# CRATE_CANONICAL_validation.md — quantaradar-validation

## quantaradar-validation

### Purpose
Standalone crate `quantaradar-validation` in the QuantRadar workspace (`crates/standalone/validation`).
# crates/standalone/validation/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/validation/codemap.md)
```text
# crates/standalone/validation/codemap

## Responsibility
Validation and schema checks.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → validation computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct ValidationEngine (lib.rs)`
- `fn new (lib.rs)`
- `fn validate (lib.rs)`
- `fn walk_forward (lib.rs)`
- `struct ValidationResult (lib.rs)`
- `fn check (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-validation"
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
- Workspace member: `crates/standalone/validation` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-validation` / `cargo doc -p quantaradar-validation --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-validation` passes
- [ ] `cargo doc -p quantaradar-validation` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
