//! Portfolio construction stub; full optimizer deferred.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Portfolio {
    pub weights: Vec<f64>,
}

impl Portfolio {
    pub fn new() -> Self { Self { weights: vec![] } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn portfolio_default() { assert!(Portfolio::new().weights.is_empty()); }
}
