use futures_lite::future::pending;
use std::{fmt::Debug, future::Future, time::Duration};
use tracing::error;

use futures_concurrency::future::Race;

use crate::{
    ds::{wait_for_disabled, wait_for_enabled},
    scheduler::{guard, spawn},
    time::delay,
};

use super::{ReleaseTrigger, Trigger};

enum ClickResult<T> {
    Clicked(T),
    Other,
}

/// A double click extension trait that is auto implemented for sized [Trigger]s
pub trait DoubleClickTarget: Trigger + Sized {
    #[doc(hidden)]
    async fn wait_for_double_click_with_duration(
        &mut self,
        duration: Duration,
    ) -> Result<Self::Output, Self::Error> {
        loop {
            self.wait_for_trigger().await?;

            let ClickResult::Clicked(value) = (
                async { ClickResult::Clicked(self.wait_for_trigger().await) },
                async {
                    delay(duration);
                    ClickResult::Other
                },
            )
                .race()
                .await
            else {
                continue;
            };

            return value;
        }
    }

    /// Create a new trigger that activates when this trigger is activated twice in 500
    /// milliseconds
    fn double_click(self) -> DoubleClick<Self> {
        self.double_click_with_duration(Duration::from_millis(500))
    }

    /// Create a new trigger that activates when this trigger is activated twice in a given amount
    /// of time
    fn double_click_with_duration(self, duration: Duration) -> DoubleClick<Self> {
        DoubleClick {
            inner: self,
            time: duration,
        }
    }
}

/// A trigger that activates when the trigger is activated twice. Created through
/// [DoubleClickTarget::double_click].
pub struct DoubleClick<T> {
    inner: T,
    time: Duration,
}

impl<T: DoubleClickTarget> Trigger for DoubleClick<T> {
    type Output = T::Output;
    type Error = T::Error;

    async fn wait_for_trigger(&mut self) -> Result<Self::Output, Self::Error> {
        self.inner
            .wait_for_double_click_with_duration(self.time)
            .await
    }
}

impl<T: ReleaseTrigger> ReleaseTrigger for DoubleClick<T> {
    async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
        self.inner.wait_for_release().await
    }
}

impl<T: Trigger> DoubleClickTarget for T {}

/// An extension trait that is automatically implemented for triggers that are ``static` and have a
/// `Trigger::Error` type that implements [Debug]
pub trait TriggerExt: Trigger + Sized
where
    Self::Error: Debug,
    Self: 'static,
{
    /// Spawns a new future using the main scheduler that waits for the trigger to activate then
    /// calls the function and runs the future. The function and future are wrapped in a
    /// cancellation scope so the main future will not be cancelled. This only runs if the robot is
    /// enabled
    fn on_pressed<Func, Fut>(mut self, mut func: Func)
    where
        Func: FnMut() -> Fut + 'static,
        Fut: Future + 'static,
    {
        spawn(async move {
            loop {
                wait_for_enabled().await;

                (
                    async {
                        loop {
                            if let Err(err) = self.wait_for_trigger().await {
                                error!("Trigger error: {:?}", err);
                            }

                            let _ = guard(func()).await;
                        }
                    },
                    wait_for_disabled(),
                )
                    .race()
                    .await;
            }
        })
        .detach();
    }
}

impl<T: Trigger> TriggerExt for T
where
    Self::Error: Debug,
    Self: 'static,
{
}

/// An extension trait that is automatically implemented for release triggers that are ``static` and have a
/// `Trigger::Error` type that implements [Debug]
pub trait ReleaseTriggerExt: ReleaseTrigger + Sized
where
    Self::Error: Debug,
    Self: 'static,
{
    /// Spawns a new future using the main scheduler that waits for the trigger to activate then
    /// calls the function and runs the future. The future is then cancelled when the trigger is
    /// released, if it is still running. The function and future are wrapped in a cancellation
    /// scope so the main future will not be cancelled. This only runs if the robot is enabled
    fn while_pressed<Func, Fut>(mut self, mut func: Func)
    where
        Func: FnMut() -> Fut + 'static,
        Fut: Future + 'static,
    {
        spawn(async move {
            loop {
                wait_for_enabled().await;

                (
                    async {
                        if let Err(err) = self.wait_for_trigger().await {
                            error!("button failed in trigger: {:?}", err);
                            return;
                        }

                        let res = (
                            async {
                                let _ = guard(func()).await;
                                pending::<()>().await;
                                unreachable!()
                            },
                            self.wait_for_release(),
                        )
                            .race()
                            .await;

                        if let Err(err) = res {
                            error!("button failed in trigger: {:?}", err);
                        }
                    },
                    wait_for_disabled(),
                )
                    .race()
                    .await;
            }
        })
        .detach();
    }
}

impl<T: ReleaseTrigger> ReleaseTriggerExt for T
where
    Self::Error: Debug,
    Self: 'static,
{
}
