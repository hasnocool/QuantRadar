#universe_history

## Responsibility

Module functionality to be documented.

## Source Map

- `pub struct UniverseHistory { pub members: Vec<(String,u64)> }`

- `impl UniverseHistory { pub fn new()->Self{Self{members:vec![]}} pub fn add(&mut self,symbol:String,ts:u64){self.members.push((symbol,ts));} pub fn at_time(&self,t:u64)->Vec<String>{self.members.iter().filter(|(_,ts)|*ts<=t).map(|(s,_)|s.clone()).collect()} }`

- `mod verify_output {`

- `let manifest = env!("CARGO_MANIFEST_DIR");`

- `let pkg = env!("CARGO_PKG_NAME");`

## Dependencies

- `anyhow.workspace`

- `serde.workspace`

- `serde_json.workspace`

- `chrono.workspace`

## Tests

- `cargo test -p <name>`

## Integration

- Part of the `universe_history` crate in the QuantRadar workspace
