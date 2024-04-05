use encoder::{absolute::SparkMaxAbsoluteEncoder, relative::SparkMaxRelativeEncoder};
use error::REVError;
use robotrs::{
    control::ControlSafe,
    motor::{MotorController, SetIdleMode},
};
use std::ops::RangeInclusive;
use tracing::trace;

use crate::bindings::*;

#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod encoder;
pub mod error;

pub struct SparkMax {
    handle: c_SparkMax_handle,
}

#[derive(Clone, Copy)]
pub enum MotorType {
    Brushed = c_SparkMax_MotorType_c_SparkMax_kBrushed as isize,
    Brushless = c_SparkMax_MotorType_c_SparkMax_kBrushless as isize,
}

#[derive(Clone, Copy)]
pub enum IdleMode {
    Brake = c_SparkMax_IdleMode_c_SparkMax_kBrake as isize,
    Coast = c_SparkMax_IdleMode_c_SparkMax_kCoast as isize,
}

impl SparkMax {
    fn new_raw(can_id: i32, motor_type: MotorType, model: u32) -> Result<SparkMax, REVError> {
        trace!("Creating sparkmax");
        let error_code = unsafe { c_SparkMax_RegisterId(can_id) };
        if error_code != 0 {
            trace!("Error while creating Spark with id: {}", can_id);
            return Err(REVError::from(error_code));
        }

        let mut error_code = 0;
        let handle =
            unsafe { c_SparkMax_Create(can_id, motor_type as u32, model, &mut error_code) };
        if error_code != 0 {
            trace!("Error while creating Spark with id: {}", can_id);
            return Err(REVError::from(error_code));
        }

        Ok(SparkMax { handle })
    }

    pub fn new(can_id: i32, motor_type: MotorType) -> Result<SparkMax, REVError> {
        SparkMax::new_raw(can_id, motor_type, 0)
    }

    /// This should work for getting the relative encoder and setting the voltage, but it is
    /// untested. All other functionality has been ignored.
    pub fn new_flex(can_id: i32, motor_type: MotorType) -> Result<SparkMax, REVError> {
        SparkMax::new_raw(can_id, motor_type, 1)
    }

    pub fn set(&mut self, speed: f32) -> Result<(), REVError> {
        let error_code = unsafe {
            c_SparkMax_SetpointCommand(
                self.handle,
                speed,
                c_SparkMax_ControlType_c_SparkMax_kDutyCycle,
                0,
                0.0,
                0,
            )
        };

        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code.into())
        }
    }

    fn set_idle_mode_rev(&mut self, idle_mode: IdleMode) -> Result<(), REVError> {
        let error_code = unsafe { c_SparkMax_SetIdleMode(self.handle, idle_mode as u32) };

        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code.into())
        }
    }

    pub fn set_smart_current_limit(&mut self, limit: u8) -> Result<(), REVError> {
        let error_code = unsafe { c_SparkMax_SetSmartCurrentLimit(self.handle, limit, 0, 20000) };

        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code.into())
        }
    }

    pub fn set_pid(&mut self, p: f32, d: f32, i: f32, feedforward: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetP(self.handle, 0, p)) }?;
        unsafe { handle_error!(c_SparkMax_SetD(self.handle, 0, d)) }?;
        unsafe { handle_error!(c_SparkMax_SetI(self.handle, 0, i)) }?;
        unsafe { handle_error!(c_SparkMax_SetFF(self.handle, 0, feedforward)) }?;

        Ok(())
    }

    pub fn set_pid_range(&mut self, range: RangeInclusive<f32>) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetOutputRange(
                self.handle,
                0,
                *range.start(),
                *range.end()
            ))
        }?;

        Ok(())
    }

    pub fn set_wrapping(&mut self, wrapping: bool, max: f32, min: f32) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetPositionPIDWrapEnable(
                self.handle,
                if wrapping { 1 } else { 0 }
            ))
        }?;

        unsafe { handle_error!(c_SparkMax_SetPositionPIDMinInput(self.handle, min)) }?;
        unsafe { handle_error!(c_SparkMax_SetPositionPIDMaxInput(self.handle, max)) }?;

        Ok(())
    }

    pub fn get_absolute_encoder(&mut self) -> Result<SparkMaxAbsoluteEncoder, REVError> {
        unsafe {
            handle_error!(c_SparkMax_AttemptToSetDataPortConfig(
                self.handle,
                c_SparkMax_DataPortConfig_c_SparkMax_kDataPortConfigLimitSwitchesAndAbsoluteEncoder
            ))
        }?;

        Ok(SparkMaxAbsoluteEncoder::new(self.handle))
    }

    pub fn set_pid_input<T: FeedbackSensor>(&mut self, sensor: &T) -> Result<(), REVError> {
        if !sensor.is_handle(self.handle) {
            return Err(REVError::General);
        }

        unsafe { handle_error!(c_SparkMax_SetFeedbackDevice(self.handle, T::get_id())) }?;

        Ok(())
    }

    pub fn reset_settings(&mut self) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_RestoreFactoryDefaults(self.handle, 0)) }
    }

    pub fn write_settings(&mut self) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_BurnFlash(self.handle)) }
    }

    pub fn get_relative_encoder(&mut self) -> Result<SparkMaxRelativeEncoder, REVError> {
        unsafe { handle_error!(c_SparkMax_SetSensorType(self.handle, 1)) }?;

        Ok(SparkMaxRelativeEncoder::new(self.handle))
    }

    pub fn set_reference(&mut self, value: f32, control_type: ControlType) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetpointCommand(
                self.handle,
                value,
                control_type as u32,
                0,
                0.0,
                0
            ))
        }
    }

    pub fn follow(&mut self, other: &SparkMax, invert: bool) -> Result<(), REVError> {
        let mut device_id = 0;
        unsafe {
            handle_error!(c_SparkMax_GetDeviceId(other.handle, &mut device_id))?;
        }

        let id = 0x2051800 | device_id;
        let predefined = 26;

        unsafe {
            handle_error!(c_SparkMax_SetFollow(
                self.handle,
                id as u32,
                (0 & 0x3FFFF)
                    | (if invert { 1 } else { 0 } & 0x1) << 18
                    | (0 & 0x1F) << 19
                    | (predefined & 0xFF) << 24
            ))?;
        }

        Ok(())
    }
}

pub enum ControlType {
    DutyCycle = 0,
    Velocity = 1,
    Voltage = 2,
    Position = 3,
    SmartMotion = 4,
    Current = 5,
    SmartVelocity = 6,
}

pub trait FeedbackSensor {
    fn get_id() -> u32;
    fn is_handle(&self, handle: c_SparkMax_handle) -> bool;
}

impl MotorController for SparkMax {
    type Error = REVError;

    fn set_percent_raw(&mut self, value: f32) -> Result<(), Self::Error> {
        self.set(value)
    }

    fn set_voltage(&mut self, value: f32) -> Result<(), Self::Error> {
        self.set_reference(value, ControlType::Voltage)
    }

    fn set_inverted(&mut self, is_inverted: bool) -> Result<(), Self::Error> {
        unsafe {
            handle_error!(c_SparkMax_SetInverted(
                self.handle,
                if is_inverted { 1 } else { 0 }
            ))?;
        }

        Ok(())
    }
}

impl SetIdleMode for SparkMax {
    fn set_idle_mode(&mut self, idle_mode: robotrs::motor::IdleMode) -> Result<(), Self::Error> {
        match idle_mode {
            robotrs::motor::IdleMode::Brake => self.set_idle_mode_rev(IdleMode::Brake),
            robotrs::motor::IdleMode::Coast => self.set_idle_mode_rev(IdleMode::Coast),
        }
    }
}

impl ControlSafe for SparkMax {
    fn stop(&mut self) {
        self.set(0.0).unwrap();
    }
}

impl Drop for SparkMax {
    fn drop(&mut self) {
        unsafe { c_SparkMax_Destroy(self.handle) }
    }
}
