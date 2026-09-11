# CRATE_CANONICAL_domain-model.md — quantaradar-domain-model

## quantaradar-domain-model

### Purpose
Shared crate `quantaradar-domain-model` in the QuantRadar workspace (`crates/shared/domain-model`).
# crates/shared/domain-model/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/domain-model/codemap.md)
```text
# crates/shared/domain-model/codemap

## Responsibility
Domain entities (orders, trades, positions).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → domain-model computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Trade (lib.rs)`
- `struct Position (lib.rs)`
- `fn new (lib.rs)`
- `fn direction_str (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-domain-model"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
quantaradar-core = { path = "../core" }
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
chrono.workspace = true
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/domain-model` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-domain-model` / `cargo doc -p quantaradar-domain-model --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-domain-model` passes
- [ ] `cargo doc -p quantaradar-domain-model` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
