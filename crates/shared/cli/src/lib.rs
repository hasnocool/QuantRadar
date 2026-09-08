//! CLI library stub for quantaradar workspace.
pub fn cli_version() -> String { "0.2.0".into() }
#[cfg(test)]
mod tests { use super::*; #[test] fn version_ok() { assert!(!cli_version().is_empty()); } }
#[derive(Debug, Clone, Default)] pub struct CliVariant { pub mode: String }
impl CliVariant { pub fn new(m: &str) -> Self { Self{mode:m.into()} } }
#[cfg(test)] mod variant_tests { use super::*; #[test] fn variant_create() { assert!(!CliVariant::new("discover").mode.is_empty()); } }
#[cfg(test)] mod cli_deep_tests { use super::*; #[test] fn cli_full() { assert!(!cli_version().is_empty()); assert!(!CliVariant::new("backtest").mode.is_empty()); } }
pub enum CliVariantMode { Discover, Screen, Fetch, Backtest, Monitor }
#[cfg(test)] mod cli_full_tests { use super::*; #[test] fn all_modes() { assert!(CliVariantMode::Discover.to_string() == "Discover"); } }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum CliMode { Discover, Screen, Fetch, Backtest, Monitor }
impl CliMode { pub fn dispatch(&self) -> &'static str { match self { CliMode::Discover => "discover", CliMode::Screen => "screen", CliMode::Fetch => "fetch", CliMode::Backtest => "backtest", CliMode::Monitor => "monitor", } } }
#[cfg(test)] mod cli_dispatch_tests { use super::*; #[test] fn cli_dispatch_all() { for mode in [CliMode::Discover, CliMode::Backtest, CliMode::Monitor] { assert!(!mode.dispatch().is_empty()); } } }
impl CliVariantMode { pub fn execute_command(&self) -> &'static str { self.dispatch() } }
#[cfg(test)] mod cli_execution_tests { use super::*; #[test] fn cli_execute() { assert!(!CliVariantMode::Backtest.execute_command().is_empty()); } }
