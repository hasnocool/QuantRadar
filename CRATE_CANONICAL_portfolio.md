# CRATE_CANONICAL_portfolio.md — portfolio

## portfolio

### Purpose
Standalone crate `portfolio` in the QuantRadar workspace (`crates/standalone/portfolio`).
# portfolio

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/portfolio/codemap.md)
```text
# portfolio

## Responsibility
Portfolio construction stub (optimizer deferred to #12, #29).

## Source Map
- `src/lib.rs`: `Portfolio`, `weights`, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct Portfolio (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
# Portfolio construction stub.
[package]
name = "portfolio"
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
- Workspace member: `crates/standalone/portfolio` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p portfolio` / `cargo doc -p portfolio --no-deps`.

### Verification checklist
- [ ] `cargo check -p portfolio` passes
- [ ] `cargo doc -p portfolio` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
