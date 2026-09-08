# crates/cli/

## Responsibility
CLI entry `quantaradar` (discover, screen, fetch, backtest, monitor) dispatching to core and strategy modules.

## Design
Clap-based argument parsing; `CliMode` and `CliVariantMode` dispatch; minimal logic — delegates to library crates.

## Flow
User command → cli parse → `CliMode::dispatch()` → library call → output.

## Integration
Binary target `quantaradar`; library target `quantaradar` (cli); depends on core, features, regime, screeners.
