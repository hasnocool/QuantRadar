# AGENT.md — QuantRadar Operating Contract & Agent Configuration

## Agent Definition
---
description: QuantRadar project agent with RTK-first conventions
mode: subagent
model: anthropic/claude-sonnet-4-6
---

This agent inherits global instructions from `~/.config/opencode/AGENTS.md` (RTK-first, skills, token etiquette).

## Project Context

QuantRadar — Rust + Python quantitative research platform.

**Structure:**
```
QuantRadar/
├── configs/              # Strategy configs, environment configs
├── crates/               # Rust workspace (shared + standalone crates)
│   ├── shared/           # Core shared crates (54 crates)
│   │   ├── archives/        # Historical data archiving
│   │   ├── backtest/        # Basic SMA backtest engine
│   │   ├── backtest_engine/ # Multi-symbol portfolio backtest engine
│   │   ├── cli/             # Main CLI binary (quantaradar)
│   │   ├── core/            # Core domain types (Bar, MarketId, Signal, Regime, Direction, OrderSide)
│   │   ├── dashboard/       # Dashboard components
│   │   ├── data-model/      # Observation/bar data models
│   │   ├── domain-model/    # Domain types (Direction, OrderSide, SignalFamily, Regime)
│   │   ├── exchange-kraken/ # Kraken REST + WebSocket v2 client
│   │   ├── execution/       # Risk controls + paper execution primitives
│   │   ├── expected-return/ # Expected return calculations
│   │   ├── experiment/      # Experiment tracking
│   │   ├── features/        # Technical indicators (EMA, RSI, ATR, Bollinger, volume-z)
│   │   ├── ingestion/       # Data ingestion pipeline
│   │   ├── microstructure/  # Order-book, trade-flow, liquidity analytics
│   │   ├── news_ingestion/  # News ingestion (Reddit, NewsAPI)
│   │   ├── order-book/      # Order book data structures
│   │   ├── ranking/         # Cross-sectional ranking model
│   │   ├── regime/          # Regime detection (classify, RegimeThresholds)
│   │   ├── reporting/       # Machine-readable reports
│   │   ├── research/        # Breadth, ranking, relative-strength, PCA, events
│   │   ├── screeners/       # Explainable screener families (trend, breakout, MR, vol-exp, vol-surge)
│   │   ├── signal-ensemble/ # Meta-model for combining signals
│   │   ├── storage/         # Persistent storage layer (Parquet/Arrow)
│   │   └── websocket/       # Generic WebSocket primitives
│   └── standalone/         # Extended crates (51 crates)
│       ├── data-quality/       # Quality flags / validation
│       ├── derivatives/        # Derivatives pricing/models
│       ├── ensemble/           # Ensemble methods
│       ├── event_bus/          # Event publishing
│       ├── events/             # Event types
│       ├── exchange-binance/   # Binance data adapter
│       ├── exchange-coinbase/  # Coinbase adapter
│       ├── expected_return/    # Expected return calculations
│       ├── feature-engine/     # Feature computation pipeline
│       ├── feature-store/      # Persistent feature storage
│       ├── freqtrade_integration/ # Freqtrade interop
│       ├── hold-period/        # Holding period analysis
│       ├── lineage/            # Feature lineage tracking
│       ├── live-exec/          # Isolated live-execution adapter (disabled)
│       ├── logging/            # Logging infrastructure
│       ├── model_registry/     # Model registry
│       ├── monitoring/         # Monitoring/observability
│       ├── multi_exchange/     # Multi-source coordination
│       ├── optimization/       # Optimization layer
│       ├── orderbook/          # Order book types
│       ├── paper-trading/      # Paper execution primitives (full engine)
│       ├── paper/              # Paper types
│       ├── pca/                # PCA analysis
│       ├── persistent-data/    # Durable storage layer
│       ├── pipeline/           # Signal pipeline (feature→screener→ensemble→rank→gate)
│       ├── portfolio/          # Portfolio types
│       ├── portfolio_risk/     # Portfolio risk engine (VaR/CVaR, regime scaling)
│       ├── promotion_gate/     # Research→Paper→Shadow→Canary→Live gates
│       ├── rate-limiter/       # Bounded concurrency controls
│       ├── regime-detector/    # Regime detection (bull/bear/neutral)
│       ├── registry/           # Experiment/model registry
│       ├── replay/             # Deterministic replay
│       ├── report-gen/         # Per-crate markdown report generator
│       ├── risk/               # Risk limits / VaR
│       ├── scheduler/          # Ops scheduling
│       ├── sentiment/          # Sentiment analysis
│       ├── signals/            # Signal types
│       ├── strategies/         # Strategy definitions
│       ├── strategy_dsl/       # DSL generation
│       ├── test-scale/         # Scale testing
│       ├── types/              # Shared type definitions
│       ├── universe/           # Symbol universe management
│       ├── universe_history/   # Historical universe tracking
│       └── validation/         # Walk-forward validation
├── python/                # Python research/analysis scripts
│   └── quantaradar/
│       ├── ml/               # ML models (Isolation Forest, etc.)
│       ├── walk_forward/     # Walk-forward evaluation (folds, metrics, aggregation)
│       ├── robustness.py     # Robustness + champion/challenger gates
│       └── test_robustness.py
├── data/                  # Market data
│   ├── raw/                # Raw OHLC JSON (e.g., BTC_USD.json, ETH_USD.json)
│   └── news/               # Cached news data
├── docs/                  # Documentation
│   ├── architecture/       # Architecture docs
│   ├── cli/                # CLI reference
│   ├── control-plan/       # Control plane docs (MASTERLIST.md, PLAN.md, TODO.md, AGENTS.md)
│   └── crates/             # Per-crate codemaps
├── plans/                 # Work plans (.omo/plans/)
├── tests/                 # Integration tests
│   ├── fixtures/           # Test fixtures (bar generator, feature row generator)
│   └── strategies/         # Strategy test modules
├── .omo/                  # OhMyOpenCode plans and state
│   ├── plans/              # Work plans
│   ├── drafts/             # Draft plans
│   ├── evidence/           # Verification evidence
│   ├── notepads/           # Scratch pads
│   ├── run-continuation/   # Continuation state
│   └── start-work/         # Start-work config
├── .agents/               # Agent runtime scripts and registry
│   ├── loop-agent.sh
│   ├── loop-cmd.sh
│   ├── recursive-agent.sh
│   ├── self-improving-agent.sh
│   └── registry.json
└── target/                # Build artifacts (gitignored)
```

> **Full detailed file tree (with all files):** See [`FILETREE.md`](FILETREE.md) — auto-generated 2026-09-11 20:05:23 UTC with complete file listing including every `.rs`, `.toml`, `.md`, `.py`, `.json` file in the project.

## Control Plane Integration

**Control-plane:** authority `MASTERLIST.md`, roadmap `PLAN.md`, operating contract `AGENT.md`, live state `TODO.md`, runtime `.agents/` (`registry.json` + scripts). Index: `CONTROL_PLANE.md`.

SEE ALSO: `docs/control-plane/MASTERLIST.md §3`

## Task Continuation & Retry

- Continue each agent task and each subagent task.
- If a task cannot proceed, close it as inactive and retry.
- Verify retry/close state via `.agents/registry.json` + `TODO.md`.

## Repository Map

A full codemap is available at `codemap.md` in the project root.

Before working on any task, read `codemap.md` to understand:
- Project architecture and entry points
- Directory responsibilities and design patterns
- Data flow and integration points between modules

For deep work on a specific folder, also read that folder's `codemap.md`.

---

## §2.6 / §24 Operating Contract

### Agent Runtime Commands
- `cargo run --bin quantaradar discover` — discover markets (1391 found)
- `cargo run --bin quantaradar scan` — scan for signals
- `cargo run --bin quantaradar fetch <PAIR>` — fetch OHLC for pair
- `cargo run --bin quantaradar screen <PAIR>` — screen a pair
- `cargo run --bin quantaradar backtest <FILE>` — backtest from data file
- `cargo run -p quantaradar -- discover`
- `cargo run -p quantaradar -- screen --pair BTC/USD --interval 1440`
- `cargo run -p quantaradar -- fetch BTC/USD --interval 1440`
- `cargo run -p quantaradar -- backtest data/raw/BTC_USD.json`

### Python Research Pipeline
```bash
python3 -m venv .venv && . .venv/bin/activate
pip install -e '.[dev,ml]'
```

### RTK Commands (Token Optimization)

**Meta Commands (always use rtk directly):**
```bash
rtk gain              # Show token savings analytics
rtk gain --history    # Show command usage history with savings
rtk discover          # Analyze Claude Code history for missed opportunities
rtk proxy <cmd>       # Execute raw command without filtering (for debugging)
```

**Core Usage — automatic command rewriting:**
```bash
rtk git              # Git with compact output
rtk gh               # GitHub CLI compact
rtk aws              # AWS CLI compact
rtk psql             # PostgreSQL compact
rtk pnpm             # pnpm compact
rtk err              # Show errors only
rtk test             # Tests failures only
rtk json             # Compact JSON
rtk deps             # Dependency summary
rtk env              # Filtered env vars
rtk find             # Find files
rtk diff             # Condensed diff
rtk log              # Filter logs
rtk dotnet           # .NET compact
rtk docker           # Docker compact
rtk kubectl          # Kubernetes compact
rtk oc               # OpenShift compact
rtk summary          # Command summary
rtk grep             # Compact grep
rtk rg               # Compact ripgrep
rtk init             # Init RTK
rtk wget             # Compact wget
rtk wc               # Word count
rtk config           # Config management
rtk session          # Session usage
rtk telemetry        # Telemetry control
rtk learn            # Learn corrections
rtk pipe             # Pipe filtering
rtk trust/untrust    # Trust filters
rtk verify           # Verify hooks
rtk help             # Help
```

**Advanced Features:**
```bash
git status           # Automatically rewritten as: rtk git status
ls -la               # Automatically rewritten as: rtk ls -la
cat file.txt         # Automatically rewritten as: rtk read file.txt
```

**RTK Usage Rules:**
- The rtk plugin auto-rewrites `bash` tool calls (`git status` → `rtk git status`)
- Prefer commands with RTK coverage; prefix `rtk` explicitly for raw/piped shells
- Unfiltered: `rtk proxy <cmd>` (tracked) or `rtk run <cmd>` (raw)
- Meta: `rtk gain` (savings), `rtk discover` (missed opportunities)
- Never pipe rtk output back through rtk

**OpenCode RTK Integration:**
```bash
# Start OpenCode with rtk enabled
opencode run "your message" --plugin opencode-rtk

# Disable rtk for specific session
opencode run "your message" --no-plugins

# Force full output (disable optimization for debugging)
opencode run "Show me the full stack trace" --rtk-level light

# Aggressive optimization for routine tasks
opencode run "Simple file read operation" --rtk-level aggressive
```

**RTK Optimization Levels:**
| Level | Behavior | When to Use |
|-------|----------|-------------|
| `light` | Minimal filtering, preserves most details | Debugging, learning |
| `medium` | Balanced optimization (default) | Routine work |
| `aggressive` | Maximum token savings | High-volume operations |

### Skills Available
- **Builtin command skills** (load on demand): programming, frontend, visual-qa, playwright, git-master, debugging, review-work, remove-ai-slops, refactor, lsp-setup, ast-grep, data-scientist, ulw-plan, start-work, handoff

### Token Etiquette
- Prefer rtk-filtered output over raw dumps; never cat whole files when grep/codegraph answers
- `codegraph_explore` is Read-equivalent for indexed code — call it before Read/grep
- Stop searching once you have enough context

### Project-Specific Conventions
- **Rust** — strict types, `thiserror`/`anyhow`, `sqlx` for DB, `tokio` async, `clap` CLI
- **Python** — `uv` + `ruff` + `basedpyright`, `pydantic` v2, `polars`/`duckdb` for data
- **Config** — TOML for configs, environment variables for secrets
- **Testing** — `cargo test` (Rust), `pytest` (Python), integration tests in `tests/`
- **Git** — conventional commits, `main` branch, PRs for changes

## QuantRadar-Specific Rules

### CLI Commands
```bash
# Build
cargo build --workspace
uv sync

# Test
cargo test --workspace
uv run pytest

# CLI (quantaradar)
cargo run --bin quantaradar discover          # discover markets (1391 found)
cargo run --bin quantaradar scan              # scan for signals
cargo run --bin quantaradar fetch <PAIR>      # fetch OHLC for pair
cargo run --bin quantaradar screen <PAIR>     # screen a pair
cargo run --bin quantaradar backtest <FILE>   # backtest from data file

# Alternative CLI (package run)
cargo run -p quantaradar -- discover
cargo run -p quantaradar -- screen --pair BTC/USD --interval 1440
cargo run -p quantaradar -- fetch BTC/USD --interval 1440
cargo run -p quantaradar -- backtest data/raw/BTC_USD.json

# Python research
python3 -m venv .venv && . .venv/bin/activate
pip install -e '.[dev,ml]'

# Lint/Format
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
uv run ruff check .
uv run basedpyright
```

### OpenCode & RTK Integration CLI
```bash
# OpenCode
opencode run "message"                    # Run OpenCode with RTK plugin
opencode run "message" --plugin opencode-rtk  # Explicitly enable rtk
opencode plugin list                      # List installed plugins
opencode config edit                      # Edit config
opencode plugin opencode-rtk --global     # Install rtk plugin globally

# RTK Meta
rtk gain                                  # Token savings analytics
rtk gain --history                        # Savings history
rtk discover                              # Find missed savings
rtk proxy <cmd>                           # Raw command (tracked)
rtk run <cmd>                             # Raw command

# RTK by Category
rtk git        rtk gh       rtk aws       # Infrastructure
rtk pnpm       rtk cargo    rtk pip       # Package managers
rtk docker     rtk kubectl  rtk oc       # Container/cluster
rtk test       rtk err      rtk json     # Testing/output
rtk diff       rtk grep    rtk rg        # Search/diff
rtk config     rtk session   rtk gain    # Session/analytics

# RTK Optimization Levels
opencode run "task" --rtk-level light     # Minimal filtering
opencode run "task" --rtk-level medium    # Balanced (default)
opencode run "task" --rtk-level aggressive # Maximum savings
```

### Pi Agent Tools
```bash
# Agent Tools
Agent                     # Launch subagent
SubagentWorkflow          # Orchestrate workflow
get_subagent_result       # Check agent status
steer_subagent            # Redirect agent

# Status Line Display (pi-statusline)
# Status line display via pi-statusline plugin

# File Operations
read                      # Read file
edit                      # Edit file
write                     # Create/overwrite file
bash                      # Execute commands
```

### Ponytail Commands (NOT INSTALLED - removed from active usage)
```bash
# /ponytail lite|full|ultra  # Set intensity - NOT AVAILABLE
# /ponytail-help            # Quick reference - NOT AVAILABLE
# /ponytail-review          # Review for over-engineering - NOT AVAILABLE
# /ponytail-audit           # Audit codebase - NOT AVAILABLE
# /ponytail-debt            # Debt ledger - NOT AVAILABLE
# /ponytail-gain            # Show impact - NOT AVAILABLE
```

### Agent Runtime Updates
- **.agents/loop-agent.sh** — contracts added referencing AGENT.md; non-blocking enforcement; termination guard: MAX required (default 10); breaks early if CMD writes .agents/DONE; missing AGENT.md -> warn, continue minimal (safe degrade §24)
- **.agents/loop-cmd.sh** — gateway (/loop): routes invocations to the right runtime agent, no polling; reads AGENT.md presence (warn if missing); forwards exit codes
- **.agents/recursive-agent.sh** — depth guard max 5 (§10); STOP file tolerance; registry append on each descent; new registry schema tolerated
- **.agents/self-improving-agent.sh** — failure recovery (§26): plateau (no gain over prev two reads) -> reset depth to 0, alert, exit 0 instead of spinning; score >= 0.8 stop condition; depth > 8 stop condition
- **.agents/pi-agent.sh** — bridge: connects pi framework to QuantRadar runtime agents and .opencode/agent/quantaradar.md; pass-through only: never wipes registry.json, never duplicates runtime logic, forwards exit codes

### .agents/Registry.json v1.1 Structure
```json
{
  "agents": [
    {"agent": "program-director", "depth": 0, "role": "program-director", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "plans/"},
    {"agent": "architect", "depth": 0, "role": "architect", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "crates/"},
    {"agent": "planner", "depth": 0, "role": "planner", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "plans/"},
    {"agent": "lead-implementer", "depth": 0, "role": "lead-implementer", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "crates/"},
    {"agent": "reviewer", "depth": 0, "role": "reviewer", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "crates/"},
    {"agent": "tester", "depth": 0, "role": "tester", "score": 0.0, "ts": 1788883200.0, "weight": 3, "workspace": "tests/"},
    {"agent": "integrator", "depth": 0, "role": "integrator", "score": 0.0, "ts": 1788883200.0, "weight": 2, "workspace": "."},
    {"agent": "researcher", "depth": 0, "role": "researcher", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": "python/quantaradar/"},
    {"agent": "loop-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "recursive-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "self-improving-agent", "depth": 0, "role": "supervisor", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".agents/"},
    {"agent": "pi-agent", "depth": 0, "role": "runtime", "score": 0.0, "ts": 1788883200.0, "weight": 1, "workspace": ".opencode/agent/"}
  ]
}
```

### Cross-Link References
- `.opencode/agent/quantaradar.md` — cross-references AGENT.md / MASTERLIST.md / TODO.md / .agents/registry.json
- `MASTERLIST.md` — stable spec: authority hierarchy (§3), mission (§1), architecture (§4)
- `TODO.md` — live projection with milestones mapped to P0–P7

## §24 Safe Degrade
If AGENT.md is missing, agent runtimes continue in minimal mode with warnings:
- loop-agent.sh: "warn: AGENT.md missing, minimal mode" >&2
- loop-cmd.sh: "warn: AGENT.md missing, minimal mode" >&2
- recursive-agent.sh: tolerates new registry schema
- self-improving-agent.sh: score defaults to 0.0 on unreadable registry
- pi-agent.sh: "warn: AGENT.md missing, minimal mode" >&2
