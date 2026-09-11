#microstructure

## Responsibility

Market microstructure analysis.

## Source Map

- `// QuantRadar microstructure analytics: order-book/trade-flow models and executable-liquidity scoring.`

- `use quantaradar_core::{OrderBookSnapshot, OrderSide, TradeTick};`

- `pub struct MicrostructureFeatures {`

- `pub spread_bps: f64,`

- `pub bid_depth_usd: f64,`

## Dependencies

- `quantaradar-core`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `microstructure` crate in the QuantRadar workspace
