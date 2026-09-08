//! storage crate documentation.
// QuantRadar Parquet/Arrow storage layer for market data persistence.
use quantaradar_core::{QualityFlag, SourceKind};
use quantaradar_data_model::{MarketObservation, TradeObservation, OrderBookObservation, MarketDataBatch};
use anyhow::Result;
use arrow::array::{Float64Array, Int32Array, StringArray, UInt64Array, ListArray, ArrayRef};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub base_path: PathBuf,
    pub compression: Compression,
    pub row_group_size: usize,
    pub max_batch_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Uncompressed,
    Snappy,
    Gzip,
    Zstd,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("data"),
            compression: Compression::Snappy,
            row_group_size: 65_536,
            max_batch_size: 10_000,
        }
    }
}

impl StorageConfig {
    pub fn writer_properties(&self) -> WriterProperties {
        let mut builder = WriterProperties::builder();
        match self.compression {
            Compression::Uncompressed => builder = builder.set_compression(parquet::basic::Compression::UNCOMPRESSED),
            Compression::Snappy => builder = builder.set_compression(parquet::basic::Compression::SNAPPY),
            Compression::Gzip => builder = builder.set_compression(parquet::basic::Compression::GZIP(Default::default())),
            Compression::Zstd => builder = builder.set_compression(parquet::basic::Compression::ZSTD(Default::default())),
        }
        builder.set_data_page_size_limit(1_048_576).set_write_batch_size(1_048_576).build()
    }
}

/// Market data writer for Parquet files
pub struct MarketDataWriter {
    config: StorageConfig,
    observation_schema: Arc<Schema>,
    trade_schema: Arc<Schema>,
    orderbook_schema: Arc<Schema>,
}

impl MarketDataWriter {
    pub fn new(config: StorageConfig) -> Self {
        let observation_schema = Arc::new(Self::observation_schema());
        let trade_schema = Arc::new(Self::trade_schema());
        let orderbook_schema = Arc::new(Self::orderbook_schema());
        Self { config, observation_schema, trade_schema, orderbook_schema }
    }

    fn observation_schema() -> Schema {
        Schema::new(vec![
            Field::new("timestamp", DataType::UInt64, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("symbol", DataType::Utf8, false),
            Field::new("base", DataType::Utf8, false),
            Field::new("quote", DataType::Utf8, false),
            Field::new("open", DataType::Float64, false),
            Field::new("high", DataType::Float64, false),
            Field::new("low", DataType::Float64, false),
            Field::new("close", DataType::Float64, false),
            Field::new("volume", DataType::Float64, false),
            Field::new("trade_count", DataType::UInt64, false),
            Field::new("bid", DataType::Float64, false),
            Field::new("ask", DataType::Float64, false),
            Field::new("bid_depth_prices", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), true),
            Field::new("bid_depth_quantities", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), true),
            Field::new("ask_depth_prices", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), true),
            Field::new("ask_depth_quantities", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), true),
            Field::new("source", DataType::Utf8, false),
            Field::new("ingested_at", DataType::UInt64, false),
            Field::new("quality_flags", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
            Field::new("regime", DataType::Utf8, true),
        ])
    }

    fn trade_schema() -> Schema {
        Schema::new(vec![
            Field::new("timestamp", DataType::UInt64, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("symbol", DataType::Utf8, false),
            Field::new("price", DataType::Float64, false),
            Field::new("quantity", DataType::Float64, false),
            Field::new("side", DataType::Utf8, false),
            Field::new("trade_id", DataType::Utf8, false),
            Field::new("ingested_at", DataType::UInt64, false),
            Field::new("quality_flags", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
        ])
    }

    fn orderbook_schema() -> Schema {
        Schema::new(vec![
            Field::new("timestamp", DataType::UInt64, false),
            Field::new("exchange", DataType::Utf8, false),
            Field::new("symbol", DataType::Utf8, false),
            Field::new("bid_prices", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), false),
            Field::new("bid_quantities", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), false),
            Field::new("ask_prices", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), false),
            Field::new("ask_quantities", DataType::List(Arc::new(Field::new("item", DataType::Float64, true))), false),
            Field::new("sequence", DataType::UInt64, false),
            Field::new("ingested_at", DataType::UInt64, false),
            Field::new("quality_flags", DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))), false),
        ])
    }

    /// Write a batch of market data to Parquet files
    pub fn write_batch(&self, batch: &MarketDataBatch) -> Result<()> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let batch_id = batch.batch_id.to_string()[..8].to_string();

        // Write observations
        if !batch.observations.is_empty() {
            let path = self.config.base_path.join("raw/ohlcv").join(format!("obs_{}_{}.parquet", timestamp, batch_id));
            self.write_observations(&batch.observations, &path)?;
        }

        // Write trades
        if !batch.trades.is_empty() {
            let path = self.config.base_path.join("raw/trades").join(format!("trades_{}_{}.parquet", timestamp, batch_id));
            self.write_trades(&batch.trades, &path)?;
        }

        // Write order books
        if !batch.order_books.is_empty() {
            let path = self.config.base_path.join("raw/books").join(format!("books_{}_{}.parquet", timestamp, batch_id));
            self.write_order_books(&batch.order_books, &path)?;
        }

        Ok(())
    }

    fn write_observations(&self, observations: &[MarketObservation], path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let n = observations.len();
        let mut timestamps = Vec::with_capacity(n);
        let mut exchanges = Vec::with_capacity(n);
        let mut symbols = Vec::with_capacity(n);
        let mut bases = Vec::with_capacity(n);
        let mut quotes = Vec::with_capacity(n);
        let mut opens = Vec::with_capacity(n);
        let mut highs = Vec::with_capacity(n);
        let mut lows = Vec::with_capacity(n);
        let mut closes = Vec::with_capacity(n);
        let mut volumes = Vec::with_capacity(n);
        let mut trade_counts = Vec::with_capacity(n);
        let mut bids = Vec::with_capacity(n);
        let mut asks = Vec::with_capacity(n);
        let mut bid_depth_prices = Vec::with_capacity(n);
        let mut bid_depth_quantities = Vec::with_capacity(n);
        let mut ask_depth_prices = Vec::with_capacity(n);
        let mut ask_depth_quantities = Vec::with_capacity(n);
        let mut sources = Vec::with_capacity(n);
        let mut ingested_ats = Vec::with_capacity(n);
        let mut quality_flags = Vec::with_capacity(n);
        let mut regimes = Vec::with_capacity(n);

        for obs in observations {
            timestamps.push(obs.timestamp);
            exchanges.push(obs.exchange.clone());
            symbols.push(obs.symbol.clone());
            bases.push(obs.base.clone());
            quotes.push(obs.quote.clone());
            opens.push(obs.open);
            highs.push(obs.high);
            lows.push(obs.low);
            closes.push(obs.close);
            volumes.push(obs.volume);
            trade_counts.push(obs.trade_count);
            bids.push(obs.bid);
            asks.push(obs.ask);

            let bid_prices: Vec<f64> = obs.bid_depth.iter().map(|(p, _)| *p).collect();
            let bid_qtys: Vec<f64> = obs.bid_depth.iter().map(|(_, q)| *q).collect();
            let ask_prices: Vec<f64> = obs.ask_depth.iter().map(|(p, _)| *p).collect();
            let ask_qtys: Vec<f64> = obs.ask_depth.iter().map(|(_, q)| *q).collect();

            bid_depth_prices.push(bid_prices);
            bid_depth_quantities.push(bid_qtys);
            ask_depth_prices.push(ask_prices);
            ask_depth_quantities.push(ask_qtys);

            sources.push(match obs.source {
                SourceKind::Rest => "rest".to_string(),
                SourceKind::WebSocket => "websocket".to_string(),
                SourceKind::Replay => "replay".to_string(),
            });
            ingested_ats.push(obs.ingested_at);
            quality_flags.push(obs.quality_flags.iter().map(|f| format!("{:?}", f)).collect::<Vec<_>>());
            regimes.push(obs.regime.as_ref().map(|r| format!("{:?}", r)).unwrap_or_default());
        }

        let bid_depth_prices_arr = Self::build_list_array(&bid_depth_prices);
        let bid_depth_qtys_arr = Self::build_list_array(&bid_depth_quantities);
        let ask_depth_prices_arr = Self::build_list_array(&ask_depth_prices);
        let ask_depth_qtys_arr = Self::build_list_array(&ask_depth_quantities);
        let quality_flags_arr = Self::build_string_list_array(&quality_flags);

        let batch = RecordBatch::try_new(
            self.observation_schema.clone(),
            vec![
                Arc::new(UInt64Array::from(timestamps)),
                Arc::new(StringArray::from(exchanges)),
                Arc::new(StringArray::from(symbols)),
                Arc::new(StringArray::from(bases)),
                Arc::new(StringArray::from(quotes)),
                Arc::new(Float64Array::from(opens)),
                Arc::new(Float64Array::from(highs)),
                Arc::new(Float64Array::from(lows)),
                Arc::new(Float64Array::from(closes)),
                Arc::new(Float64Array::from(volumes)),
                Arc::new(UInt64Array::from(trade_counts)),
                Arc::new(Float64Array::from(bids)),
                Arc::new(Float64Array::from(asks)),
                bid_depth_prices_arr,
                bid_depth_qtys_arr,
                ask_depth_prices_arr,
                ask_depth_qtys_arr,
                Arc::new(StringArray::from(sources)),
                Arc::new(UInt64Array::from(ingested_ats)),
                quality_flags_arr,
                Arc::new(StringArray::from(regimes)),
            ],
        )?;

        self.write_record_batch(batch, path)
    }

    fn write_trades(&self, trades: &[TradeObservation], path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let n = trades.len();
        let mut timestamps = Vec::with_capacity(n);
        let mut exchanges = Vec::with_capacity(n);
        let mut symbols = Vec::with_capacity(n);
        let mut prices = Vec::with_capacity(n);
        let mut quantities = Vec::with_capacity(n);
        let mut sides = Vec::with_capacity(n);
        let mut trade_ids = Vec::with_capacity(n);
        let mut ingested_ats = Vec::with_capacity(n);
        let mut quality_flags = Vec::with_capacity(n);

        for t in trades {
            timestamps.push(t.timestamp);
            exchanges.push(t.exchange.clone());
            symbols.push(t.symbol.clone());
            prices.push(t.price);
            quantities.push(t.quantity);
            sides.push(match t.side {
                quantaradar_core::OrderSide::Buy => "buy".to_string(),
                quantaradar_core::OrderSide::Sell => "sell".to_string(),
            });
            trade_ids.push(t.trade_id.clone());
            ingested_ats.push(t.ingested_at);
            quality_flags.push(t.quality_flags.iter().map(|f| format!("{:?}", f)).collect::<Vec<_>>());
        }

        let quality_flags_arr = Self::build_string_list_array(&quality_flags);

        let batch = RecordBatch::try_new(
            self.trade_schema.clone(),
            vec![
                Arc::new(UInt64Array::from(timestamps)),
                Arc::new(StringArray::from(exchanges)),
                Arc::new(StringArray::from(symbols)),
                Arc::new(Float64Array::from(prices)),
                Arc::new(Float64Array::from(quantities)),
                Arc::new(StringArray::from(sides)),
                Arc::new(StringArray::from(trade_ids)),
                Arc::new(UInt64Array::from(ingested_ats)),
                quality_flags_arr,
            ],
        )?;

        self.write_record_batch(batch, path)
    }

    fn write_order_books(&self, books: &[OrderBookObservation], path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let n = books.len();
        let mut timestamps = Vec::with_capacity(n);
        let mut exchanges = Vec::with_capacity(n);
        let mut symbols = Vec::with_capacity(n);
        let mut bid_prices = Vec::with_capacity(n);
        let mut bid_quantities = Vec::with_capacity(n);
        let mut ask_prices = Vec::with_capacity(n);
        let mut ask_quantities = Vec::with_capacity(n);
        let mut sequences = Vec::with_capacity(n);
        let mut ingested_ats = Vec::with_capacity(n);
        let mut quality_flags = Vec::with_capacity(n);

        for b in books {
            timestamps.push(b.timestamp);
            exchanges.push(b.exchange.clone());
            symbols.push(b.symbol.clone());

            let bp: Vec<f64> = b.bids.iter().map(|(p, _)| *p).collect();
            let bq: Vec<f64> = b.bids.iter().map(|(_, q)| *q).collect();
            let ap: Vec<f64> = b.asks.iter().map(|(p, _)| *p).collect();
            let aq: Vec<f64> = b.asks.iter().map(|(_, q)| *q).collect();

            bid_prices.push(bp);
            bid_quantities.push(bq);
            ask_prices.push(ap);
            ask_quantities.push(aq);

            sequences.push(b.sequence);
            ingested_ats.push(b.ingested_at);
            quality_flags.push(b.quality_flags.iter().map(|f| format!("{:?}", f)).collect::<Vec<_>>());
        }

        let bid_prices_arr = Self::build_list_array(&bid_prices);
        let bid_qtys_arr = Self::build_list_array(&bid_quantities);
        let ask_prices_arr = Self::build_list_array(&ask_prices);
        let ask_qtys_arr = Self::build_list_array(&ask_quantities);
        let quality_flags_arr = Self::build_string_list_array(&quality_flags);

        let batch = RecordBatch::try_new(
            self.orderbook_schema.clone(),
            vec![
                Arc::new(UInt64Array::from(timestamps)),
                Arc::new(StringArray::from(exchanges)),
                Arc::new(StringArray::from(symbols)),
                bid_prices_arr,
                bid_qtys_arr,
                ask_prices_arr,
                ask_qtys_arr,
                Arc::new(UInt64Array::from(sequences)),
                Arc::new(UInt64Array::from(ingested_ats)),
                quality_flags_arr,
            ],
        )?;

        self.write_record_batch(batch, path)
    }

    fn build_list_array(values: &[Vec<f64>]) -> ArrayRef {
        let mut all_values = Vec::new();
        let mut offsets = Vec::with_capacity(values.len() + 1);
        offsets.push(0);
        for v in values {
            all_values.extend_from_slice(v);
            offsets.push(offsets.last().unwrap() + v.len() as i32);
        }
        let value_array = Arc::new(Float64Array::from(all_values)) as ArrayRef;
        let offset_buffer = arrow::buffer::OffsetBuffer::new(offsets.into());
        Arc::new(ListArray::new(
            Arc::new(Field::new("item", DataType::Float64, true)),
            offset_buffer,
            value_array,
            None,
        ))
    }

    fn build_string_list_array(values: &[Vec<String>]) -> ArrayRef {
        let mut all_values = Vec::new();
        let mut offsets = Vec::with_capacity(values.len() + 1);
        offsets.push(0);
        for v in values {
            all_values.extend(v.iter().cloned());
            offsets.push(offsets.last().unwrap() + v.len() as i32);
        }
        let value_array = Arc::new(StringArray::from(all_values)) as ArrayRef;
        let offset_buffer = arrow::buffer::OffsetBuffer::new(offsets.into());
        Arc::new(ListArray::new(
            Arc::new(Field::new("item", DataType::Utf8, true)),
            offset_buffer,
            value_array,
            None,
        ))
    }

    fn write_record_batch(&self, batch: RecordBatch, path: &Path) -> Result<()> {
        let file = fs::File::create(path)?;
        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(self.config.writer_properties()))?;
        writer.write(&batch)?;
        writer.close()?;
        Ok(())
    }
}

pub struct MarketDataReader {
    config: StorageConfig,
}

impl MarketDataReader {
    pub fn new(config: StorageConfig) -> Self {
        Self { config }
    }

    pub fn read_observations(
        &self,
        start: u64,
        end: u64,
        symbol: Option<&str>,
        exchange: Option<&str>,
    ) -> Result<Vec<MarketObservation>> {
        let mut observations = Vec::new();
        let dir = self.config.base_path.join("raw/ohlcv");
        if !dir.exists() {
            return Ok(observations);
        }
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("parquet") {
                continue;
            }
            let _ = path;
        }
        Ok(observations)
    }

    pub fn read_trades(&self, start: u64, end: u64, symbol: Option<&str>) -> Result<Vec<TradeObservation>> {
        Ok(Vec::new())
    }

    pub fn read_order_books(&self, start: u64, end: u64, symbol: Option<&str>) -> Result<Vec<OrderBookObservation>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert_eq!(config.compression, Compression::Snappy);
        assert_eq!(config.row_group_size, 65_536);
    }

    #[test]
    fn test_write_observations() {
        let dir = tempdir().unwrap();
        let config = StorageConfig {
            base_path: dir.path().to_path_buf(),
            ..Default::default()
        };

        let writer = MarketDataWriter::new(config.clone());

        let mut batch = MarketDataBatch::new();
        batch.observations.push(MarketObservation::new(
            1_700_000_000_000,
            "kraken".into(),
            "BTC/USD".into(),
            "BTC".into(),
            "USD".into(),
            50_000.0, 51_000.0, 49_000.0, 50_500.0,
            100.0, 50, 50_490.0, 50_510.0,
            vec![(50_490.0, 1.0)],
            vec![(50_510.0, 1.5)],
            SourceKind::Rest,
            1_700_000_000_100,
            vec![QualityFlag::Valid],
        ));

        writer.write_batch(&batch).unwrap();

        // Find the written file
        let entries = fs::read_dir(dir.path().join("raw/ohlcv")).unwrap();
        let mut file_path = None;
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().extension().map_or(false, |e| e == "parquet") {
                file_path = Some(entry.path());
                break;
            }
        }

        assert!(file_path.is_some());
        // Verify file exists and has content
        let metadata = fs::metadata(&file_path.unwrap()).unwrap();
        assert!(metadata.len() > 0);
    }
}
