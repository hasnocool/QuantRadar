# CRATE_CANONICAL_microstructure.md — quantaradar-microstructure

## quantaradar-microstructure

### Purpose
Shared crate `quantaradar-microstructure` in the QuantRadar workspace (`crates/shared/microstructure`).
# crates/microstructure/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/microstructure/codemap.md)
```text
# crates/microstructure/

## Responsibility
Order-book/trade-flow/liquidity analytics (liquidity metrics, order-book models).

## Design
Order-book event processing; liquidity feature computation.

## Flow
Order-book events → liquidity metrics → feature augmentation.

## Integration
Used by feature engine and regime; depends on core.
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct MicrostructureFeatures (lib.rs)`
- `fn analyze (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar microstructure crate manifest
[package]
name = "quantaradar-microstructure"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true
[dependencies]
quantaradar-core = { path = "../core" }
```

### Dependencies (manifest keys)
`name`, `quantaradar-core`

### Integration points
- Workspace member: `crates/shared/microstructure` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-microstructure` / `cargo doc -p quantaradar-microstructure --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-microstructure` passes
- [ ] `cargo doc -p quantaradar-microstructure` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
