use std::time::Duration;

use crate::time::delay;
use futures::{select, Future, FutureExt};

pub mod axis;
pub mod button;
pub mod controller;
pub mod joystick;
mod reactor;

pub trait PressTrigger: Future<Output = Result<Self::Release, crate::error::Error>> {
    type Release: ReleaseTrigger;
}
pub trait ReleaseTrigger: Future<Output = Result<(), crate::error::Error>> {}

pub trait DoubleClick<T>: Sized {
    async fn double_click_with_duration(self, duration: Duration) -> T;

    async fn double_click(self) -> T {
        self.double_click_with_duration(Duration::from_millis(500))
            .await
    }
}

impl<Rt, T> DoubleClick<Result<Rt, crate::error::Error>> for T
where
    Rt: ReleaseTrigger,
    T: PressTrigger<Release = Rt> + Clone,
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
