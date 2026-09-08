# crates/reporting/

## Responsibility
Machine-readable reports (backtest.json, markets.json, scan.json, screen.json).

## Design
JSON schema aligned with backtest/screen outputs.

## Flow
Backtest results → machine-readable reports → storage/review.

## Integration
Used by CLI, dashboard; consumes backtest/screen outputs.
