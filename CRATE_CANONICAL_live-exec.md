# CRATE_CANONICAL_live-exec.md — quantaradar-live-exec

## quantaradar-live-exec

### Purpose
Standalone crate `quantaradar-live-exec` in the QuantRadar workspace (`crates/standalone/live-exec`).
# crates/standalone/live-exec/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/live-exec/codemap.md)
```text
# crates/standalone/live-exec/codemap

## Responsibility
Live execution adapter (disabled, isolated).

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → live-exec computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct LiveExec (lib.rs)`
- `fn start (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-live-exec"
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
- Workspace member: `crates/standalone/live-exec` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-live-exec` / `cargo doc -p quantaradar-live-exec --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-live-exec` passes
- [ ] `cargo doc -p quantaradar-live-exec` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
