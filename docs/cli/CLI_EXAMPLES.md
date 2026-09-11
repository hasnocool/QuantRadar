# QuantRadar CLI Examples

## Quick Start

### 1. Discover Markets
```bash
quantaradar analyze discover --exchange kraken --output markets.json
```
Discovers all available markets on Kraken and saves to JSON.

### 2. Fetch Historical Data
```bash
quantaradar utils fetch BTCUSD --interval 1440 --output-dir data/raw
```
Fetches 1-day OHLCV data for BTCUSD.

### 3. Ingest Real-time Data
```bash
quantaradar data ingest --exchange kraken --symbols BTCUSD ETHUSD ADAUSD --duration 3600
```
Ingests real-time data for 1 hour.

### 4. Validate Data Quality
```bash
quantaradar data validate --dataset BTCUSD --checks gap sequence checksum
```
Validates data quality with specified checks.

### 5. Calculate Features
```bash
quantaradar analyze features BTCUSD ETHUSD --output features.parquet
```
Calculates technical features for multiple symbols.

### 6. Detect Regime
```bash
quantaradar analyze regime BTCUSD --interval 1440
```
Detects current market regime.

### 7. Screen Markets
```bash
quantaradar analyze scan --interval 1440 --limit 50 --quote USD --output scan.json
```
Screens top 50 USD pairs for opportunities.

### 8. Backtest Strategy
```bash
quantaradar research backtest --strategy mean_reversion --dataset BTCUSD_1Y --cash 10000 --output backtest.json
```
Backtests mean reversion strategy.

### 9. Walk-forward Validation
```bash
quantaradar research walkforward --strategy mean_reversion --dataset BTCUSD_2Y --train-days 252 --test-days 63
```
Performs walk-forward validation.

### 10. Paper Trading
```bash
quantaradar execute paper --strategy mean_reversion --cash 10000
```
Runs strategy in paper trading mode.

## Advanced Workflows

### Complete Research Pipeline
```bash
#!/bin/bash
set -e

SYMBOL="BTCUSD"
EXCHANGE="kraken"

echo "1. Discovering markets..."
quantaradar analyze discover --exchange $EXCHANGE --output markets.json

echo "2. Fetching historical data..."
quantaradar utils fetch $SYMBOL --interval 1440 --output-dir data/raw

echo "3. Validating data..."
quantaradar data validate --dataset $SYMBOL --checks all

echo "4. Calculating features..."
quantaradar analyze features $SYMBOL --output features.parquet

echo "5. Detecting regime..."
quantaradar analyze regime $SYMBOL --interval 1440

echo "6. Backtesting..."
quantaradar research backtest --strategy mean_reversion --dataset ${SYMBOL}_1Y --cash 10000 --output backtest.json

echo "7. Walk-forward validation..."
quantaradar research walkforward --strategy mean_reversion --dataset ${SYMBOL}_2Y

echo "8. Generating report..."
quantaradar utils report --type backtest --output report.html

echo "Research complete!"
```

### Multi-Symbol Analysis
```bash
#!/bin/bash

SYMBOLS=("BTCUSD" "ETHUSD" "ADAUSD" "DOTUSD" "LINKUSD")

# Fetch all symbols
for sym in "${SYMBOLS[@]}"; do
  quantaradar utils fetch $sym --interval 1440 --output-dir data/raw &
done
wait

# Calculate features
quantaradar analyze features "${SYMBOLS[@]}" --output features.parquet

# Rank symbols
quantaradar analyze rank --universe crypto --interval 1440 --top-n 10

# Generate report
quantaradar utils report --type ranking --output ranking.html
```

### Strategy Development Workflow
```bash
# 1. Create strategy
quantaradar strategy create momentum --template ma_crossover

# 2. Test on historical data
quantaradar strategy test momentum --dataset BTCUSD_1Y

# 3. Optimize parameters
quantaradar research optimize --strategy momentum --dataset BTCUSD_1Y --method bayes

# 4. Compare with benchmarks
quantaradar research compare --strategies momentum buy_hold --dataset BTCUSD_1Y --output compare.json

# 5. Deploy to paper
quantaradar strategy deploy momentum --environment paper

# 6. Monitor performance
quantaradar ops monitor --component strategy
```

### Data Management Pipeline
```bash
# Import from external source
quantaradar data import --source csv --format parquet --output data/

# Clean data
quantaradar data clean --dataset BTCUSD --output data/clean/

# Validate quality
quantaradar data validate --dataset BTCUSD --checks gap sequence checksum

# Export for analysis
quantaradar data export --dataset BTCUSD --format csv --output ./export/

# Create dataset manifest
quantaradar data list --filter "BTC*" > datasets.txt
```

### Real-time Trading Setup
```bash
# Start data ingestion
quantaradar data ingest --exchange kraken --symbols BTCUSD ETHUSD --duration 0 &

# Start monitoring
quantaradar ops monitor --component websocket &

# Deploy strategy
quantaradar execute live --strategy mean_reversion --exchange kraken --paper true

# Monitor metrics
quantaradar ops metrics --component strategy --period 1h
```

### Backtesting with Multiple Strategies
```bash
STRATEGIES=("mean_reversion" "momentum" "breakout" "grid")

for strategy in "${STRATEGIES[@]}"; do
  echo "Backtesting $strategy..."
  quantaradar research backtest \
    --strategy $strategy \
    --dataset BTCUSD_2Y \
    --cash 10000 \
    --output results/${strategy}.json
  
  quantaradar research walkforward \
    --strategy $strategy \
    --dataset BTCUSD_2Y \
    --train-days 252 \
    --test-days 63
done

# Compare all results
quantaradar research compare \
  --strategies "${STRATEGIES[@]}" \
  --dataset BTCUSD_2Y \
  --output results/comparison.json
```

## Configuration Examples

### config.toml
```toml
[data]
dir = "/data/quantaradar"
format = "parquet"
compression = "zstd"

[exchange]
default = "kraken"
api_key = "${KRAKEN_API_KEY}"
secret = "${KRAKEN_SECRET}"

[backtest]
initial_cash = 10000
commission = 0.001
slippage = 0.0005

[monitoring]
log_level = "info"
metrics_port = 9090
```

### Environment Variables
```bash
export QUANTARADAR_DATA_DIR=/data/quantaradar
export QUANTARADAR_CONFIG=/config/quantaradar.toml
export KRAKEN_API_KEY=your_key
export KRAKEN_SECRET=your_secret
export LOG_LEVEL=info
```

## Tips

1. **Use `--help`** for command-specific help
2. **Chain commands** with pipes where possible
3. **Save outputs** to files for later analysis
4. **Use background jobs** (`&`) for long-running tasks
5. **Monitor logs** with `quantaradar ops logs`
6. **Validate data** before analysis
7. **Test strategies** in paper mode first
8. **Use walk-forward** for robust validation

## Common Issues

### Data Not Found
```bash
# Check available datasets
quantaradar data list

# Fetch missing data
quantaradar utils fetch SYMBOL --interval 1440
```

### Validation Failed
```bash
# Validate with detailed checks
quantaradar data validate --dataset SYMBOL --checks all -v

# Clean data
quantaradar data clean --dataset SYMBOL --output data/clean/
```

### Strategy Compilation Error
```bash
# Check strategy syntax
quantaradar strategy compile STRATEGY_NAME --output /tmp/

# View logs
quantaradar ops logs --component strategy --lines 100
```
