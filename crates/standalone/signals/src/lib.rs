//! Signal aggregator stub (ensemble covers aggregation in #25).
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalSet;

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn signal_set_default() { assert_eq!(SignalSet, SignalSet); }
}
