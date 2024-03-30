use crate::error::{Error, HalError, Result};
use hal_sys::{HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_JoystickAxes, HAL_JoystickButtons};

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Joystick {
    num: u32,
}

impl Joystick {
    pub fn new(num: u32) -> Result<Self> {
        if num > 6 {
            // TODO: Fix bindings to include consts
            Err(Error::JoystickIndexOutOfRange(num))
        } else {
            Ok(Self { num })
        }
    }

    pub fn get_num(&self) -> u32 {
        self.num
    }

    pub fn get_button_data(&self) -> Result<HAL_JoystickButtons> {
        let mut buttons = HAL_JoystickButtons {
            buttons: 0,
            count: 0,
        };

        let status = unsafe { HAL_GetJoystickButtons(self.num as i32, &mut buttons) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(buttons)
    }

    pub fn get_axes_data(&self) -> Result<HAL_JoystickAxes> {
        let mut axes = HAL_JoystickAxes {
            count: 0,
            axes: [0.; 12],
            raw: [0; 12],
        };

        let status = unsafe { HAL_GetJoystickAxes(self.num as i32, &mut axes) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(axes)
    }
}
