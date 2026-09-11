# CRATE_CANONICAL_portfolio_risk.md — quantaradar-portfolio_risk

## quantaradar-portfolio_risk

### Purpose
Standalone crate `quantaradar-portfolio_risk` in the QuantRadar workspace (`crates/standalone/portfolio_risk`).
# crates/standalone/portfolio_risk/codemap

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/portfolio_risk/codemap.md)
```text
# crates/standalone/portfolio_risk/codemap

## Responsibility
Portfolio risk and optimization.

## Design
Strict Rust types, native computation, minimal speculation.

## Flow
Upstream core/features → portfolio_risk computation → downstream consumers.

## Integration
Used by workspace pipeline; depends on core/features.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct PortfolioRisk (lib.rs)`
- `fn new (lib.rs)`
- `fn scale (lib.rs)`
- `fn limit_exceeded (lib.rs)`
- `struct VaRResult (lib.rs)`
- `fn compute (lib.rs)`

### Cargo manifest (abridged)
```toml
[package]
name = "quantaradar-portfolio_risk"
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
- Workspace member: `crates/standalone/portfolio_risk` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-portfolio_risk` / `cargo doc -p quantaradar-portfolio_risk --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-portfolio_risk` passes
- [ ] `cargo doc -p quantaradar-portfolio_risk` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
