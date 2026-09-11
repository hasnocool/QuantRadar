#orderbook

## Responsibility

L2 order-book model (delta/rebuild deferred to #3).

## Source Map

- `use serde::{Deserialize, Serialize};`

- `pub struct L2Depth {`

- `pub price: f64,`

- `pub qty: f64,`

- `}`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `orderbook` crate in the QuantRadar workspace
