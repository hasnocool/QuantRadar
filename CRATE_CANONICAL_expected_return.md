# CRATE_CANONICAL_expected_return.md — quantaradar-expected_return

## quantaradar-expected_return

### Purpose
Standalone crate `quantaradar-expected_return` in the QuantRadar workspace (`crates/standalone/expected_return`).
# crates/standalone/expected_return/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/expected_return/codemap.md)
```text
# crates/standalone/expected_return/codemap

## Responsibility
Standalone expected-return computation.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → expected_return computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct ExpectedReturn (lib.rs)`
- `fn compute (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-expected_return"
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
- Workspace member: `crates/standalone/expected_return` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-expected_return` / `cargo doc -p quantaradar-expected_return --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-expected_return` passes
- [ ] `cargo doc -p quantaradar-expected_return` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
