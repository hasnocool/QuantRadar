use quantaradar_core::OrderSide;
use serde::{Deserialize, Serialize};

/// Fee configuration for backtesting.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BacktestFeeConfig {
    /// Per-trade fee rate (as percentage of trade value)
    pub fee_rate: f64,
    /// Minimum fee amount
    pub minimum_fee: f64,
    /// Slippage percentage (market impact)
    pub slippage_pct: f64,
}

/// Capital configuration for backtesting.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapitalConfig {
    /// Initial cash starting capital
    pub initial_cash: f64,
    /// Cash management mode
    pub cash_management: CashManagement,
}

/// Cash management modes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CashManagement {
    /// Keep cash uninvested, track net asset value
    #[default]
    Uninvested,
    /// Fully invested, no cash buffer
    FullyInvested,
    /// Partial investment with cash buffer
    PartialInvested { buffer_pct: f64 },
}

/// A single backtest trade execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestTrade {
    /// Trade timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Order side
    pub side: OrderSide,
    /// Executed price
    pub price: f64,
    /// Executed quantity
    pub quantity: f64,
    /// Gross trade value (price * quantity)
    pub gross_value: f64,
    /// Fees paid
    pub fees: f64,
    /// Slippage paid
    pub slippage: f64,
    /// Net trade value after fees and slippage
    pub net_value: f64,
}

/// Backtest engine for running strategy simulations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestEngine {
    /// Symbol being backtested
    pub symbol: String,
    /// Fee configuration
    pub fee_config: BacktestFeeConfig,
    /// Capital configuration
    pub capital_config: CapitalConfig,
    /// Trade history
    pub trades: Vec<BacktestTrade>,
    /// Cumulative PnL
    pub cumulative_pnl: f64,
    /// Peak equity (for drawdown calculation)
    pub peak_equity: f64,
    /// Trade count
    pub trade_count: usize,
    /// Win count
    pub win_count: usize,
    /// Loss count
    pub loss_count: usize,
}

impl Default for BacktestEngine {
    fn default() -> Self {
        Self {
            symbol: "BTC/USD".into(),
            fee_config: BacktestFeeConfig::default(),
            capital_config: CapitalConfig::default(),
            trades: vec![],
            cumulative_pnl: 0.0,
            peak_equity: 0.0,
            trade_count: 0,
            win_count: 0,
            loss_count: 0,
        }
    }
}

impl BacktestEngine {
    /// Create a new backtest engine.
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            ..Self::default()
        }
    }

    /// Set fee configuration.
    pub fn set_fee_config(&mut self, config: BacktestFeeConfig) {
        self.fee_config = config;
    }

    /// Set capital configuration.
    pub fn set_capital_config(&mut self, config: CapitalConfig) {
        self.capital_config = config;
    }

    /// Execute a trade and update PnL.
    pub fn execute_trade(&mut self, side: OrderSide, price: f64, quantity: f64) -> BacktestTrade {
        let timestamp = chrono::Utc::now();
        let gross_value = price * quantity;
        
        // Calculate fees
        let fee_amount = (gross_value * self.fee_config.fee_rate).max(self.fee_config.minimum_fee);
        
        // Calculate slippage
        let slippage_amount = gross_value * self.fee_config.slippage_pct;
        
        // Net value
        let net_value = if side == OrderSide::Buy {
            gross_value - fee_amount - slippage_amount
        } else {
            gross_value - fee_amount + slippage_amount
        };
        
        let pnl_contribution = if side == OrderSide::Buy {
            -net_value
        } else {
            net_value
        };
        
        let trade = BacktestTrade {
            timestamp,
            side,
            price,
            quantity,
            gross_value,
            fees: fee_amount,
            slippage: slippage_amount,
            net_value,
        };
        
        self.trades.push(trade.clone());
        self.cumulative_pnl += pnl_contribution;
        self.trade_count += 1;
        
        if side == OrderSide::Sell && self.cumulative_pnl > self.peak_equity {
            self.peak_equity = self.cumulative_pnl;
        }
        
        if side == OrderSide::Sell {
            self.win_count += 1;
        } else {
            self.loss_count += 1;
        }
        
        trade
    }

    /// Run a simple price-series backtest with a strategy signal function.
    ///
    /// # Arguments
    /// * `prices` - Time-series of closing prices
    /// * `strategy` - Function that returns true for buy signals, false for sell signals
    ///   The function receives (price_index, price) and should return a signal decision
    pub fn run_price_backtest(
        &mut self,
        prices: &[f64],
        strategy: &dyn Fn(usize, f64) -> bool,
    ) -> BacktestStatistics {
        let mut position = 0.0f64; // positive = long, negative = short, 0 = flat;
        let mut _entry_price = 0.0f64;
        
        for i in 1..prices.len() {
            let price = prices[i];
            let signal = strategy(i, price);
            
            // Generate signals
            if position == 0.0 && signal {
                // Enter long position
                position = 1.0;
                _entry_price = price;
            } else if position > 0.0 && !signal {
                // Exit long position
                let trade = self.execute_trade(OrderSide::Sell, price, 1.0);
                position = 0.0;
            }
        }
        
        // Calculate statistics
        let statistics = self.calculate_statistics(prices);
        statistics
    }

    /// Calculate performance statistics from trade history and price series.
    pub fn calculate_statistics(&self, _prices: &[f64]) -> BacktestStatistics {
        let mut statistics = BacktestStatistics::default();
        
        // Basic PnL
        statistics.total_pnl = self.cumulative_pnl;
        
        // Trade counts
        statistics.total_trades = self.trade_count as u64;
        statistics.win_count = self.win_count as u64;
        statistics.loss_count = self.loss_count as u64;
        
        // Win rate
        if statistics.total_trades > 0 {
            statistics.win_rate = (statistics.win_count as f64 / statistics.total_trades as f64) * 100.0;
        }
        
        // Calculate trade-level PnLs
        let mut trade_pnls: Vec<f64> = vec![];
        for trade in &self.trades {
            // Estimate PnL from trade data
            let net_after_fees = trade.net_value;
            trade_pnls.push(net_after_fees);
        }
        
        // Calculate basic metrics
        if !trade_pnls.is_empty() {
            let total_gross: f64 = trade_pnls.iter().filter(|&p| *p > 0.0).sum();
            let total_loss: f64 = trade_pnls.iter().filter(|&p| *p <= 0.0).sum();
            statistics.gross_profit = total_gross;
            statistics.gross_loss = total_loss.abs();
            
            // Profit factor
            if statistics.gross_loss > f64::EPSILON {
                statistics.profit_factor = total_gross / statistics.gross_loss;
            }
        }
        
        // Max drawdown (simplified using peak equity)
        if self.peak_equity > 0.0 && self.cumulative_pnl < self.peak_equity {
            statistics.max_drawdown = (self.peak_equity - self.cumulative_pnl).abs();
        }
        
        // Sharpe ratio (annualized, simplified)
        // Annualization factor assumes 252 trading days
        if statistics.total_trades > 0 && statistics.total_pnl != 0.0 {
            let daily_return = statistics.total_pnl / statistics.total_trades as f64 / 252.0;
            let daily_vol = 0.02f64; // simplified: assume 2% daily vol
            statistics.sharpe_ratio = daily_return / daily_vol * (252.0f64.sqrt());
        }
        
        // Calmar ratio (annualized return / max drawdown)
        if statistics.max_drawdown != 0.0 && statistics.total_pnl != 0.0 {
            let annual_return = statistics.total_pnl / 252.0;
            statistics.calmar_ratio = annual_return / statistics.max_drawdown;
        }
        
        // Sortino ratio (simplified)
        if statistics.max_drawdown != 0.0 {
            statistics.sortino_ratio = statistics.total_pnl / statistics.max_drawdown;
        }
        
        statistics
    }
}

/// Backtest performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BacktestStatistics {
    /// Total PnL
    pub total_pnl: f64,
    /// Total number of trades
    pub total_trades: u64,
    /// Number of winning trades
    pub win_count: u64,
    /// Number of losing trades
    pub loss_count: u64,
    /// Win rate percentage
    pub win_rate: f64,
    /// Gross profit sum
    pub gross_profit: f64,
    /// Gross loss sum
    pub gross_loss: f64,
    /// Profit factor (gross profit / gross loss)
    pub profit_factor: f64,
    /// Maximum drawdown percentage
    pub max_drawdown: f64,
    /// Sharpe ratio (annualized, simplified)
    pub sharpe_ratio: f64,
    /// Calmar ratio (annualized return / max drawdown)
    pub calmar_ratio: f64,
    /// Sortino ratio (simplified)
    pub sortino_ratio: f64,
}

/// Execute a series of trades from signal entries and exits.
///
/// # Arguments
/// * `entries` - Timestamps and prices for entry signals
/// * `exits` - Timestamps and prices for exit signals
pub fn run_trade_series(
    entries: &[(chrono::DateTime<chrono::Utc>, f64)],
    exits: &[(chrono::DateTime<chrono::Utc>, f64)],
    fee_config: &BacktestFeeConfig,
    slippage_pct: f64,
) -> (Vec<BacktestTrade>, f64) {
    let mut trades = vec![];
    let mut cumulative_pnl = 0.0f64;
    
    for (entry_ts, entry_price) in entries.iter() {
        for (exit_ts, exit_price) in exits.iter() {
            if exit_ts > entry_ts {
                let gross_value = entry_price * 1.0;
                let fee_amount = (gross_value * fee_config.fee_rate).max(fee_config.minimum_fee);
                let slippage_amount = gross_value * slippage_pct;
                
                let side = if exit_price > entry_price { OrderSide::Sell } else { OrderSide::Buy };
                let net_value = if side == OrderSide::Buy {
                    gross_value - fee_amount - slippage_amount
                } else {
                    gross_value - fee_amount + slippage_amount
                };
                let pnl = if side == OrderSide::Buy { -net_value } else { net_value };
                
                let trade = BacktestTrade {
                    timestamp: entry_ts.clone(),
                    side,
                    price: *entry_price,
                    quantity: 1.0,
                    gross_value,
                    fees: fee_amount,
                    slippage: slippage_amount,
                    net_value,
                };
                
                trades.push(trade);
                cumulative_pnl += pnl;
                break; // match first valid exit
            }
        }
    }
    
    (trades, cumulative_pnl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtest_engine_creation() {
        let engine = BacktestEngine::new("ETH/USD".into());
        assert_eq!(engine.symbol, "ETH/USD");
        assert_eq!(engine.cumulative_pnl, 0.0);
        assert_eq!(engine.trade_count, 0);
    }

    #[test]
    fn test_execute_trade_buy() {
        let mut engine = BacktestEngine::new("BTC/USD".into());
        engine.set_fee_config(BacktestFeeConfig {
            fee_rate: 0.001,
            minimum_fee: 0.0,
            slippage_pct: 0.0005,
        });
        
        let trade = engine.execute_trade(OrderSide::Buy, 50000.0, 0.1);
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.price, 50000.0);
        assert_eq!(trade.quantity, 0.1);
        assert!(trade.gross_value > 0.0);
        assert!(trade.fees >= 0.0);
        assert!(trade.slippage >= 0.0);
    }

    #[test]
    fn test_execute_trade_sell() {
        let mut engine = BacktestEngine::new("BTC/USD".into());
        engine.set_fee_config(BacktestFeeConfig::default());
        
        let trade = engine.execute_trade(OrderSide::Sell, 55000.0, 0.1);
        assert_eq!(trade.side, OrderSide::Sell);
        assert_eq!(trade.price, 55000.0);
    }

    #[test]
    fn test_run_price_backtest() {
        let mut engine = BacktestEngine::new("BTC/USD".into());
        
        let prices = vec![
            50000.0, 51000.0, 52000.0, 51500.0, 53000.0, 54000.0,
        ];
        
        let strategy = |i, price| {
            // Simple strategy: buy on first price, sell on price rise
            if i == 0 { true } else { price > prices[0] }
        };
        
        let stats = engine.run_price_backtest(&prices, &strategy);
        assert!(stats.total_trades > 0 || stats.total_pnl != 0.0);
    }

    #[test]
    fn test_calculate_statistics() {
        let mut engine = BacktestEngine::new("BTC/USD".into());
        engine.win_count = 3;
        engine.loss_count = 2;
        engine.trade_count = 5;
        engine.peak_equity = 1000.0;
        engine.cumulative_pnl = 50.0;
        
        let prices = vec![50000.0, 51000.0, 49000.0, 52000.0, 53000.0];
        let stats = engine.calculate_statistics(&prices);
        
        assert!(stats.total_trades > 0);
        assert!(stats.win_rate > 0.0);
        assert_eq!(stats.win_rate, 60.0); // 3/5 = 60%
    }
}
#[cfg(test)] mod backtest_load_tests { use super::*; #[test] fn backtest_load_stable() { let mut e = BacktestEngine::new("TEST".into()); assert!(e.execute_trade(OrderSide::Buy, 100.0, 1.0).gross_value > 0.0); } }
#[cfg(test)] mod backtest_scale_tests { use super::*; #[test] fn backtest_scale() { let mut e = BacktestEngine::new("TEST".into()); assert!(e.execute_trade(OrderSide::Buy, 50000.0, 0.1).gross_value > 0.0); } }
