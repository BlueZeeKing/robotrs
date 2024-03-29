use std::{future::Future, task::Poll};

use futures::{join, select, FutureExt};
use pin_project::pin_project;
use robotrs::{
    hid::{PressTrigger, ReleaseTrigger},
    robot::AsyncRobot,
    scheduler::RobotScheduler,
};
use tracing::error;

pub trait TriggerExt: PressTrigger + Sized + Clone {
    fn on_press<'a, Func, Fut>(self, scheduler: &'a RobotScheduler<'a, impl AsyncRobot>, func: Func)
    where
        Func: Fn() -> Fut + 'a,
        Fut: Future,
    {
        scheduler
            .schedule(async move {
                loop {
                    let released = match self.clone().await {
                        Err(err) => {
                            error!("button failed in trigger: {:?}", err);
                            continue;
                        }
                        Ok(released) => released,
                    };

                    let (_, released) = join!(func(), released);

                    if let Err(err) = released {
                        error!("button failed in trigger: {:?}", err);
                    }
                }
            })
            .detach();
    }

    fn while_pressed<'a, Func, Fut>(
        self,
        scheduler: &'a RobotScheduler<'a, impl AsyncRobot>,
        func: Func,
    ) where
        Func: Fn() -> Fut + 'a,
        Fut: Future,
    {
        scheduler
            .schedule(async move {
                loop {
                    let released = match self.clone().await {
                        Err(err) => {
                            error!("Button failed in trigger: {:?}", err);
                            continue;
                        }
                        Ok(released) => released,
                    };

                    select! {
                        _ = func().fuse() => {},
                        _ = released.fuse() => {},
                    }
                }
            })
            .detach();
    }

    fn or<T: PressTrigger>(self, other: T) -> EitherTriggerPressed<Self, T> {
        EitherTriggerPressed {
            trigger1: self,
            trigger2: other,
        }
    }
}

impl<T: PressTrigger + Sized + Clone> TriggerExt for T {}

#[derive(Clone)]
#[pin_project]
pub struct EitherTriggerPressed<T1: PressTrigger, T2: PressTrigger> {
    #[pin]
    trigger1: T1,
    #[pin]
    trigger2: T2,
}

impl<T1: PressTrigger, T2: PressTrigger> Future for EitherTriggerPressed<T1, T2> {
    type Output = Result<EitherTriggerReleased<T1::Release, T2::Release>, robotrs::error::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let trigger = self.project();

        if let Poll::Ready(released) = trigger.trigger1.poll(cx) {
            Poll::Ready(released.map(|released| EitherTriggerReleased::First(released)))
        } else if let Poll::Ready(released) = trigger.trigger2.poll(cx) {
            Poll::Ready(released.map(|released| EitherTriggerReleased::Second(released)))
        } else {
            Poll::Pending
        }
    }
}

impl<T1: PressTrigger, T2: PressTrigger> PressTrigger for EitherTriggerPressed<T1, T2> {
    type Release = EitherTriggerReleased<T1::Release, T2::Release>;
}

#[derive(Clone)]
#[pin_project(project = EitherTriggerReleasedProjection)]
pub enum EitherTriggerReleased<T1: ReleaseTrigger, T2: ReleaseTrigger> {
    First(#[pin] T1),
    Second(#[pin] T2),
}

impl<T1: ReleaseTrigger, T2: ReleaseTrigger> Future for EitherTriggerReleased<T1, T2> {
    type Output = Result<(), robotrs::error::Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            EitherTriggerReleasedProjection::First(trigger) => trigger.poll(cx),
            EitherTriggerReleasedProjection::Second(trigger) => trigger.poll(cx),
        }
    }
}

impl<T1: ReleaseTrigger, T2: ReleaseTrigger> ReleaseTrigger for EitherTriggerReleased<T1, T2> {}
