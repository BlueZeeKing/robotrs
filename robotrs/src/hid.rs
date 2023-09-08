use std::time::Duration;

use crate::time::delay;
use async_trait::async_trait;
use futures::{select, Future, FutureExt};

pub mod axis;
pub mod button;
pub mod controller;
pub mod joystick;
mod reactor;

pub trait PressTrigger<T: ReleaseTrigger>: Future<Output = Result<T, crate::error::Error>> {}
pub trait ReleaseTrigger: Future<Output = Result<(), crate::error::Error>> {}

#[async_trait(?Send)]
pub trait DoubleClick<T>: Sized {
    async fn double_click_with_duration(self, duration: Duration) -> T;

    async fn double_click(self) -> T {
        self.double_click_with_duration(Duration::from_millis(500))
            .await
    }
}

#[async_trait(?Send)]
impl<Rt, T> DoubleClick<Result<Rt, crate::error::Error>> for T
where
    Rt: ReleaseTrigger,
    T: PressTrigger<Rt> + Clone,
{
    async fn double_click_with_duration(
        self,
        duration: Duration,
    ) -> Result<Rt, crate::error::Error> {
        loop {
            let release = self.clone().await?;
            release.await?;

            select! {
                button = self.clone().fuse() => {
                    return button;
                }
                alarm = delay(duration).fuse() => {
                    alarm?;
                }
            }
        }
    }
}
