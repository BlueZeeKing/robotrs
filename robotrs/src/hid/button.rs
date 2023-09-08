use std::{marker::PhantomData, pin::Pin, task::Poll};

use futures::Future;
use hal_sys::HAL_JoystickButtons;

use crate::error::Result;

use super::{joystick::Joystick, reactor::add_button};

pub(super) fn get_button(buttons: &HAL_JoystickButtons, index: u32) -> Result<bool> {
    if index >= buttons.count.into() {
        return Err(crate::error::Error::ButtonIndexOutOfRange(index));
    }

    Ok(buttons.buttons & (1 >> index) > 0)
}

#[derive(Clone)]
pub struct Pressed;
#[derive(Clone)]
pub struct Released;

#[derive(Clone)]
pub struct ButtonFuture<T: Clone> {
    joystick_index: u32,
    button_index: u32,
    phantom: PhantomData<T>,
}

impl ButtonFuture<Pressed> {
    pub fn released(&self) -> ButtonFuture<Released> {
        ButtonFuture {
            joystick_index: self.joystick_index,
            button_index: self.button_index,
            phantom: PhantomData,
        }
    }
}

impl ButtonFuture<Released> {
    pub fn pressed(&self) -> ButtonFuture<Pressed> {
        ButtonFuture {
            joystick_index: self.joystick_index,
            button_index: self.button_index,
            phantom: PhantomData,
        }
    }
}

impl<T: Clone> ButtonFuture<T> {
    pub fn new(joystick_index: u32, button_index: u32) -> Self {
        ButtonFuture {
            joystick_index,
            button_index,
            phantom: PhantomData,
        }
    }

    pub fn poll(&mut self) -> Result<(Joystick, bool)> {
        let joystick = Joystick::new(self.joystick_index)?;

        let value = get_button(&joystick.get_button_data()?, self.button_index)?;

        Ok((joystick, value))
    }
}

impl super::PressTrigger<ButtonFuture<Released>> for ButtonFuture<Pressed> {}
impl super::ReleaseTrigger for ButtonFuture<Released> {}

impl Future for ButtonFuture<Pressed> {
    type Output = Result<ButtonFuture<Released>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let (joystick, button_val) = match data.poll() {
            Ok(val) => val,
            Err(err) => {
                return Poll::Ready(Err(err));
            }
        };

        if button_val {
            Poll::Ready(Ok(data.released()))
        } else {
            add_button(&joystick, data.button_index, true, cx.waker().clone());
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

        let (joystick, button_val) = match data.poll() {
            Ok(val) => val,
            Err(err) => {
                return Poll::Ready(Err(err));
            }
        };

        if !button_val {
            Poll::Ready(Ok(()))
        } else {
            add_button(&joystick, data.button_index, false, cx.waker().clone());
            Poll::Pending
        }
    }
}
