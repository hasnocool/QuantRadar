//! Risk limits stub; VaR/CVaR deferred (#12).
pub fn max_loss() -> f64 { 0.05 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn max_loss_positive() { assert!(max_loss() > 0.0); }
}
