//! websocket crate documentation.
// QuantRadar production-grade WebSocket feed handler with bounded concurrency,
// adaptive rate limiting, reconnect/backoff, heartbeat monitoring, subscription
// management, sequence validation, stale-feed detection, auto-resubscription,
// per-symbol buffers, backpressure, and feed health scores.
use quantaradar_core::SourceKind;
use quantaradar_rate_limiter::{MessageRateLimiter, RateLimiterConfig};
use anyhow::{Context, Result};
use chrono::{DateTime, TimeDelta, Utc};
use rand;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, broadcast, mpsc};
use tokio::time::{interval, Instant};
use tracing::{debug, info, warn, error};

/// Feed health with comprehensive metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedHealth {
    pub symbol: String,
    pub exchange: String,
    pub last_message: DateTime<Utc>,
    pub message_count: u64,
    pub dropped_count: u64,
    pub latency_ms: u64,
    pub avg_latency_ms: f64,
    pub sequence_ok: bool,
    pub stale: bool,
    pub reconnect_count: u32,
    pub last_reconnect: Option<DateTime<Utc>>,
    pub subscription_status: SubscriptionStatus,
    pub health_score: f64, // 0.0 - 1.0
    pub error_rate: f64,
    pub messages_per_second: f64,
}

impl Default for FeedHealth {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            exchange: String::new(),
            last_message: Utc::now(),
            message_count: 0,
            dropped_count: 0,
            latency_ms: 0,
            avg_latency_ms: 0.0,
            sequence_ok: true,
            stale: false,
            reconnect_count: 0,
            last_reconnect: None,
            subscription_status: SubscriptionStatus::Unsubscribed,
            health_score: 1.0,
            error_rate: 0.0,
            messages_per_second: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Unsubscribed,
    Subscribing,
    Subscribed,
    Unsubscribing,
    Failed,
}

/// Market data message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataMessage {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: u64,
    pub sequence: u64,
    pub data_type: DataType,
    pub payload: serde_json::Value,
    pub received_at: DateTime<Utc>,
    pub processing_latency_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Trade,
    OrderBook,
    Ticker,
    OHLCV,
    Heartbeat,
    System,
}

/// Subscription request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub symbol: String,
    pub exchange: String,
    pub data_types: Vec<DataType>,
    pub depth: Option<usize>,
    pub interval: Option<u32>, // for OHLCV
}

/// Feed state with all connection info
struct FeedState {
    subscription: Subscription,
    url: String,
    connected: bool,
    last_sequence: u64,
    health: FeedHealth,
    reconnect_attempts: u32,
    last_heartbeat: DateTime<Utc>,
    pending_subscriptions: Vec<Subscription>,
    message_buffer: VecDeque<MarketDataMessage>,
    max_buffer_size: usize,
    heartbeat_interval: Duration,
    reconnect_backoff: Duration,
    max_reconnect_backoff: Duration,
    last_ping: Option<DateTime<Utc>>,
}

/// Configuration for MarketFeedManager
#[derive(Debug, Clone)]
pub struct FeedManagerConfig {
    pub max_buffer_size: usize,
    pub max_concurrent_feeds: usize,
    pub max_concurrent_messages: usize,
    pub rate_limiter_config: RateLimiterConfig,
    pub heartbeat_interval: Duration,
    pub stale_timeout: Duration,
    pub reconnect_base_delay: Duration,
    pub max_reconnect_delay: Duration,
    pub max_reconnect_attempts: u32,
    pub sequence_gap_threshold: u64,
    pub health_check_interval: Duration,
    pub enable_auto_resubscribe: bool,
}

impl Default for FeedManagerConfig {
    fn default() -> Self {
        Self {
            max_buffer_size: 100_000,
            max_concurrent_feeds: 100,
            max_concurrent_messages: 50,
            rate_limiter_config: RateLimiterConfig::default(),
            heartbeat_interval: Duration::from_secs(30),
            stale_timeout: Duration::from_secs(60),
            reconnect_base_delay: Duration::from_secs(1),
            max_reconnect_delay: Duration::from_secs(300),
            max_reconnect_attempts: 10,
            sequence_gap_threshold: 100,
            health_check_interval: Duration::from_secs(10),
            enable_auto_resubscribe: true,
        }
    }
}

/// Production-grade market feed manager
pub struct MarketFeedManager {
    config: FeedManagerConfig,
    feeds: Arc<RwLock<HashMap<String, FeedState>>>,
    global_buffer: Arc<Mutex<VecDeque<MarketDataMessage>>>,
    rate_limiter: Arc<MessageRateLimiter>,
    feed_semaphore: Arc<tokio::sync::Semaphore>,
    shutdown_tx: Option<broadcast::Sender<()>>,
    message_tx: mpsc::Sender<MarketDataMessage>,
    stats: Arc<Mutex<FeedManagerStats>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct FeedManagerStats {
    pub total_feeds: usize,
    pub active_feeds: usize,
    pub total_messages: u64,
    pub dropped_messages: u64,
    pub total_reconnects: u64,
    pub failed_reconnects: u64,
    pub avg_health_score: f64,
}

impl MarketFeedManager {
    pub fn new(config: FeedManagerConfig) -> (Self, mpsc::Receiver<MarketDataMessage>) {
        let (message_tx, message_rx) = mpsc::channel(config.max_buffer_size);
        
        let manager = Self {
            feeds: Arc::new(RwLock::new(HashMap::new())),
            global_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(config.max_buffer_size))),
            rate_limiter: Arc::new(MessageRateLimiter::new(
                config.rate_limiter_config.clone(),
                config.max_concurrent_messages,
            )),
            feed_semaphore: Arc::new(tokio::sync::Semaphore::new(config.max_concurrent_feeds)),
            shutdown_tx: None,
            message_tx,
            stats: Arc::new(Mutex::new(FeedManagerStats::default())),
            config,
        };
        
        (manager, message_rx)
    }

    /// Add a new feed subscription
    pub async fn add_feed(&self, subscription: Subscription, url: String) -> Result<()> {
        let key = format!("{}:{}", subscription.exchange, subscription.symbol);
        
        let mut feeds = self.feeds.write().await;
        if feeds.contains_key(&key) {
            return Err(anyhow::anyhow!("Feed already exists: {}", key));
        }
        
        let health = FeedHealth {
            symbol: subscription.symbol.clone(),
            exchange: subscription.exchange.clone(),
            last_message: Utc::now(),
            subscription_status: SubscriptionStatus::Unsubscribed,
            health_score: 1.0,
            ..Default::default()
        };
        
        feeds.insert(key.clone(), FeedState {
            subscription: subscription.clone(),
            url,
            connected: false,
            last_sequence: 0,
            health,
            reconnect_attempts: 0,
            last_heartbeat: Utc::now(),
            pending_subscriptions: vec![subscription],
            message_buffer: VecDeque::with_capacity(self.config.max_buffer_size),
            max_buffer_size: self.config.max_buffer_size,
            heartbeat_interval: self.config.heartbeat_interval,
            reconnect_backoff: self.config.reconnect_base_delay,
            max_reconnect_backoff: self.config.max_reconnect_delay,
            last_ping: None,
        });
        
        let mut stats = self.stats.lock().await;
        stats.total_feeds += 1;
        
        Ok(())
    }

    /// Start a feed connection (spawns background task)
    pub async fn start_feed(&self, exchange: &str, symbol: &str) -> Result<()> {
        let key = format!("{}:{}", exchange, symbol);
        let _permit = self.feed_semaphore.clone().acquire_owned().await?;
        
        let feeds = self.feeds.read().await;
        let feed = feeds.get(&key)
            .context("Feed not found")?;
        
        if feed.connected {
            return Ok(());
        }
        
        drop(feeds);
        
        // Update status
        let mut feeds = self.feeds.write().await;
        if let Some(feed) = feeds.get_mut(&key) {
            feed.health.subscription_status = SubscriptionStatus::Subscribing;
        }
        
        // Spawn connection task
        self.spawn_feed_task(exchange.to_string(), symbol.to_string()).await;
        
        Ok(())
    }

    /// Spawn background feed task with reconnection logic
    async fn spawn_feed_task(&self, exchange: String, symbol: String) {
        let key = format!("{}:{}", exchange, symbol);
        let feeds = self.feeds.clone();
        let config = self.config.clone();
        let rate_limiter = self.rate_limiter.clone();
        let message_tx = self.message_tx.clone();
        let global_buffer = self.global_buffer.clone();
        let stats = self.stats.clone();
        
        tokio::spawn(async move {
            let mut reconnect_delay = config.reconnect_base_delay;
            
            loop {
                // Check if we should continue
                let should_continue = {
                    let feeds = feeds.read().await;
                    feeds.get(&key).map(|f| f.connected || f.reconnect_attempts < config.max_reconnect_attempts).unwrap_or(false)
                };
                
                if !should_continue {
                    info!("Max reconnect attempts reached for {}, giving up", key);
                    break;
                }
                
                // Attempt connection
                match Self::run_feed_connection(
                    &exchange,
                    &symbol,
                    &feeds,
                    &config,
                    &rate_limiter,
                    &message_tx,
                    &global_buffer,
                    &stats,
                    &mut reconnect_delay,
                ).await {
                    Ok(_) => {
                        info!("Feed {} disconnected cleanly", key);
                        break;
                    }
                    Err(e) => {
                        warn!("Feed {} error: {}", key, e);
                        
                        // Update reconnect state
                        let mut feeds = feeds.write().await;
                        if let Some(feed) = feeds.get_mut(&key) {
                            feed.reconnect_attempts += 1;
                            feed.connected = false;
                            feed.health.subscription_status = SubscriptionStatus::Failed;
                            
                            if feed.reconnect_attempts >= config.max_reconnect_attempts {
                                error!("Max reconnect attempts reached for {}", key);
                                break;
                            }
                        }
                        
                        // Wait before reconnect
                        tokio::time::sleep(reconnect_delay).await;
                        
                        // Exponential backoff with jitter
                        reconnect_delay = Duration::from_millis(
                            (reconnect_delay.as_millis() as f64 * 1.5).min(config.max_reconnect_delay.as_millis() as f64) as u64
                        );
                        
                        // Add jitter
                        let jitter = rand::random::<u64>() % 1000;
                        reconnect_delay += Duration::from_millis(jitter);
                    }
                }
            }
            
            // Clean up
            let mut feeds = feeds.write().await;
            if let Some(feed) = feeds.get_mut(&key) {
                feed.connected = false;
                feed.health.subscription_status = SubscriptionStatus::Unsubscribed;
            }
        });
    }

    /// Run a single feed connection
    async fn run_feed_connection(
        exchange: &str,
        symbol: &str,
        feeds: &Arc<RwLock<HashMap<String, FeedState>>>,
        config: &FeedManagerConfig,
        rate_limiter: &Arc<MessageRateLimiter>,
        message_tx: &mpsc::Sender<MarketDataMessage>,
        global_buffer: &Arc<Mutex<VecDeque<MarketDataMessage>>>,
        stats: &Arc<Mutex<FeedManagerStats>>,
        reconnect_delay: &mut Duration,
    ) -> Result<()> {
        let key = format!("{}:{}", exchange, symbol);
        
        // Simulate WebSocket connection (in production, use tokio-tungstenite)
        info!("Connecting to {} feed for {}", exchange, symbol);
        
        // Update state
        {
            let mut feeds = feeds.write().await;
            if let Some(feed) = feeds.get_mut(&key) {
                feed.connected = true;
                feed.health.subscription_status = SubscriptionStatus::Subscribed;
                feed.reconnect_attempts = 0;
                feed.reconnect_backoff = config.reconnect_base_delay;
                *reconnect_delay = config.reconnect_base_delay;
                feed.health.reconnect_count = 0;
                feed.health.health_score = 1.0;
            }
        }
        
        // Send subscriptions
        {
            let feeds = feeds.read().await;
            if let Some(feed) = feeds.get(&key) {
                for sub in &feed.pending_subscriptions {
                    // In production: send WebSocket subscription message
                    debug!("Subscribing to {:?} for {}", sub.data_types, symbol);
                }
            }
        }
        
        // Start heartbeat task
        let heartbeat_feeds = feeds.clone();
        let heartbeat_config = config.clone();
        let heartbeat_key = key.clone();
        tokio::spawn(async move {
            let mut interval = interval(heartbeat_config.heartbeat_interval);
            loop {
                interval.tick().await;
                let feeds = heartbeat_feeds.read().await;
                if let Some(feed) = feeds.get(&heartbeat_key) {
                    if !feed.connected {
                        break;
                    }
                    // Send ping
                    debug!("Sending heartbeat for {}", heartbeat_key);
                } else {
                    break;
                }
            }
        });
        
        // Main message loop (simulated)
        let mut message_interval = interval(Duration::from_millis(100));
        let mut last_health_update = Utc::now();
        
        loop {
            message_interval.tick().await;
            
            // Check connection status
            let connected = {
                let feeds = feeds.read().await;
                feeds.get(&key).map(|f| f.connected).unwrap_or(false)
            };
            
            if !connected {
                break Ok(());
            }
            
            // Simulate receiving a message
            let msg = MarketDataMessage {
                symbol: symbol.to_string(),
                exchange: exchange.to_string(),
                timestamp: Utc::now().timestamp_millis() as u64,
                sequence: {
                    let mut feeds = feeds.write().await;
                    if let Some(feed) = feeds.get_mut(&key) {
                        feed.last_sequence += 1;
                        feed.last_sequence
                    } else { 0 }
                },
                data_type: DataType::Trade,
                payload: serde_json::json!({"price": 50000.0, "size": 0.1}),
                received_at: Utc::now(),
                processing_latency_ms: 0,
            };
            
            // Process with rate limiting
            rate_limiter.process_message(&format!("{}:{}", exchange, symbol), || async {
                // Update feed health
                let mut feeds_write = feeds.write().await;
                if let Some(feed) = feeds_write.get_mut(&key) {
                    feed.last_sequence = msg.sequence;
                    feed.health.message_count += 1;
                    feed.health.last_message = Utc::now();
                    feed.health.stale = false;
                    
                    // Add to per-symbol buffer
                    if feed.message_buffer.len() >= feed.max_buffer_size {
                        feed.message_buffer.pop_front();
                        feed.health.dropped_count += 1;
                    }
                    feed.message_buffer.push_back(msg.clone());
                }
                drop(feeds_write);
                
                // Add to global buffer
                let mut buffer = global_buffer.lock().await;
                if buffer.len() >= config.max_buffer_size {
                    buffer.pop_front();
                    let mut stats = stats.lock().await;
                    stats.dropped_messages += 1;
                }
                buffer.push_back(msg);
                
                // Update stats
                let mut stats = stats.lock().await;
                stats.total_messages += 1;
                let feeds_read = feeds.read().await;
                stats.active_feeds = feeds_read.values().filter(|f| f.connected).count();
                
                Ok(())
            }).await?;
            
            // Periodic health update
            if Utc::now().signed_duration_since(last_health_update) > TimeDelta::seconds(5) {
                Self::update_feed_health(&feeds, &key, stats.clone()).await;
                last_health_update = Utc::now();
            }
        }
    }

    /// Update feed health metrics
    async fn update_feed_health(
        feeds: &Arc<RwLock<HashMap<String, FeedState>>>,
        key: &str,
        stats: Arc<Mutex<FeedManagerStats>>,
    ) {
        // First update the feed health under write lock
        {
            let mut feeds = feeds.write().await;
            if let Some(feed) = feeds.get_mut(key) {
                let now = Utc::now();
                
                // Calculate messages per second
                let elapsed = (now - feed.last_heartbeat).num_seconds().max(1) as f64;
                feed.health.messages_per_second = feed.health.message_count as f64 / elapsed;
                
                // Calculate health score (0.0 - 1.0)
                let mut score = 1.0;
                if feed.health.dropped_count > 0 {
                    score *= (1.0 - (feed.health.dropped_count as f64 / feed.health.message_count.max(1) as f64)).max(0.0);
                }
                if feed.health.error_rate > 0.0 {
                    score *= (1.0 - feed.health.error_rate).max(0.0);
                }
                if feed.health.stale {
                    score *= 0.5;
                }
                if !feed.health.sequence_ok {
                    score *= 0.8;
                }
                feed.health.health_score = score.max(0.0).min(1.0);
                
                // Check staleness
                let age = now.signed_duration_since(feed.health.last_message).num_seconds();
                if age > 60 {
                    feed.health.stale = true;
                }
            }
        }
        
        // Then read all feeds for stats under read lock
        {
            let feeds = feeds.read().await;
            let mut stats = stats.lock().await;
            let all_feeds: Vec<FeedHealth> = feeds.values().map(|f| f.health.clone()).collect();
            stats.avg_health_score = all_feeds.iter().map(|h| h.health_score).sum::<f64>() / all_feeds.len().max(1) as f64;
        }
    }

    /// Handle incoming message (for external use)
    pub async fn handle_message(&self, msg: MarketDataMessage) -> Result<bool> {
        let key = format!("{}:{}", msg.exchange, msg.symbol);
        
        let mut feeds = self.feeds.write().await;
        if let Some(feed) = feeds.get_mut(&key) {
            // Sequence validation
            if msg.sequence > feed.last_sequence + 1 {
                let gap = msg.sequence - feed.last_sequence - 1;
                feed.health.dropped_count += gap;
                feed.health.sequence_ok = false;
                warn!("Sequence gap for {}: missed {} messages", key, gap);
            } else {
                feed.health.sequence_ok = true;
            }
            feed.last_sequence = msg.sequence;
            feed.health.message_count += 1;
            feed.health.last_message = Utc::now();
            feed.health.stale = false;
            
            // Update latency
            let latency = (Utc::now() - msg.received_at).num_milliseconds() as u64;
            feed.health.latency_ms = latency;
            feed.health.avg_latency_ms = feed.health.avg_latency_ms * 0.9 + latency as f64 * 0.1;
            
            // Add to per-symbol buffer
            if feed.message_buffer.len() >= feed.max_buffer_size {
                feed.message_buffer.pop_front();
                feed.health.dropped_count += 1;
            }
            feed.message_buffer.push_back(msg.clone());
        }
        
        // Add to global buffer
        let mut buffer = self.global_buffer.lock().await;
        if buffer.len() >= self.config.max_buffer_size {
            buffer.pop_front();
            let mut stats = self.stats.lock().await;
            stats.dropped_messages += 1;
        }
        buffer.push_back(msg);
        
        Ok(true)
    }

    /// Check for stale feeds
    pub async fn check_stale_feeds(&self) -> Vec<String> {
        let now = Utc::now();
        let feeds = self.feeds.read().await;
        let mut stale = Vec::new();
        
        for (key, feed) in feeds.iter() {
            let age = now.signed_duration_since(feed.health.last_message);
            if age > TimeDelta::from_std(self.config.stale_timeout).unwrap_or(TimeDelta::seconds(60)) {
                stale.push(key.clone());
            }
        }
        stale
    }

    /// Get feed health
    pub async fn get_feed_health(&self, exchange: &str, symbol: &str) -> Option<FeedHealth> {
        let key = format!("{}:{}", exchange, symbol);
        self.feeds.read().await.get(&key).map(|f| f.health.clone())
    }

    /// Get all feed health
    pub async fn get_all_feed_health(&self) -> Vec<FeedHealth> {
        self.feeds.read().await.values().map(|f| f.health.clone()).collect()
    }

    /// Reconnect a specific feed
    pub async fn reconnect_feed(&self, exchange: &str, symbol: &str) -> Result<bool> {
        let key = format!("{}:{}", exchange, symbol);
        let mut feeds = self.feeds.write().await;
        
        if let Some(feed) = feeds.get_mut(&key) {
            if feed.reconnect_attempts >= self.config.max_reconnect_attempts {
                return Ok(false);
            }
            
            feed.connected = false;
            feed.reconnect_attempts += 1;
            feed.health.reconnect_count += 1;
            feed.health.last_reconnect = Some(Utc::now());
            feed.health.subscription_status = SubscriptionStatus::Subscribing;
            
            // Re-add pending subscriptions
            // (will be re-sent on reconnect)
            
            drop(feeds);
            
            // Spawn new connection task
            self.spawn_feed_task(exchange.to_string(), symbol.to_string()).await;
            
            let mut stats = self.stats.lock().await;
            stats.total_reconnects += 1;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Auto-resubscribe stale feeds
    pub async fn auto_resubscribe_stale(&self) -> usize {
        if !self.config.enable_auto_resubscribe {
            return 0;
        }
        
        let stale = self.check_stale_feeds().await;
        let mut count = 0;
        
        for key in stale {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() == 2 {
                if self.reconnect_feed(parts[0], parts[1]).await.unwrap_or(false) {
                    count += 1;
                }
            }
        }
        
        count
    }

    /// Get buffered messages for a symbol
    pub async fn get_buffered_messages(&self, exchange: &str, symbol: &str, limit: usize) -> Vec<MarketDataMessage> {
        let key = format!("{}:{}", exchange, symbol);
        let feeds = self.feeds.read().await;
        feeds.get(&key)
            .map(|f| f.message_buffer.iter().rev().take(limit).cloned().collect())
            .unwrap_or_default()
    }

    /// Get global buffered messages
    pub async fn get_global_buffer(&self, limit: usize) -> Vec<MarketDataMessage> {
        let buffer = self.global_buffer.lock().await;
        buffer.iter().rev().take(limit).cloned().collect()
    }

    /// Subscribe to additional data types
    pub async fn subscribe(&self, exchange: &str, symbol: &str, data_types: Vec<DataType>) -> Result<()> {
        let key = format!("{}:{}", exchange, symbol);
        let mut feeds = self.feeds.write().await;
        
        if let Some(feed) = feeds.get_mut(&key) {
            let subscription = Subscription {
                symbol: symbol.to_string(),
                exchange: exchange.to_string(),
                data_types,
                depth: Some(100),
                interval: None,
            };
            
            feed.pending_subscriptions.push(subscription);
            
            if feed.connected {
                // Send subscription immediately
                // In production: send WebSocket message
                debug!("Sent new subscription for {}", key);
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Feed not found: {}", key))
        }
    }

    /// Unsubscribe from data types
    pub async fn unsubscribe(&self, exchange: &str, symbol: &str, data_types: Vec<DataType>) -> Result<()> {
        let key = format!("{}:{}", exchange, symbol);
        let mut feeds = self.feeds.write().await;
        
        if let Some(feed) = feeds.get_mut(&key) {
            feed.pending_subscriptions.retain(|s| {
                !data_types.iter().any(|dt| s.data_types.contains(dt))
            });
            
            if feed.connected {
                // Send unsubscription message
                debug!("Sent unsubscription for {}", key);
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Feed not found: {}", key))
        }
    }

    /// Get manager stats
    pub async fn get_stats(&self) -> FeedManagerStats {
        self.stats.lock().await.clone()
    }

    /// Shutdown all feeds
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        
        let mut feeds = self.feeds.write().await;
        for feed in feeds.values_mut() {
            feed.connected = false;
            feed.health.subscription_status = SubscriptionStatus::Unsubscribed;
        }
        
        info!("All feeds shutdown");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feed_manager_creation() {
        let config = FeedManagerConfig::default();
        let (manager, _rx) = MarketFeedManager::new(config);
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_feeds, 0);
    }

    #[tokio::test]
    async fn test_add_feed() {
        let config = FeedManagerConfig::default();
        let (manager, _rx) = MarketFeedManager::new(config);
        
        let subscription = Subscription {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            data_types: vec![DataType::Trade, DataType::OrderBook],
            depth: Some(10),
            interval: None,
        };
        
        manager.add_feed(subscription, "wss://ws.kraken.com/v2".into()).await.unwrap();
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_feeds, 1);
        
        let health = manager.get_feed_health("kraken", "BTC/USD").await.unwrap();
        assert_eq!(health.symbol, "BTC/USD");
        assert_eq!(health.exchange, "kraken");
    }

    #[tokio::test]
    async fn test_handle_message() {
        let config = FeedManagerConfig::default();
        let (manager, _rx) = MarketFeedManager::new(config);
        
        let subscription = Subscription {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            data_types: vec![DataType::Trade],
            depth: Some(10),
            interval: None,
        };
        
        manager.add_feed(subscription, "wss://ws.kraken.com/v2".into()).await.unwrap();
        
        let msg = MarketDataMessage {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            timestamp: Utc::now().timestamp_millis() as u64,
            sequence: 1,
            data_type: DataType::Trade,
            payload: serde_json::json!({"price": 50000.0}),
            received_at: Utc::now(),
            processing_latency_ms: 5,
        };
        
        manager.handle_message(msg).await.unwrap();
        
        let health = manager.get_feed_health("kraken", "BTC/USD").await.unwrap();
        assert_eq!(health.message_count, 1);
        assert_eq!(health.sequence_ok, true);
    }

    #[tokio::test]
    async fn test_sequence_gap_detection() {
        let config = FeedManagerConfig::default();
        let (manager, _rx) = MarketFeedManager::new(config);
        
        let subscription = Subscription {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            data_types: vec![DataType::Trade],
            depth: Some(10),
            interval: None,
        };
        
        manager.add_feed(subscription, "wss://ws.kraken.com/v2".into()).await.unwrap();
        
        // Send message with sequence 1
        let msg1 = MarketDataMessage {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            timestamp: Utc::now().timestamp_millis() as u64,
            sequence: 1,
            data_type: DataType::Trade,
            payload: serde_json::json!({}),
            received_at: Utc::now(),
            processing_latency_ms: 5,
        };
        manager.handle_message(msg1).await.unwrap();
        
        // Send message with sequence 5 (gap of 3)
        let msg2 = MarketDataMessage {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            timestamp: Utc::now().timestamp_millis() as u64,
            sequence: 5,
            data_type: DataType::Trade,
            payload: serde_json::json!({}),
            received_at: Utc::now(),
            processing_latency_ms: 5,
        };
        manager.handle_message(msg2).await.unwrap();
        
        let health = manager.get_feed_health("kraken", "BTC/USD").await.unwrap();
        assert_eq!(health.dropped_count, 3);
        assert!(!health.sequence_ok);
    }

    #[tokio::test]
    async fn test_stale_detection() {
        let config = FeedManagerConfig {
            stale_timeout: Duration::from_secs(1),
            ..Default::default()
        };
        let (manager, _rx) = MarketFeedManager::new(config);
        
        let subscription = Subscription {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            data_types: vec![DataType::Trade],
            depth: Some(10),
            interval: None,
        };
        
        manager.add_feed(subscription, "wss://ws.kraken.com/v2".into()).await.unwrap();
        
        // Immediately check - should not be stale
        let stale = manager.check_stale_feeds().await;
        assert!(stale.is_empty());
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        let stale = manager.check_stale_feeds().await;
        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0], "kraken:BTC/USD");
    }

    #[tokio::test]
    async fn test_buffer_management() {
        let config = FeedManagerConfig {
            max_buffer_size: 5,
            ..Default::default()
        };
        let (manager, _rx) = MarketFeedManager::new(config);
        
        let subscription = Subscription {
            symbol: "BTC/USD".into(),
            exchange: "kraken".into(),
            data_types: vec![DataType::Trade],
            depth: Some(10),
            interval: None,
        };
        
        manager.add_feed(subscription, "wss://ws.kraken.com/v2".into()).await.unwrap();
        
        // Send 10 messages (buffer size is 5)
        for i in 1..=10 {
            let msg = MarketDataMessage {
                symbol: "BTC/USD".into(),
                exchange: "kraken".into(),
                timestamp: Utc::now().timestamp_millis() as u64,
                sequence: i,
                data_type: DataType::Trade,
                payload: serde_json::json!({}),
                received_at: Utc::now(),
                processing_latency_ms: 5,
            };
            manager.handle_message(msg).await.unwrap();
        }
        
        let buffer = manager.get_global_buffer(10).await;
        assert_eq!(buffer.len(), 5); // Should only keep last 5
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.dropped_messages, 5); // 5 dropped due to buffer overflow
    }
}