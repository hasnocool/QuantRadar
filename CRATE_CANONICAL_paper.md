# CRATE_CANONICAL_paper.md — paper

## paper

### Purpose
Standalone crate `paper` in the QuantRadar workspace (`crates/standalone/paper`).
# paper

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/paper/codemap.md)
```text
# paper

## Responsibility
Paper-trading stub aligned with `paper-trading`; consolidate when permitted.

## Source Map
- `src/lib.rs`: `PaperTrader`, `submit()`, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct PaperTrader (lib.rs)`
- `fn new (lib.rs)`
- `fn submit (lib.rs)`

### Cargo manifest (abridged)
```toml
# Paper execution stub aligned with paper-trading.
[package]
name = "paper"
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
- Workspace member: `crates/standalone/paper` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p paper` / `cargo doc -p paper --no-deps`.

### Verification checklist
- [ ] `cargo check -p paper` passes
- [ ] `cargo doc -p paper` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
