# CRATE_CANONICAL_paper-trading.md — quantaradar-paper-trading

## quantaradar-paper-trading

### Purpose
Standalone crate `quantaradar-paper-trading` in the QuantRadar workspace (`crates/standalone/paper-trading`).
# crates/standalone/paper-trading/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/paper-trading/codemap.md)
```text
# crates/standalone/paper-trading/codemap

## Responsibility
Paper-trading state machine.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → paper-trading computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct PaperAccount (lib.rs)`
- `fn new (lib.rs)`
- `fn update_equity (lib.rs)`
- `fn add_fill (lib.rs)`
- `fn add_position (lib.rs)`
- `fn current_pnl (lib.rs)`
- `fn ledger_snapshot (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-paper-trading"
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
- Workspace member: `crates/standalone/paper-trading` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-paper-trading` / `cargo doc -p quantaradar-paper-trading --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-paper-trading` passes
- [ ] `cargo doc -p quantaradar-paper-trading` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
