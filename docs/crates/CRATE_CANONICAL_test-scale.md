# CRATE_CANONICAL_test-scale.md — quantaradar-test-scale

## quantaradar-test-scale

### Purpose
Standalone crate `quantaradar-test-scale` in the QuantRadar workspace (`crates/standalone/test-scale`).
# crates/standalone/test-scale/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/test-scale/codemap.md)
```text
# crates/standalone/test-scale/codemap

## Responsibility
Test-scale and load-testing.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → test-scale computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`
- `src/property_tests.rs`

### Public API surface (sampled from source)
- `struct TestScale (lib.rs)`
- `fn new (lib.rs)`
- `fn process (lib.rs)`
- `fn scale_tests (lib.rs)`
- `fn run_parallel (lib.rs)`
- `fn load_benchmark (lib.rs)`
- `fn run_batch (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-test-scale"
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
- Workspace member: `crates/standalone/test-scale` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-test-scale` / `cargo doc -p quantaradar-test-scale --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-test-scale` passes
- [ ] `cargo doc -p quantaradar-test-scale` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
