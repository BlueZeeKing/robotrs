use std::time::Duration;

use crate::error::Result;

use super::{alarm::Alarm, get_time};

pub struct Periodic {
    period: Duration,
    end_time: Duration,
}

impl Periodic {
    pub fn new(period: Duration) -> Result<Self> {
        Ok(Self {
            period,
            end_time: get_time()? + period,
        })
    }

    pub async fn wait(&mut self) -> Result<()> {
        Alarm {
            end_time: Some(self.end_time),
            duration: self.period, // not actually used
        }
        .await?;

        self.end_time += self.period;

        Ok(())
    }
}
