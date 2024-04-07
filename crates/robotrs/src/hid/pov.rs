use std::{marker::PhantomData, pin::Pin, task::Poll};

use futures::Future;
use hal_sys::HAL_JoystickPOVs;

use crate::{error::Result, queue_waker};

use super::{
    button::{Pressed, Released},
    joystick::Joystick,
    reactor::add_pov,
};

pub(super) fn get_pov(povs: &HAL_JoystickPOVs, index: u32) -> Result<i16> {
    if index as i16 >= povs.count {
        return Err(crate::error::Error::PovIndexOutOfRange(index));
    }

    Ok(povs.povs[index as usize])
}

#[derive(Clone)]
pub struct PovFuture<T: Clone> {
    joystick: Joystick,
    pov_index: u32,
    phantom: PhantomData<T>,
    direction: i16,
    run: bool,
}

impl PovFuture<Pressed> {
    pub fn released(&self) -> PovFuture<Released> {
        PovFuture {
            joystick: self.joystick,
            pov_index: self.pov_index,
            phantom: PhantomData,
            direction: self.direction,
            run: false,
        }
    }
}

impl PovFuture<Released> {
    pub fn pressed(&self) -> PovFuture<Pressed> {
        PovFuture {
            joystick: self.joystick,
            pov_index: self.pov_index,
            phantom: PhantomData,
            direction: self.direction,
            run: false,
        }
    }
}

impl<T: Clone> PovFuture<T> {
    pub(super) fn new(joystick: Joystick, pov_index: u32, direction: i16) -> Self {
        PovFuture {
            joystick,
            pov_index,
            direction,
            phantom: PhantomData,
            run: false,
        }
    }

    pub fn value(&self) -> Result<i16> {
        get_pov(&self.joystick.get_pov_data()?, self.pov_index)
    }

    fn poll(&mut self) -> Result<bool> {
        let value = get_pov(&self.joystick.get_pov_data()?, self.pov_index)?;

        Ok(value == self.direction)
    }
}

impl super::PressTrigger for PovFuture<Pressed> {
    type Release = PovFuture<Released>;
}
impl super::ReleaseTrigger for PovFuture<Released> {}

impl Future for PovFuture<Pressed> {
    type Output = Result<PovFuture<Released>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let button_val = match data.poll() {
            Ok(val) => val,
            Err(err) => {
                return if data.run {
                    Poll::Ready(Err(err))
                } else {
                    queue_waker(cx.waker().clone());
                    Poll::Pending
                };
            }
        };

        data.run = true;

        if button_val {
            Poll::Ready(Ok(data.released()))
        } else {
            add_pov(
                &data.joystick,
                data.pov_index,
                data.direction,
                true,
                cx.waker().clone(),
            );
            Poll::Pending
        }
    }
}

impl Future for PovFuture<Released> {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let button_val = match data.poll() {
            Ok(val) => val,
            Err(err) => {
                return if data.run {
                    Poll::Ready(Err(err))
                } else {
                    queue_waker(cx.waker().clone());
                    Poll::Pending
                };
            }
        };

        data.run = true;

        if !button_val {
            Poll::Ready(Ok(()))
        } else {
            add_pov(
                &data.joystick,
                data.pov_index,
                data.direction,
                false,
                cx.waker().clone(),
            );
            Poll::Pending
        }
    }
}
