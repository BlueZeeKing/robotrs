use std::{marker::PhantomData, pin::Pin, task::Poll};

use futures::Future;
use hal_sys::HAL_JoystickButtons;
use tracing::error;

use crate::{error::Result, queue_waker};

use super::{joystick::Joystick, reactor::add_button};

pub(super) fn get_button(buttons: &HAL_JoystickButtons, index: u32) -> Result<bool> {
    if index >= buttons.count.into() {
        return Err(crate::error::Error::ButtonIndexOutOfRange(index));
    }

    Ok(buttons.buttons & (1 << index) > 0)
}

#[derive(Clone)]
pub struct Pressed;
#[derive(Clone)]
pub struct Released;

#[derive(Clone)]
pub struct ButtonFuture<T: Clone> {
    joystick: Joystick,
    button_index: u32,
    phantom: PhantomData<T>,
    run: bool,
}

impl ButtonFuture<Pressed> {
    pub fn released(&self) -> ButtonFuture<Released> {
        ButtonFuture {
            joystick: self.joystick,
            button_index: self.button_index,
            phantom: PhantomData,
            run: false,
        }
    }
}

impl ButtonFuture<Released> {
    pub fn pressed(&self) -> ButtonFuture<Pressed> {
        ButtonFuture {
            joystick: self.joystick,
            button_index: self.button_index,
            phantom: PhantomData,
            run: false,
        }
    }
}

impl<T: Clone> ButtonFuture<T> {
    pub(super) fn new(joystick: Joystick, button_index: u32) -> Self {
        ButtonFuture {
            joystick,
            button_index,
            phantom: PhantomData,
            run: false,
        }
    }

    pub fn value(&self) -> Result<bool> {
        get_button(&self.joystick.get_button_data()?, self.button_index)
    }

    fn poll(&mut self) -> Result<bool> {
        let value = get_button(&self.joystick.get_button_data()?, self.button_index)?;

        Ok(value)
    }
}

impl super::PressTrigger for ButtonFuture<Pressed> {
    type Release = ButtonFuture<Released>;
}
impl super::ReleaseTrigger for ButtonFuture<Released> {}

impl Future for ButtonFuture<Pressed> {
    type Output = Result<ButtonFuture<Released>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let button_val = match data.poll() {
            Ok(val) => val,
            Err(err) => {
                return if data.run {
                    error!("Rerun");
                    Poll::Ready(Err(err))
                } else {
                    error!("Error");
                    queue_waker(cx.waker().clone());
                    Poll::Pending
                }
            }
        };

        data.run = true;

        if button_val {
            Poll::Ready(Ok(data.released()))
        } else {
            add_button(&data.joystick, data.button_index, true, cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Future for ButtonFuture<Released> {
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
                    error!("Rerun");
                    Poll::Ready(Err(err))
                } else {
                    error!("Error");
                    queue_waker(cx.waker().clone());
                    Poll::Pending
                }
            }
        };

        data.run = true;

        if !button_val {
            Poll::Ready(Ok(()))
        } else {
            add_button(&data.joystick, data.button_index, false, cx.waker().clone());
            Poll::Pending
        }
    }
}
