# CRATE_CANONICAL_cli.md — quantaradar

## quantaradar

### Purpose
Shared crate `quantaradar` in the QuantRadar workspace (`crates/shared/cli`).
# crates/cli/

### Version / License / Edition
- version.workspace = true (workspace 0.2.0), edition.workspace = true (2024), license.workspace = true (MIT), rust-version 1.85

### Full codemap (from crates/shared/cli/codemap.md)
```text
# crates/cli/

## Responsibility
CLI entry `quantaradar` (discover, screen, fetch, backtest, monitor) dispatching to core and strategy modules.

## Design
Clap-based argument parsing; `CliMode` and `CliVariantMode` dispatch; minimal logic — delegates to library crates.

## Flow
User command → cli parse → `CliMode::dispatch()` → library call → output.

## Integration
Binary target `quantaradar`; library target `quantaradar` (cli); depends on core, features, regime, screeners.
```

### Source layout (`src/`)
- `src/lib.rs`
- `src/main.rs`
- `src/main_clean.rs`
- `src/main_implemented.rs`
- `src/main_minimal.rs`
- `src/main_new.rs`

### Public API surface (sampled from source)
- `fn cli_version (lib.rs)`
- `struct CliVariant (lib.rs)`
- `fn new (lib.rs)`
- `enum CliVariantMode (lib.rs)`
- `enum CliMode (lib.rs)`
- `fn dispatch (lib.rs)`
- `fn execute_command (lib.rs)`

### Cargo manifest (abridged)
```toml
# QuantRadar command-line application
[package]
name = "quantaradar"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
[[bin]]
name = "quantaradar"
path = "src/main.rs"
[dependencies]
anyhow.workspace = true
chrono.workspace = true
clap = { workspace = true, features = ["derive"] }
futures.workspace = true
serde_json.workspace = true
tokio = { workspace = true, features = ["full"] }
tracing.workspace = true
tracing-subscriber.workspace = true
sha2.workspace = true
quantaradar-core = { path = "../core" }
quantaradar-exchange-kraken = { path = "../exchange-kraken" }
quantaradar-features = { path = "../features" }
quantaradar-regime = { path = "../regime" }
quantaradar-screeners = { path = "../screeners" }
quantaradar-backtest = { path = "../backtest" }
quantaradar-reporting = { path = "../reporting" }
quantaradar-storage = { path = "../storage" }
quantaradar-data-model = { path = "../data-model" }
quantaradar-replay = { path = "../../standalone/replay" }
```

### Dependencies (manifest keys)
`name`, `name`, `path`, `clap`, `tokio`, `quantaradar-core`, `quantaradar-exchange-kraken`, `quantaradar-features`, `quantaradar-regime`, `quantaradar-screeners`, `quantaradar-backtest`, `quantaradar-reporting`, `quantaradar-storage`, `quantaradar-data-model`, `quantaradar-replay`

### Integration points
- Workspace member: `crates/shared/cli` (see root `Cargo.toml` members list).
- Shared vs standalone: `shared` — core infrastructure consumed by multiple crates.
- See `ARCHITECTURE_CANONICAL.md` data-flow section and root `codemap.md` for cross-crate wiring.
- Verify: `cargo check -p quantaradar` / `cargo doc -p quantaradar --no-deps`.

### Verification checklist
- [ ] `cargo check -p quantaradar` passes
- [ ] `cargo doc -p quantaradar` renders public API
- [ ] codemap.md matches `src/` contents
- [ ] No unused-dependency warnings (`cargo machete` / `cargo udeps` optional)
