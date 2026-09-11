# CRATE_CANONICAL_exchange-coinbase.md — exchange-coinbase

## exchange-coinbase

### Purpose
Standalone crate `exchange-coinbase` in the QuantRadar workspace (`crates/standalone/exchange-coinbase`).
# exchange-coinbase

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/exchange-coinbase/codemap.md)
```text
# exchange-coinbase

## Responsibility
Coinbase REST/WebSocket adapter stub, expanded when multi-exchange grows.

## Source Map
- `src/lib.rs`: `CoinbaseAdapter`, tests
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct CoinbaseAdapter (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
# Coinbase REST/WebSocket ingestion adapter.
[package]
name = "exchange-coinbase"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
serde.workspace = true
reqwest.workspace = true
tokio.workspace = true
```

### Dependencies (manifest keys)
`name`

### Integration points
- Workspace member: `crates/standalone/exchange-coinbase` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p exchange-coinbase` / `cargo doc -p exchange-coinbase --no-deps`.

### Verification checklist
- [ ] `cargo check -p exchange-coinbase` passes
- [ ] `cargo doc -p exchange-coinbase` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
