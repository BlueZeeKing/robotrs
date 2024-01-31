use std::time::Duration;

use crate::time::get_time;

pub trait Filter {
    fn apply_with_time(&mut self, value: f32, time: Duration) -> f32;

    fn apply(&mut self, value: f32) -> crate::error::Result<f32> {
        Ok(self.apply_with_time(value, get_time()?))
    }
}

pub struct SlewRateLimiter {
    last_val: f32,
    last_time: Duration,
    limit: f32,
}

impl SlewRateLimiter {
    pub fn new(limit: f32) -> crate::error::Result<Self> {
        Ok(Self {
            last_val: 0.0,
            last_time: get_time()?,
            limit,
        })
    }

    pub fn set_limit(&mut self, limit: f32) {
        self.limit = limit
    }
}

impl Filter for SlewRateLimiter {
    fn apply_with_time(&mut self, value: f32, time: Duration) -> f32 {
        let new_val = if (self.last_val - value).abs() < self.limit {
            value
        } else if self.last_val < value {
            self.last_val + self.limit * time.as_secs_f32()
        } else {
            self.last_val - self.limit * time.as_secs_f32()
        };

        self.last_val = new_val;
        self.last_time = time;

        new_val
    }
}
