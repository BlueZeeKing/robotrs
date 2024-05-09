use std::time::Duration;

use crate::get_time;

pub trait Filter {
    fn apply_with_time(&mut self, value: f32, time: Duration) -> f32;

    fn apply(&mut self, value: f32) -> f32 {
        self.apply_with_time(value, get_time())
    }
}

pub struct SlewRateLimiter {
    last_val: f32,
    last_time: Duration,
    limit: f32,
}

impl SlewRateLimiter {
    pub fn new(limit: f32) -> Self {
        Self {
            last_val: 0.0,
            last_time: get_time(),
            limit,
        }
    }

    pub fn set_limit(&mut self, limit: f32) {
        self.limit = limit
    }
}

impl Filter for SlewRateLimiter {
    fn apply_with_time(&mut self, value: f32, time: Duration) -> f32 {
        let delta_time = time - self.last_time;
        let max_delta_position = self.limit * delta_time.as_secs_f32();
        let delta_position = (self.last_val - value).abs();

        let new_val = if delta_position <= max_delta_position {
            value
        } else if self.last_val < value {
            self.last_val + self.limit * delta_time.as_secs_f32()
        } else {
            self.last_val - self.limit * delta_time.as_secs_f32()
        };

        self.last_val = new_val;
        self.last_time = time;

        new_val
    }
}
