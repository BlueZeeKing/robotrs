use hal_sys::HAL_JoystickPOVs;

use super::{
    joystick::Joystick,
    reactor::{add_trigger, remove_trigger, wait_for_released, wait_for_triggered},
    ReleaseTrigger, Trigger,
};

pub(super) fn get_pov(povs: &HAL_JoystickPOVs, index: u32) -> Option<i16> {
    if index as i16 >= povs.count {
        None
    } else {
        Some(povs.povs[index as usize])
    }
}

/// This defines when a [Pov] trigger activates
#[derive(Copy, Clone, Debug)]
pub enum PovTarget {
    Raw(i16),
}

impl PovTarget {
    pub(super) fn is_active(&self, value: i16) -> bool {
        match self {
            Self::Raw(desired) => value == *desired,
        }
    }
}

/// A trigger for a joystick pov. This the the d-pad
pub struct Pov {
    joystick: Joystick,
    pov_index: u32,
    target: PovTarget,
    reactor_idx: usize,
}

impl Clone for Pov {
    fn clone(&self) -> Self {
        Self {
            joystick: self.joystick,
            pov_index: self.pov_index,
            target: self.target,
            reactor_idx: add_trigger(&self.joystick, self.pov_index, self.target.into()),
        }
    }
}

impl Drop for Pov {
    fn drop(&mut self) {
        remove_trigger(self.reactor_idx);
    }
}

impl Pov {
    pub(super) fn new(joystick: Joystick, pov_index: u32, target: PovTarget) -> Self {
        Pov {
            reactor_idx: add_trigger(&joystick, pov_index, target.into()),
            joystick,
            pov_index,
            target,
        }
    }

    /// Get the current value of the pov. This is an angle in degrees
    pub fn value(&self) -> Option<i16> {
        get_pov(&self.joystick.get_pov_data(), self.pov_index)
    }

    /// Set the target of the trigger
    pub fn set_target(&mut self, target: PovTarget) {
        self.target = target;
    }
}

impl Trigger for Pov {
    type Error = ();
    type Output = ();

    async fn wait_for_trigger(&mut self) -> Result<(), ()> {
        wait_for_triggered(self.reactor_idx).await
    }
}

impl ReleaseTrigger for Pov {
    async fn wait_for_release(&mut self) -> Result<(), ()> {
        wait_for_released(self.reactor_idx).await
    }
}
