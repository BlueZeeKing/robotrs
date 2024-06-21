use std::mem::MaybeUninit;

use hal_sys::{
    HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_GetJoystickPOVs, HAL_JoystickAxes,
    HAL_JoystickButtons, HAL_JoystickPOVs,
};

use super::{
    axis::{get_axis, Axis, AxisTarget},
    button::{Button, ButtonTarget},
    pov::{Pov, PovTarget},
};

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Joystick {
    num: u32,
}

impl Joystick {
    /// Create a new generic joystick. `num` is zero based
    ///
    /// Panics if `num` >= 6
    pub fn new(num: u32) -> Self {
        if num >= 6 {
            panic!("Joystick number out of range")
        } else {
            Self { num }
        }
    }

    /// Get the joystick index
    pub fn get_num(&self) -> u32 {
        self.num
    }

    pub(crate) fn get_button_data(&self) -> HAL_JoystickButtons {
        let mut buttons = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickButtons(self.num as i32, buttons.as_mut_ptr()) };

        if status != 0 {
            panic!("Something is very wrong with the HAL");
        }

        unsafe { buttons.assume_init() }
    }

    pub(crate) fn get_axes_data(&self) -> HAL_JoystickAxes {
        let mut axes = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickAxes(self.num as i32, axes.as_mut_ptr()) };

        if status != 0 {
            panic!("Something is very wrong with the HAL");
        }

        unsafe { axes.assume_init() }
    }

    pub(crate) fn get_pov_data(&self) -> HAL_JoystickPOVs {
        let mut povs = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickPOVs(self.num as i32, povs.as_mut_ptr()) };

        if status != 0 {
            panic!("Something is very wrong with the HAL");
        }

        unsafe { povs.assume_init() }
    }

    /// Get a trigger for the button at the given index (zero indexed). The trigger activates on
    /// press, but this can be changed through [Button::set_target]
    pub fn get_button(&self, idx: u32) -> Button {
        Button::new(*self, idx, ButtonTarget::Pressed)
    }

    /// Get a trigger for the axis at the given index (zero indexed).
    pub fn get_axis(&self, idx: u32, target: AxisTarget) -> Axis {
        Axis::new(*self, idx, target)
    }

    /// Get the value for the axis at the given index  (zero indexed). This returns [None] if the
    /// axis does not exist
    pub fn get_axis_value(&self, idx: u32) -> Option<f32> {
        get_axis(&self.get_axes_data(), idx)
    }

    /// Get a trigger for the pov at the given index (zero indexed).
    pub fn get_pov(&self, idx: u32, target: PovTarget) -> Pov {
        Pov::new(*self, idx, target)
    }
}
