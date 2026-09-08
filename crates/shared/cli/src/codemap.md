# crates/cli/src/

## Responsibility
Command-line interface for QuantRadar platform. Provides comprehensive tools for market data management, analysis, research, strategy development, and execution.

## Design
- Hierarchical subcommands organized by domain (data, analyze, research, strategy, execute, ops, utils)
- Clap-based argument parsing with derive macros
- Async handlers for I/O operations
- Modular command structure for easy extension

## Flow
1. CLI parses command and arguments
2. Routes to appropriate handler based on subcommand
3. Handler executes business logic via crate APIs
4. Results written to output files or stdout
5. Exit code returned

## Integration
- Data: crates/persistent-data, crates/websocket, crates/storage
- Analysis: crates/feature-engine, crates/regime-detector, crates/ranking, crates/pca
- Research: crates/backtest_engine, crates/validation, crates/strategy_dsl
- Strategy: crates/expected-return, crates/hold-period, crates/signal-ensemble
- Execution: crates/execution, crates/paper-trading, crates/live-exec
- Ops: crates/monitoring, crates/scheduler, crates/dashboard

## Commands
- data: import, export, ingest, list, validate, clean
- analyze: discover, scan, screen, features, regime, rank, pca
- research: backtest, walkforward, generate, optimize, compare
- strategy: list, create, compile, test, deploy
- execute: paper, live, account, order, cancel
- ops: monitor, metrics, health, logs, schedule, jobs
- utils: fetch, replay, report, config
