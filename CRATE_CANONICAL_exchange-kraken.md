# CRATE_CANONICAL_exchange-kraken.md — quantaradar-exchange-kraken

## quantaradar-exchange-kraken

### Purpose
Shared crate `quantaradar-exchange-kraken` in the QuantRadar workspace (`crates/shared/exchange-kraken`).
# crates/exchange-kraken/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/exchange-kraken/codemap.md)
```text
# crates/exchange-kraken/

## Responsibility
Kraken REST + WebSocket ingestion; normalization to core contracts.

## Design
REST polling + WebSocket ingestion; normalization layer.

## Flow
Kraken API → ingestion/normalization → core contracts.

## Integration
Produces market data for core; depends on core.
```

### Source layout (`src/`)
- `src/lib.rs`
- `src/sequence.rs`
- `src/ws.rs`

### Public API surface (sampled from source)
- `mod ws (lib.rs)`
- `mod sequence (lib.rs)`
- `struct KrakenClient (lib.rs)`
- `struct KrakenPair (lib.rs)`
- `enum ValidationError (sequence.rs)`
- `struct SequenceValidator (sequence.rs)`
- `fn new (sequence.rs)`
- `fn validate (sequence.rs)`
- `fn metrics (sequence.rs)`
- `enum MarketEvent (ws.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar Kraken public market-data adapter
[package]
name = "quantaradar-exchange-kraken"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[dependencies]
anyhow.workspace = true
chrono.workspace = true
futures.workspace = true
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
tokio-tungstenite = { version = "0.24", features = ["rustls-tls-webpki-roots"] }
quantaradar-core = { path = "../core" }
urlencoding = "2"
```

### Dependencies (manifest keys)
`name`, `tokio-tungstenite`, `quantaradar-core`, `urlencoding`

### Integration points
- Workspace member: `crates/shared/exchange-kraken` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar-exchange-kraken` / `cargo doc -p quantaradar-exchange-kraken --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar-exchange-kraken` passes
- [ ] `cargo doc -p quantaradar-exchange-kraken` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
