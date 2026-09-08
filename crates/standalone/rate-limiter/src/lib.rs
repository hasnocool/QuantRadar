// QuantRadar adaptive rate limiter for API and WebSocket connections.
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore};
use tracing::{debug, warn};

/// Rate limiter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterConfig {
    pub max_requests_per_second: f64,
    pub burst_size: usize,
    pub adaptive: bool,
    pub min_rate: f64,
    pub max_rate: f64,
    pub adaptation_window_seconds: u64,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests_per_second: 10.0,
            burst_size: 20,
            adaptive: true,
            min_rate: 1.0,
            max_rate: 100.0,
            adaptation_window_seconds: 60,
        }
    }
}

/// Token bucket state
#[derive(Debug, Clone)]
struct TokenBucketState {
    capacity: f64,
    tokens: f64,
    refill_rate: f64,
    last_refill: DateTime<Utc>,
}

/// Token bucket rate limiter with interior mutability
#[derive(Clone)]
pub struct TokenBucket {
    state: Arc<Mutex<TokenBucketState>>,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            state: Arc::new(Mutex::new(TokenBucketState {
                capacity,
                tokens: capacity,
                refill_rate,
                last_refill: Utc::now(),
            })),
        }
    }

    /// Try to consume tokens, returns true if successful
    pub async fn try_consume(&self, tokens: f64) -> bool {
        let mut state = self.state.lock().await;
        state.refill();
        if state.tokens >= tokens {
            state.tokens -= tokens;
            true
        } else {
            false
        }
    }

    /// Wait until tokens are available
    pub async fn consume(&self, tokens: f64) {
        loop {
            if self.try_consume(tokens).await {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    pub async fn available_tokens(&self) -> f64 {
        let state = self.state.lock().await;
        // Note: this doesn't refill, but gives a rough estimate
        state.tokens
    }

    pub async fn set_rate(&self, rate: f64) {
        let mut state = self.state.lock().await;
        state.refill_rate = rate;
    }
}

impl TokenBucketState {
    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).num_milliseconds() as f64 / 1000.0;
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = now;
    }
}

/// Adaptive rate limiter that adjusts based on response codes
pub struct AdaptiveRateLimiter {
    buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
    config: RateLimiterConfig,
    stats: Arc<Mutex<RateLimiterStats>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RateLimiterStats {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub denied_requests: u64,
    pub current_rate: f64,
    pub rate_adjustments: u64,
    pub last_adjustment: Option<DateTime<Utc>>,
    pub error_rates: HashMap<String, f64>,
}

impl AdaptiveRateLimiter {
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(RateLimiterStats::default())),
        }
    }

    /// Get or create a bucket for an endpoint
    async fn get_bucket(&self, key: &str) -> TokenBucket {
        let mut buckets = self.buckets.lock().await;
        buckets.entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(self.config.burst_size as f64, self.config.max_requests_per_second))
            .clone()
    }

    /// Try to acquire permission for a request
    pub async fn try_acquire(&self, endpoint: &str) -> bool {
        let bucket = self.get_bucket(endpoint).await;
        let allowed = bucket.try_consume(1.0).await;
        
        let mut stats = self.stats.lock().await;
        stats.total_requests += 1;
        if allowed {
            stats.allowed_requests += 1;
        } else {
            stats.denied_requests += 1;
        }
        allowed
    }

    /// Wait for permission (blocking)
    pub async fn acquire(&self, endpoint: &str) {
        let bucket = self.get_bucket(endpoint).await;
        bucket.consume(1.0).await;
        
        let mut stats = self.stats.lock().await;
        stats.total_requests += 1;
        stats.allowed_requests += 1;
    }

    /// Record response for adaptive rate adjustment
    pub async fn record_response(&self, endpoint: &str, status_code: u16, latency_ms: u64) {
        if !self.config.adaptive {
            return;
        }

        let mut stats = self.stats.lock().await;
        
        // Track error rates
        let error_rate = stats.error_rates.entry(endpoint.to_string()).or_insert(0.0);
        if status_code >= 400 {
            *error_rate = *error_rate * 0.9 + 0.1;
        } else {
            *error_rate = *error_rate * 0.9;
        }

        // Adjust rate based on error rate and latency
        if *error_rate > 0.1 || latency_ms > 1000 {
            // Decrease rate
            let new_rate = (self.config.max_requests_per_second * 0.8).max(self.config.min_rate);
            self.adjust_rate(new_rate).await;
            stats.rate_adjustments += 1;
            stats.last_adjustment = Some(Utc::now());
        } else if *error_rate < 0.01 && latency_ms < 200 {
            // Increase rate slightly
            let new_rate = (self.config.max_requests_per_second * 1.05).min(self.config.max_rate);
            self.adjust_rate(new_rate).await;
        }
    }

    async fn adjust_rate(&self, new_rate: f64) {
        // Clone bucket references while holding the lock briefly
        let bucket_refs: Vec<TokenBucket> = {
            let buckets = self.buckets.lock().await;
            buckets.values().cloned().collect()
        };
        
        // Update rates without holding the buckets lock
        for bucket in bucket_refs {
            bucket.set_rate(new_rate).await;
        }
        
        let mut stats = self.stats.lock().await;
        stats.current_rate = new_rate;
        
        debug!("Rate adjusted to {:.2} req/s", new_rate);
    }

    /// Get current stats
    pub async fn get_stats(&self) -> RateLimiterStats {
        self.stats.lock().await.clone()
    }

    /// Get current rate for an endpoint
    pub async fn get_rate(&self, endpoint: &str) -> f64 {
        let bucket = self.get_bucket(endpoint).await;
        bucket.available_tokens().await
    }
}

/// Semaphore-based concurrency limiter
pub struct ConcurrencyLimiter {
    semaphore: Arc<Semaphore>,
    max_concurrent: usize,
    active_count: Arc<Mutex<usize>>,
}

impl ConcurrencyLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
            active_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Acquire a permit (blocking)
    pub async fn acquire(&self) -> ConcurrencyPermit {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        let mut count = self.active_count.lock().await;
        *count += 1;
        ConcurrencyPermit {
            _permit: permit,
            active_count: self.active_count.clone(),
        }
    }

    /// Try to acquire a permit (non-blocking)
    pub fn try_acquire(&self) -> Option<ConcurrencyPermit> {
        if let Ok(permit) = self.semaphore.clone().try_acquire_owned() {
            let mut count = self.active_count.blocking_lock();
            *count += 1;
            Some(ConcurrencyPermit {
                _permit: permit,
                active_count: self.active_count.clone(),
            })
        } else {
            None
        }
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub fn active_count(&self) -> usize {
        *self.active_count.blocking_lock()
    }
}

/// RAII permit for concurrency limiting
pub struct ConcurrencyPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
    active_count: Arc<Mutex<usize>>,
}

impl Drop for ConcurrencyPermit {
    fn drop(&mut self) {
        let mut count = self.active_count.blocking_lock();
        *count = count.saturating_sub(1);
    }
}

/// Rate limiter for WebSocket message processing
#[derive(Clone)]
pub struct MessageRateLimiter {
    limiter: Arc<AdaptiveRateLimiter>,
    concurrency: Arc<ConcurrencyLimiter>,
}

impl MessageRateLimiter {
    pub fn new(rate_config: RateLimiterConfig, max_concurrent: usize) -> Self {
        Self {
            limiter: Arc::new(AdaptiveRateLimiter::new(rate_config)),
            concurrency: Arc::new(ConcurrencyLimiter::new(max_concurrent)),
        }
    }

    /// Process a message with rate limiting
    pub async fn process_message<F, Fut>(&self, endpoint: &str, f: F) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        // Rate limit
        self.limiter.acquire(endpoint).await;
        
        // Concurrency limit
        let _permit = self.concurrency.acquire().await;
        
        // Execute
        let start = std::time::Instant::now();
        let result = f().await;
        let latency_ms = start.elapsed().as_millis() as u64;
        
        // Record for adaptation
        let status = if result.is_ok() { 200 } else { 500 };
        self.limiter.record_response(endpoint, status, latency_ms).await;
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_bucket() {
        let bucket = TokenBucket::new(10.0, 5.0); // 10 capacity, 5 tokens/sec
        
        // Should be able to consume 10 immediately
        for _ in 0..10 {
            assert!(bucket.try_consume(1.0).await);
        }
        
        // Should fail when empty
        assert!(!bucket.try_consume(1.0).await);
        
        // Wait for refill
        tokio::time::sleep(Duration::from_millis(500)).await;
        assert!(bucket.try_consume(1.0).await);
    }

    #[tokio::test]
    async fn test_adaptive_rate_limiter() {
        let config = RateLimiterConfig {
            max_requests_per_second: 10.0,
            burst_size: 20,
            adaptive: true,
            ..Default::default()
        };
        let limiter = AdaptiveRateLimiter::new(config);
        
        // Should allow bursts up to burst_size
        for _ in 0..20 {
            assert!(limiter.try_acquire("test").await);
        }
        
        // Should deny after burst
        assert!(!limiter.try_acquire("test").await);
        
        // Record successful responses
        for _ in 0..10 {
            limiter.record_response("test", 200, 50).await;
        }
        
        let stats = limiter.get_stats().await;
        assert_eq!(stats.allowed_requests, 20);
        assert_eq!(stats.denied_requests, 1);
    }

    #[tokio::test]
    async fn test_concurrency_limiter() {
        let limiter = ConcurrencyLimiter::new(3);
        
        // Acquire 3 permits
        let p1 = limiter.acquire().await;
        let p2 = limiter.acquire().await;
        let p3 = limiter.acquire().await;
        
        assert_eq!(limiter.active_count(), 3);
        assert_eq!(limiter.available_permits(), 0);
        
        // Drop one
        drop(p1);
        assert_eq!(limiter.active_count(), 2);
    }
}