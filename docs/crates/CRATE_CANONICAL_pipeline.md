# CRATE_CANONICAL_pipeline.md — Canonical Pipeline Crate Documentation

## quantaradar-pipeline

### Purpose
Real-time signal processing pipeline that transforms raw market data into tradable
trading signals. Orchestrates the flow from data ingestion through feature engineering
to signal generation and ranking.

### Version
0.2.0 (workspace-managed)

### License
MIT (workspace license)

### Rust Version
1.85 (workspace requirement)

### Key Types and APIs

#### Pipeline Stages
- `Pipeline` — Main pipeline orchestrator
  - Methods: `run(&mut self) -> Result<(), PipelineError>`,
    `add_stage(stage: Box<dyn PipelineStage>)`,
    `set_max_latency(ms: u64)`, `get_latency_metrics() -> LatencyMetrics`
  - Runs all registered stages in order, collects metrics

- `PipelineStage` — Trait for individual pipeline stages
  - Required method: `fn process(&mut self, data: &PipelineData) -> Result<(), StageError>`
  - Optional: `fn setup(&mut self)` — Called once at pipeline start
  - Optional: `fn teardown(&mut self)` — Called at pipeline shutdown

#### Pipeline Data
- `PipelineData` — Shared data context passed between stages
  - Fields: `ohlc_stream: mpsc::Receiver<NormalizedOhlc>`,
    `features: Option<FeaturesBundle>`, 
    `signals: Option<SignalBundle>`, `metadata: PipelineMetadata`
  - Methods: `insert::<T>(key: &str, value: T) -> Result<(), InsertError>`,
    `get::<T>(key: &str) -> Result<&T, GetError>`

- `PipelineMetadata` — Metadata accompanying data through pipeline
  - Fields: `source: String` (exchange symbol), `timestamp: SystemTime`,
    `ingest_stage: String`, `pipeline_stage: usize`, `metrics: BTreeMap<String, f64>`

#### Pipeline Stages (Concrete Implementations)
- `FeatureEngineeringStage` — Transforms raw OHLC into feature vectors
  - Uses `crates/shared/features` crate for feature computation
  - Outputs `FeaturesBundle` with 50+ feature fields
  - Configurable: which features to compute, feature parameters

- `SignalGenerationStage` — Generates alpha signals from features
  - Uses `crates/standalone/signals` crate for signal algorithms
  - Outputs `SignalBundle` with signal strengths per symbol
  - Configurable: which signal types (momentum, mean-reversion, etc.)

- `RankingStage` — Ranks signals and selects top-N
  - Methods: `rank(signals: &SignalBundle, n: usize) -> RankedSignals`
  - Configurable: ranking metric (sharpe, information ratio, custom)
  - Outputs top-N signals with scores

- `DriftDetectionStage` — Monitors for data drift and regime changes
  - Uses `crates/standalone/regime-detector` for regime detection
  - Triggers alerts when drift thresholds exceeded
  - Configurable: drift threshold, monitoring window

- `RiskManagementStage` — Applies risk filters to signals
  - Uses `crates/standalone/risk` for VaR/CVaR calculations
  - Filters signals that violate risk limits
  - Configurable: max portfolio var, position limits

#### Pipeline Control
- `PipelineConfig` — Configuration for pipeline behavior
  - Fields: `max_latency_ms: u64` (default: 100),
    `max_signals: usize` (default: 10),
    `enable_drift_detection: bool` (default: true),
    `enable_risk_filtering: bool` (default: true),
    `stage_order: Vec<PipelineStageType>` (default: all stages)

- `PipelineRunner` — High-level runner API
  - `Runner::new(config: PipelineConfig) -> Self`
  - `runner.run(data: PipelineData) -> Result<SignalBundle, PipelineError>`
  - `runner.with_stage(stage: Box<dyn PipelineStage>) -> Self`

### Async Architecture
- All pipeline stages are `async` compatible
- Uses `tokio::sync::mpsc` for data streaming between stages
- Backpressure handling via bounded channels (capacity: 100)
- Stage timeouts via `tokio::time::timeout`
- Metrics exported via `tracing` for observability

### Dependencies
**Runtime:**
- `tokio` v1 with macros, rt-multi-thread features
- `tracing` v0.1 with env-filter, fmt features
- `mpsc` channels via tokio sync
- `crates/shared/features` — Feature computation
- `crates/standalone/signals` — Signal generation
- `crates/standalone/regime-detector` — Drift detection
- `crates/standalone/risk` — Risk filtering
- `serde` v1 — Configuration serialization

**Development:**
- No specific dev dependencies beyond workspace deps

### Integration Points

#### Used By
- `crates/standalone/live-exec` — Real-time signal feed for execution
- `crates/standalone/backtest_engine` — Historical signal pipeline
- `crates/standalone/scheduler` — Automated research pipeline
- `crates/standalone/monitoring` — Pipeline metrics export
- `crates/standalone/paper-trading` — Signal feed for paper trading

#### Consumes From
- `crates/standalone/ingestion` — Market data output
- `crates/shared/features` — Feature vectors input
- `crates/standalone/signals` — Generated signals input (later stages)

### Representative Code Example

```rust
use quantaradar_pipeline::{Pipeline, PipelineConfig, PipelineData};
use quantaradar_ingestion::NormalizedOhlc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Configure pipeline with all stages
    let config = PipelineConfig {
        max_latency_ms: 50,
        max_signals: 5,
        enable_drift_detection: true,
        enable_risk_filtering: true,
        stage_order: PipelineStageType::all(),
    };

    let mut pipeline = Pipeline::new(config);

    // Create pipeline data with OHLC channel
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    pipeline.set_data(PipelineData {
        ohlc_stream: rx,
        features: None,
        signals: None,
        metadata: PipelineMetadata::default(),
    });

    // Run pipeline
    if let Err(e) = pipeline.run().await {
        eprintln!("Pipeline error: {:?}", e);
        return;
    }

    // Get generated signals
    let signals = pipeline.take_signals();
    println!("Generated {} signals", signals.len());
}
```

### Verification
- Pipeline compiles with `cargo check -p quantaradar-pipeline`
- All stages process data without deadlocks
- Backpressure tested with faster producer than consumer
- Timeout behavior: stages exceeding latency limit are dropped
- Drift detection triggers correctly on synthetic data shifts
- Risk filtering removes flagged signals as expected
- End-to-end: ingestion → features → signals → ranking pipeline integration test passes