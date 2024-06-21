use hal_sys::HAL_JoystickAxes;

use super::{
    joystick::Joystick,
    reactor::{add_trigger, remove_trigger, set_target, wait_for_released, wait_for_triggered},
    ReleaseTrigger, Trigger,
};

pub(super) fn get_axis(data: &HAL_JoystickAxes, index: u32) -> Option<f32> {
    if index >= data.count as u32 {
        None
    } else {
        Some(data.axes[index as usize])
    }
}

/// A target that defines when an [Axis] should trigger
#[derive(Copy, Clone, Debug)]
pub enum AxisTarget {
    /// Activates when the axis moves farther than the given value. Direction doesn't matter
    Away(f32),
    /// Activates when the axis moves closer than the given value. Direction doesn't matter
    Within(f32),
    /// Activates when the axis moves higher than the given value
    Up(f32),
    /// Activates when the axis moves lower than the given value
    Down(f32),
}

impl AxisTarget {
    pub(super) fn is_active(&self, value: f32) -> bool {
        match self {
            AxisTarget::Away(dist) => value.abs() > *dist,
            AxisTarget::Within(dist) => value.abs() < *dist,
            AxisTarget::Down(target) => value < *target,
            AxisTarget::Up(target) => value > *target,
        }
    }
}

/// A trigger a given axis
pub struct Axis {
    joystick: Joystick,
    axis_index: u32,
    target: AxisTarget,
    reactor_idx: usize,
}

impl Clone for Axis {
    fn clone(&self) -> Self {
        Self {
            joystick: self.joystick,
            axis_index: self.axis_index,
            target: self.target,
            reactor_idx: add_trigger(&self.joystick, self.axis_index, self.target.into()),
        }
    }
}

impl Drop for Axis {
    fn drop(&mut self) {
        remove_trigger(self.reactor_idx);
    }
}

impl Axis {
    pub(super) fn new(joystick: Joystick, axis_index: u32, target: AxisTarget) -> Self {
        Self {
            reactor_idx: add_trigger(&joystick, axis_index, target.into()),
            joystick,
            axis_index,
            target,
        }
    }

    /// Get the value of the axis. Returns [None] if the axis does not exist
    pub fn value(&self) -> Option<f32> {
        get_axis(&self.joystick.get_axes_data(), self.axis_index)
    }

    /// Change the target of the trigger
    pub fn set_target(&mut self, target: AxisTarget) {
        set_target(self.reactor_idx, target.into());
        self.target = target;
    }
}

impl Trigger for Axis {
    type Error = ();
    type Output = ();

    async fn wait_for_trigger(&mut self) -> Result<(), ()> {
        wait_for_triggered(self.reactor_idx).await
    }
}

impl ReleaseTrigger for Axis {
    async fn wait_for_release(&mut self) -> Result<(), ()> {
        wait_for_released(self.reactor_idx).await
    }
}
