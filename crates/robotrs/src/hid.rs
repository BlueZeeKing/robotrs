use std::future::Future;

pub mod all;
pub mod any;
pub mod axis;
pub mod button;
pub mod controller;
pub mod ext;
pub mod joystick;
pub mod pov;
mod reactor;

/// A generic async trigger
pub trait Trigger {
    type Error;
    type Output;

    /// Wait for the rising edge of the trigger. This returns early if an error occurs
    fn wait_for_trigger(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

pub trait ReleaseTrigger: Trigger {
    /// Wait for the falling edge of the trigger. This returns early if an error occurs
    fn wait_for_release(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
