# CRATE_CANONICAL_universe_history.md — quantaradar-universe_history

## quantaradar-universe_history

### Purpose
Standalone crate `quantaradar-universe_history` in the QuantRadar workspace (`crates/standalone/universe_history`).
# crates/standalone/universe_history/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/universe_history/codemap.md)
```text
# crates/standalone/universe_history/codemap

## Responsibility
Universe history tracking.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → universe_history computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct UniverseHistory (lib.rs)`
- `fn new (lib.rs)`
- `fn add (lib.rs)`
- `fn at_time (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-universe_history"
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
- Workspace member: `crates/standalone/universe_history` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-universe_history` / `cargo doc -p quantaradar-universe_history --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-universe_history` passes
- [ ] `cargo doc -p quantaradar-universe_history` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
