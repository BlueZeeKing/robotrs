use hal_sys::{HAL_ControlWord, HAL_GetControlWord, HAL_RefreshDSData};

use crate::error::{HalError, Result};

pub fn get_control_word() -> Result<HAL_ControlWord> {
    unsafe {
        HAL_RefreshDSData();
    }

    let mut word = HAL_ControlWord {
        _bitfield_align_1: [0; 0],
        _bitfield_1: HAL_ControlWord::new_bitfield_1(0, 0, 0, 0, 0, 0, 0),
    };

    let status = unsafe { HAL_GetControlWord(&mut word) };

    if status != 0 {
        Err(HalError(status).into())
    } else {
        Ok(word)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Teleop,
    Auto,
    Test,
    Disabled,
}

impl State {
    pub fn from_control_word(word: &HAL_ControlWord) -> Self {
        if word.enabled() == 0 {
            Self::Disabled
        } else if word.autonomous() == 1 {
            Self::Auto
        } else if word.test() == 1 {
            Self::Test
        } else {
            Self::Teleop
        }
    }
}
