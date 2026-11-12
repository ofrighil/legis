use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Accept,
    RetryAfter(Duration),
    Fail,
}

#[derive(Clone, Copy, Debug)]
pub struct RetryPolicy {
    pub delay: Duration,
    pub max_retries: u8,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            delay: Duration::from_secs(5),
            max_retries: 1,
        }
    }
}

impl RetryPolicy {
    pub fn decide(&self, status: u16, retries: u8) -> Decision {
        match status {
            200..=299 => Decision::Accept,
            429 | 500..=599 if retries < self.max_retries => Decision::RetryAfter(self.delay),
            _ => Decision::Fail,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pacer {
    min_interval: Duration,
    last: Option<Instant>,
}

impl Default for Pacer {
    fn default() -> Self {
        Self::new(Duration::from_millis(750))
    }
}

impl Pacer {
    pub fn new(min_interval: Duration) -> Self {
        Self {
            min_interval,
            last: None,
        }
    }

    pub fn delay(&mut self, now: Instant) -> Duration {
        let wait = self.last.map_or(Duration::ZERO, |last| {
            (last + self.min_interval).saturating_duration_since(now)
        });
        self.last = Some(now + wait);
        wait
    }
}
