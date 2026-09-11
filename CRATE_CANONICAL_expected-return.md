# CRATE_CANONICAL_expected-return.md — quantaradar-expected-return

## quantaradar-expected-return

### Purpose
Shared crate `quantaradar-expected-return` in the QuantRadar workspace (`crates/shared/expected-return`).
# crates/shared/expected-return/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/expected-return/codemap.md)
```text
# crates/shared/expected-return/codemap

## Responsibility
Expected-return modeling and promotion scoring.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → expected-return computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct ExpectedReturn (lib.rs)`
- `struct ReturnModel (lib.rs)`
- `fn estimate_expected_return (lib.rs)`
- `fn register_factor_return (lib.rs)`
- `fn set_factor_covariance (lib.rs)`
- `fn expected_return_with_risk_adjustment (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-expected-return"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
quantaradar-feature-store = { path = "../../standalone/feature-store" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-feature-store`

### Integration points
- Workspace member: `crates/shared/expected-return` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-expected-return` / `cargo doc -p quantaradar-expected-return --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-expected-return` passes
- [ ] `cargo doc -p quantaradar-expected-return` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
