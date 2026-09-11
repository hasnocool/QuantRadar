# CRATE_CANONICAL_exchange-binance.md — exchange-binance

## exchange-binance

### Purpose
Standalone crate `exchange-binance` in the QuantRadar workspace (`crates/standalone/exchange-binance`).
# exchange-binance

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/standalone/exchange-binance/codemap.md)
```text
# exchange-binance

## Responsibility
Binance REST + websocket ingestion adapter.

## Source Map
- `src/lib.rs`: `BinanceAdapter`, tests

## Dependencies
- `anyhow`, `serde`, `reqwest`, `tokio`
```

### Source layout (`src/`)
- `src/lib.rs`

### Public API surface (sampled from source)
- `struct BinanceAdapter (lib.rs)`
- `fn new (lib.rs)`

### Cargo manifest (abridged)
```toml
# Binance REST/WebSocket ingestion.
[package]
name = "exchange-binance"
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
- Workspace member: `crates/standalone/exchange-binance` (see root `Cargo.toml` members list).
- Shared vs standalone: `standalone` — domain/application-level crate built on shared infrastructure.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p exchange-binance` / `cargo doc -p exchange-binance --no-deps`.

### Verification checklist
- [ ] `cargo check -p exchange-binance` passes
- [ ] `cargo doc -p exchange-binance` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
