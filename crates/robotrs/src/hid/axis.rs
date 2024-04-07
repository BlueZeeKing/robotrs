use std::{marker::PhantomData, pin::Pin, task::Poll};

use futures::Future;
use hal_sys::HAL_JoystickAxes;

use crate::{
    error::{Error, Result},
    queue_waker,
};

use super::{joystick::Joystick, reactor::add_axis};

pub(super) fn get_axis(data: &HAL_JoystickAxes, index: u32) -> Result<f32> {
    if index >= data.count as u32 {
        return Err(Error::AxisIndexOutOfRange(index));
    }

    Ok(data.axes[index as usize])
}

#[derive(Copy, Clone, Debug)]
pub enum AxisTarget {
    Away(f32),
    Up(f32),
    Down(f32),
}

impl AxisTarget {
    pub(super) fn is_active(&self, value: f32) -> bool {
        match self {
            AxisTarget::Away(dist) => value.abs() > *dist,
            AxisTarget::Down(target) => value < *target,
            AxisTarget::Up(target) => value > *target,
        }
    }
}

#[derive(Clone)]
pub struct Initial;
#[derive(Clone)]
pub struct Release;

#[derive(Clone)]
pub struct AxisFuture<T> {
    joystick_index: u32,
    axis_index: u32,
    target: AxisTarget,
    phantom: PhantomData<T>,
    run: bool,
}

impl AxisFuture<Initial> {
    pub fn release(&self) -> AxisFuture<Release> {
        AxisFuture {
            joystick_index: self.joystick_index,
            axis_index: self.axis_index,
            target: self.target,
            phantom: PhantomData,
            run: false,
        }
    }
}

impl AxisFuture<Release> {
    pub fn initial(&self) -> AxisFuture<Initial> {
        AxisFuture {
            joystick_index: self.joystick_index,
            axis_index: self.axis_index,
            target: self.target,
            phantom: PhantomData,
            run: false,
        }
    }
}

impl<T> AxisFuture<T> {
    pub fn new(joystick_index: u32, axis_index: u32, target: AxisTarget) -> Self {
        Self {
            joystick_index,
            axis_index,
            target,
            phantom: PhantomData,
            run: false,
        }
    }

    fn poll(&mut self) -> Result<(Joystick, bool)> {
        let joystick = Joystick::new(self.joystick_index)?;

        let value = self
            .target
            .is_active(get_axis(&joystick.get_axes_data()?, self.axis_index)?);

        Ok((joystick, value))
    }
}

impl super::PressTrigger for AxisFuture<Initial> {
    type Release = AxisFuture<Release>;
}
impl super::ReleaseTrigger for AxisFuture<Release> {}

impl Future for AxisFuture<Initial> {
    type Output = Result<AxisFuture<Release>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let (joystick, val) = match data.poll() {
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

        if val {
            Poll::Ready(Ok(data.release()))
        } else {
            add_axis(
                &joystick,
                data.axis_index,
                true,
                data.target,
                cx.waker().clone(),
            );
            Poll::Pending
        }
    }
}

impl Future for AxisFuture<Release> {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let data = Pin::into_inner(self);

        let (joystick, val) = match data.poll() {
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

        if !val {
            Poll::Ready(Ok(()))
        } else {
            add_axis(
                &joystick,
                data.axis_index,
                false,
                data.target,
                cx.waker().clone(),
            );
            Poll::Pending
        }
    }
}
