use quantaradar_expected_return::*;
use std::collections::HashMap;
#[test] fn expected_default() { let mut model = ReturnModel { risk_free_rate: 0.05, factor_returns: HashMap::new(), factor_covariance: HashMap::new(), transaction_cost_rate: 0.03, slippage_rate: 0.01 }; register_factor_return("momentum".into(), 0.15, &mut model); let features = HashMap::from([("momentum".into(), 0.8)]); let er = estimate_expected_return(&features, &model, "AAPL"); assert!(er.expected_return > 0.05); }
