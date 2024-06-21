use hal_sys::HAL_JoystickButtons;

use super::{
    joystick::Joystick,
    reactor::{add_trigger, remove_trigger, set_target, wait_for_released, wait_for_triggered},
    ReleaseTrigger, Trigger,
};

pub(super) fn get_button(buttons: &HAL_JoystickButtons, index: u32) -> Option<bool> {
    if index >= buttons.count.into() {
        None
    } else {
        Some(buttons.buttons & (1 << index) > 0)
    }
}

/// A target for the button trigger
#[derive(Copy, Clone, Debug)]
pub enum ButtonTarget {
    /// Activate the trigger when pressed
    Pressed,
    /// Activate the trigger when released
    Released,
}

impl ButtonTarget {
    pub(super) fn is_active(&self, value: bool) -> bool {
        match self {
            ButtonTarget::Pressed => value,
            ButtonTarget::Released => !value,
        }
    }
}

/// A button trigger
pub struct Button {
    joystick: Joystick,
    button_index: u32,
    target: ButtonTarget,
    reactor_idx: usize,
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Self {
            joystick: self.joystick,
            button_index: self.button_index,
            target: self.target,
            reactor_idx: add_trigger(&self.joystick, self.button_index, self.target.into()),
        }
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        remove_trigger(self.reactor_idx);
    }
}

impl Button {
    pub(super) fn new(joystick: Joystick, button_index: u32, target: ButtonTarget) -> Self {
        Self {
            reactor_idx: add_trigger(&joystick, button_index, target.into()),
            joystick,
            button_index,
            target,
        }
    }

    /// Change the target of the trigger
    pub fn set_target(&mut self, target: ButtonTarget) {
        set_target(self.reactor_idx, target.into());
        self.target = target;
    }

    /// Get the value of the button. Returns [None] if the button does not exist.
    pub fn value(&self) -> Option<bool> {
        get_button(&self.joystick.get_button_data(), self.button_index)
    }
}

impl Trigger for Button {
    type Error = ();
    type Output = ();

    async fn wait_for_trigger(&mut self) -> Result<(), ()> {
        wait_for_triggered(self.reactor_idx).await
    }
}

impl ReleaseTrigger for Button {
    async fn wait_for_release(&mut self) -> Result<(), ()> {
        wait_for_released(self.reactor_idx).await
    }
}
