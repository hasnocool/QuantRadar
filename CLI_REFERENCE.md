# QuantRadar CLI - Complete Reference

## Overview
QuantRadar provides a comprehensive command-line interface for market intelligence and quantitative research.

## Installation
```bash
cargo install --path crates/cli
```

## Usage
```bash
quantaradar <COMMAND> [OPTIONS]
```

## Commands

### Data Management
Manage market data ingestion, validation, and persistence.

```bash
# Import data
quantaradar data import --source csv --format parquet --output data/

# Export data
quantaradar data export --dataset BTCUSD --format csv --output ./export/

# Ingest real-time data
quantaradar data ingest --exchange kraken --symbols BTCUSD ETHUSD --duration 3600

# List datasets
quantaradar data list --filter "BTC*"

# Validate data quality
quantaradar data validate --dataset BTCUSD --checks gap sequence checksum

# Clean data
quantaradar data clean --dataset BTCUSD --output data/clean/
```

### Analysis
Market analysis and screening tools.

```bash
# Discover markets
quantaradar analyze discover --exchange kraken --output markets.json

# Scan for opportunities
quantaradar analyze scan --interval 1440 --limit 50 --quote USD --output scan.json

# Screen specific symbol
quantaradar analyze screen BTCUSD --interval 1440 --output screen.json

# Calculate features
quantaradar analyze features BTCUSD ETHUSD --output features.parquet

# Detect regime
quantaradar analyze regime BTCUSD --interval 1440

# Cross-sectional ranking
quantaradar analyze rank --universe crypto --interval 1440 --top-n 10

# PCA analysis
quantaradar analyze pca --symbols BTCUSD ETHUSD --interval 1440 --components 3
```

### Research & Backtesting
Strategy development and validation.

```bash
# Run backtest
quantaradar research backtest --strategy mean_reversion --dataset BTCUSD_1Y --cash 10000 --output backtest.json

# Walk-forward validation
quantaradar research walkforward --strategy mean_reversion --dataset BTCUSD_2Y --train-days 252 --test-days 63

# Generate strategy from template
quantaradar research generate --template mean_reversion --output strategies/my_strategy.rs

# Optimize parameters
quantaradar research optimize --strategy mean_reversion --dataset BTCUSD_1Y --method bayes

# Compare strategies
quantaradar research compare --strategies s1 s2 s3 --dataset BTCUSD_1Y --output compare.json
```

### Strategy Management
Manage trading strategies.

```bash
# List strategies
quantaradar strategy list --filter "mean*"

# Create new strategy
quantaradar strategy create my_strategy --template mean_reversion

# Compile strategy
quantaradar strategy compile my_strategy --output strategies/

# Test strategy
quantaradar strategy test my_strategy --dataset BTCUSD_1Y

# Deploy strategy
quantaradar strategy deploy my_strategy --environment paper
```

### Execution
Paper and live trading.

```bash
# Paper trading
quantaradar execute paper --strategy mean_reversion --cash 10000

# Live trading
quantaradar execute live --strategy mean_reversion --exchange kraken --paper false

# Check account
quantaradar execute account --exchange kraken

# Execute order
quantaradar execute order BTCUSD buy 0.1 --price 50000 --exchange kraken

# Cancel order
quantaradar execute cancel order_123 --exchange kraken
```

### Operations
System monitoring and management.

```bash
# Monitor components
quantaradar ops monitor --component websocket

# View metrics
quantaradar ops metrics --component backtest --period 24h

# Check health
quantaradar ops health

# View logs
quantaradar ops logs --component ingestion --lines 100

# Schedule job
quantaradar ops schedule daily_backtest --cron "0 0 * * *"

# List jobs
quantaradar ops jobs
```

### Utilities
Miscellaneous tools.

```bash
# Fetch historical data
quantaradar utils fetch BTCUSD --interval 1440 --output-dir data/raw

# Replay dataset
quantaradar utils replay dataset_123 --symbol BTCUSD --start 2024-01-01

# Generate report
quantaradar utils report --type backtest --output report.html

# Export config
quantaradar utils config --output config.yaml

# Import config
quantaradar utils import-config --input config.yaml
```

## Examples

### Complete Workflow
```bash
# 1. Discover markets
quantaradar analyze discover --exchange kraken --output markets.json

# 2. Ingest data
quantaradar data ingest --exchange kraken --symbols BTCUSD ETHUSD &

# 3. Validate data
quantaradar data validate --dataset BTCUSD --checks all

# 4. Calculate features
quantaradar analyze features BTCUSD ETHUSD --output features.parquet

# 5. Screen markets
quantaradar analyze scan --interval 1440 --limit 100 --quote USD --output scan.json

# 6. Backtest strategy
quantaradar research backtest --strategy mean_reversion --dataset BTCUSD_1Y --cash 10000

# 7. Walk-forward validation
quantaradar research walkforward --strategy mean_reversion --dataset BTCUSD_2Y

# 8. Deploy to paper
quantaradar execute paper --strategy mean_reversion --cash 10000
```

### Data Pipeline
```bash
# Import from CSV
quantaradar data import --source csv --format parquet --output data/

# Clean data
quantaradar data clean --dataset BTCUSD --output data/clean/

# Validate
quantaradar data validate --dataset BTCUSD --checks gap sequence

# Export
quantaradar data export --dataset BTCUSD --format csv --output ./export/
```

## Configuration
Configuration files are located in:
- `~/.quantaradar/config.toml` - User config
- `./config/` - Project config
- `crates/cli/config/` - Default config

## Environment Variables
```bash
export QUANTARADAR_DATA_DIR=/data/quantaradar
export QUANTARADAR_LOG_LEVEL=info
export QUANTARADAR_EXCHANGE_API_KEY=...
export QUANTARADAR_EXCHANGE_SECRET=...
```

## Shell Completion
```bash
# Bash
quantaradar completions bash > /etc/bash_completion.d/quantaradar

# Zsh
quantaradar completions zsh > ~/.zsh/completions/_quantaradar

# Fish
quantaradar completions fish > ~/.config/fish/completions/quantaradar.fish
```

## Exit Codes
- 0 - Success
- 1 - General error
- 2 - Invalid arguments
- 3 - Data not found
- 4 - Validation failed
- 5 - Execution error

## Support
For issues and questions:
- GitHub: https://github.com/quantradar/quantradar
- Docs: https://docs.quantradar.dev
- Discord: https://discord.gg/quantradar
