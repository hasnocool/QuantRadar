# QuantRadar CLI Reference

Market intelligence and quantitative research platform — command-line interface documentation.

## Global Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help (use with any subcommand for detailed usage) |
| `-V, --version` | Print version |

---

## Data Commands

### `data-import` — Import market data

```bash
quantaradar data-import <SOURCE> <FORMAT>
```

**Arguments:**
- `<SOURCE>` — Path to data file or exchange endpoint
- `<FORMAT>` — Data format (csv, json, parquet, arrow)

**Example:**
```bash
quantaradar data-import ./data/btc_1h.csv csv
quantaradar data-import wss://ws.kraken.com parquet
```

---

### `data-list` — List available datasets

```bash
quantaradar data-list
```

**Example:**
```bash
quantaradar data-list
```

---

### `data-validate` — Verify data quality

```bash
quantaradar data-validate <DATASET>
```

**Arguments:**
- `<DATASET>` — Dataset identifier to validate

**Example:**
```bash
quantaradar data-validate btc_1h
quantaradar data-validate ./data/eth_5m.parquet
```

---

## Analyze Commands

### `analyze-discover` — Discover markets

```bash
quantaradar analyze-discover
```

Discovers available markets/symbols from configured exchanges.

**Example:**
```bash
quantaradar analyze-discover
```

---

### `analyze-screen` — Screen specific symbol

```bash
quantaradar analyze-screen <SYMBOL>
```

Runs multi-family screeners (Trend, Breakout, MeanReversion, VolatilityExpansion, VolumeSurge, MomentumDivergence, SupportResistanceBounce, Momentum, Microstructure, Event) on a symbol.

**Arguments:**
- `<SYMBOL>` — Trading symbol (e.g., BTC/USD, ETH/USD)

**Example:**
```bash
quantaradar analyze-screen BTC/USD
quantaradar analyze-screen ETH/USD
```

---

### `analyze-scan` — Scan markets for opportunities

```bash
quantaradar analyze-scan
```

Scans configured universe for screening signals across all symbols.

**Example:**
```bash
quantaradar analyze-scan
```

---

### `analyze-features` — Calculate features for symbols

```bash
quantaradar analyze-features
```

Computes technical indicators (EMA, RSI, ATR, Bollinger Bands, volume z-scores, returns) for the configured universe.

**Example:**
```bash
quantaradar analyze-features
```

---

### `analyze-regime` — Detect market regime

```bash
quantaradar analyze-regime <SYMBOL>
```

Classifies market regime (BullTrend, BearTrend, SidewaysHighVol, etc.) using trend/volatility thresholds.

**Arguments:**
- `<SYMBOL>` — Trading symbol

**Example:**
```bash
quantaradar analyze-regime BTC/USD
quantaradar analyze-regime ETH/USD
```

---

### `analyze-rank` — Cross-sectional ranking

```bash
quantaradar analyze-rank
```

Ranks symbols by composite score across screening families.

**Example:**
```bash
quantaradar analyze-rank
```

---

### `analyze-pca` — PCA and correlation analysis

```bash
quantaradar analyze-pca
```

Performs principal component analysis on returns correlation matrix.

**Example:**
```bash
quantaradar analyze-pca
```

---

## Quick Reference Table

| Command | Arguments | Description |
|---------|-----------|-------------|
| `data-import <SOURCE> <FORMAT>` | source, format | Import market data |
| `data-list` | (none) | List datasets |
| `data-validate <DATASET>` | dataset | Validate data quality |
| `analyze-discover` | (none) | Discover markets |
| `analyze-screen <SYMBOL>` | symbol | Screen symbol (10 families) |
| `analyze-scan` | (none) | Scan universe |
| `analyze-features` | (none) | Calculate features |
| `analyze-regime <SYMBOL>` | symbol | Detect regime |
| `analyze-rank` | (none) | Cross-sectional rank |
| `analyze-pca` | (none) | PCA correlation |

---

## Screeners (10 Families)

The `analyze-screen` command runs all 10 screener families:

| Family | SignalFamily Enum | Description |
|--------|-------------------|-------------|
| Trend | `Trend` | EMA20 > EMA50 > EMA200 + positive momentum |
| Breakout | `Breakout` | 20-period close breakout + volume confirmation |
| Mean Reversion | `MeanReversion` | RSI < 30 + EMA20 distance < -1.5 ATR |
| Volatility Expansion | `VolatilityExpansion` | Bollinger width expanding + volume surge |
| Volume Surge | `VolumeSurge` | Volume z-score ≥ 2.0 + positive momentum |
| Momentum Divergence | `MomentumDivergence` | RSI divergence vs price action |
| Support/Resistance Bounce | `SupportResistanceBounce` | Near EMA20 ±1 ATR with RSI extremes |
| Momentum | `Momentum` | Short + medium-term momentum positive |
| Microstructure | `Microstructure` | Breakout with positive tick |
| Event | `Event` | New high/low or volume anomaly |

---

## Usage Examples

```bash
# Full workflow: import → validate → screen → rank
quantaradar data-import ./data/btc.csv csv
quantaradar data-validate btc
quantaradar analyze-screen BTC/USD
quantaradar analyze-rank

# Research pipeline: features → regime → scan
quantaradar analyze-features
quantaradar analyze-regime BTC/USD
quantaradar analyze-scan

# Portfolio analysis: PCA on multiple symbols
quantaradar analyze-pca
```

---

## Notes

- All commands output to stdout; errors to stderr
- Exit code 0 = success, non-zero = error
- Configuration via environment variables or config file (see `utils config`)
- Screeners use `SignalFamily` enum internally; output includes family, direction, score, regime, rationale, and feature map

---

*Generated from `quantaradar --help` and subcommand help. Run any command with `--help` for latest usage.*