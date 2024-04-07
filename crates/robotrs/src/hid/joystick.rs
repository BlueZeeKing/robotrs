use std::mem::MaybeUninit;

use crate::error::{Error, HalError, Result};
use hal_sys::{
    HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_GetJoystickPOVs, HAL_JoystickAxes,
    HAL_JoystickButtons, HAL_JoystickPOVs,
};

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
        let mut buttons = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickButtons(self.num as i32, buttons.as_mut_ptr()) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(unsafe { buttons.assume_init() })
    }

    pub fn get_axes_data(&self) -> Result<HAL_JoystickAxes> {
        let mut axes = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickAxes(self.num as i32, axes.as_mut_ptr()) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(unsafe { axes.assume_init() })
    }

    pub fn get_pov_data(&self) -> Result<HAL_JoystickPOVs> {
        let mut povs = MaybeUninit::uninit();

        let status = unsafe { HAL_GetJoystickPOVs(self.num as i32, povs.as_mut_ptr()) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(unsafe { povs.assume_init() })
    }
}
