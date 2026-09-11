# CRATE_CANONICAL_strategies.md — strategies

## strategies

### Purpose
Standalone crate `strategies` in the QuantRadar workspace (`crates/standalone/strategies`).
# strategies

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/strategies/codemap.md)
```text
# strategies
## Responsibility
Stub aligned with full domain in core / strategy_dsl / ensemble.
## Source Map
- src/lib.rs: definitions, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `enum StrategyType (lib.rs)`

### Cargo manifest (abridged)
```toml
# Stub crate
[package]
name = "strategies"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
serde.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/strategies` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p strategies` / `cargo doc -p strategies --no-deps`.

### Verification checklist
- [ ] `cargo check -p strategies` passes
- [ ] `cargo doc -p strategies` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
