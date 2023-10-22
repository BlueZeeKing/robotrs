use std::time::Duration;

use crate::time::get_time;

pub trait Filter {
    fn apply_with_time(&mut self, value: f64, time: Duration) -> f64;

    fn apply(&mut self, value: f64) -> crate::error::Result<f64> {
        Ok(self.apply_with_time(value, get_time()?))
    }
}

pub struct SlewRateLimiter {
    last_val: f64,
    last_time: Duration,
    limit: f64,
}

impl SlewRateLimiter {
    pub fn new(limit: f64) -> crate::error::Result<Self> {
        Ok(Self {
            last_val: 0.0,
            last_time: get_time()?,
            limit,
        })
    }

    pub fn set_limit(&mut self, limit: f64) {
        self.limit = limit
    }
}

impl Filter for SlewRateLimiter {
    fn apply_with_time(&mut self, value: f64, time: Duration) -> f64 {
        let new_val = if self.last_val < value {
            self.last_val + self.limit * time.as_secs_f64()
        } else {
            self.last_val - self.limit * time.as_secs_f64()
        };

        self.last_val = new_val;
        self.last_time = time;

        new_val
    }
}
