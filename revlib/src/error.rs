use std::{error::Error, ffi::CStr, fmt::Display};

use crate::bindings::c_REVLib_ErrorFromCode;

#[derive(Debug, Clone)]
pub enum REVError {
    General = 1,
    CANTimeout = 2,
    NotImplemented = 3,
    HAL = 4,
    CantFindFirmware = 5,
    FirmwareTooOld = 6,
    FirmwareTooNew = 7,
    ParamInvalidID = 8,
    ParamMismatchType = 9,
    ParamAccessMode = 10,
    ParamInvalid = 11,
    ParamNotImplementedDeprecated = 12,
    FollowConfigMismatch = 13,
    Invalid = 14,
    SetpointOutOfRange = 15,
    Unknown = 16,
    CANDisconnected = 17,
    DuplicateCANId = 18,
    InvalidCANId = 19,
    SparkMaxDataPortAlreadyConfiguredDifferently = 20,
    NumCodes = 21,
}

impl Display for REVError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            CStr::from_ptr(c_REVLib_ErrorFromCode(self.clone() as u32))
                .to_str()
                .unwrap()
        })
    }
}

impl Error for REVError {}

impl From<u32> for REVError {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::General,
            2 => Self::CANTimeout,
            3 => Self::NotImplemented,
            4 => Self::HAL,
            5 => Self::CantFindFirmware,
            6 => Self::FirmwareTooOld,
            7 => Self::FirmwareTooNew,
            8 => Self::ParamInvalidID,
            9 => Self::ParamMismatchType,
            10 => Self::ParamAccessMode,
            11 => Self::ParamInvalid,
            12 => Self::ParamNotImplementedDeprecated,
            13 => Self::FollowConfigMismatch,
            14 => Self::Invalid,
            15 => Self::SetpointOutOfRange,
            16 => Self::Unknown,
            17 => Self::CANDisconnected,
            18 => Self::DuplicateCANId,
            19 => Self::InvalidCANId,
            20 => Self::SparkMaxDataPortAlreadyConfiguredDifferently,
            21 => Self::NumCodes,
            code => panic!("Unknown rev error code {code}"),
        }
    }
}
