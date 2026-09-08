//! Portfolio optimization and basic position-sizing utilities.
/// Basic equal-weight sizing stub; full mean-variance / CVaR to follow.
pub fn basic_size() -> f64 { 1.0 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_size_positive() { assert!(basic_size() > 0.0); }
}
