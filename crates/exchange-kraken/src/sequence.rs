// Kraken WebSocket sequence validation, gap detection, and late-event rejection.
use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    Gap { expected: u64, got: u64 },
    LateEvent { max_lateness_ms: u64, lateness_ms: u64 },
    Duplicate { seq: u64 },
}

#[derive(Debug, Clone, Default)]
pub struct SequenceValidator {
    last_seq: Option<u64>,
    last_processed_time: Option<u64>,
    dropped: u64,
    gaps: u64,
    max_lateness_ms: u64,
}

impl SequenceValidator {
    pub fn new(max_lateness_ms: u64) -> Self {
        Self { max_lateness_ms, ..Default::default() }
    }

    pub fn validate(&mut self, seq: u64, ts: u64) -> Result<(), ValidationError> {
        // Check for duplicate sequence
        if let Some(last) = self.last_seq {
            if seq == last {
                return Err(ValidationError::Duplicate { seq });
            }
        }

        // Check for gap
        if let Some(last) = self.last_seq {
            if seq != last + 1 {
                self.gaps = self.gaps.saturating_add(1);
                return Err(ValidationError::Gap { expected: last + 1, got: seq });
            }
        }

        // Check for late event
        if let Some(last_ts) = self.last_processed_time {
            if ts < last_ts {
                let lateness = last_ts.saturating_sub(ts);
                if lateness > self.max_lateness_ms {
                    self.dropped = self.dropped.saturating_add(1);
                    return Err(ValidationError::LateEvent { max_lateness_ms: self.max_lateness_ms, lateness_ms: lateness });
                }
            }
        }

        self.last_seq = Some(seq);
        self.last_processed_time = Some(ts);
        Ok(())
    }

    pub fn metrics(&self) -> (u64, u64, u64) {
        (self.dropped, self.gaps, self.last_seq.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_detection() {
        let mut v = SequenceValidator::new(1000);
        assert!(v.validate(1, 1000).is_ok());
        assert!(v.validate(2, 1001).is_ok());
        let err = v.validate(4, 1003).unwrap_err();
        assert!(matches!(err, ValidationError::Gap { expected: 3, got: 4 }));
    }

    #[test]
    fn late_event_rejection() {
        let mut v = SequenceValidator::new(100);
        assert!(v.validate(1, 1000).is_ok());
        assert!(v.validate(2, 1001).is_ok());
        let err = v.validate(3, 800).unwrap_err();
        assert!(matches!(err, ValidationError::LateEvent { .. }));
    }

    #[test]
    fn duplicate_rejection() {
        let mut v = SequenceValidator::new(1000);
        assert!(v.validate(1, 1000).is_ok());
        let err = v.validate(1, 1001).unwrap_err();
        assert!(matches!(err, ValidationError::Duplicate { seq: 1 }));
    }
}