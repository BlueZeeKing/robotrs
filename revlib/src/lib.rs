use bindings::c_SparkMax_handle;
use error::REVError;
use robotrs::control::ControlSafe;

use crate::bindings::c_SparkMax_ControlType_c_SparkMax_kDutyCycle;

#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod error;

pub struct SparkMax {
    handle: c_SparkMax_handle,
}

#[derive(Clone, Copy)]
pub enum MotorType {
    Brushed = bindings::c_SparkMax_MotorType_c_SparkMax_kBrushed as isize,
    Brushless = bindings::c_SparkMax_MotorType_c_SparkMax_kBrushless as isize,
}

#[derive(Clone, Copy)]
pub enum IdleMode {
    Brake = bindings::c_SparkMax_IdleMode_c_SparkMax_kBrake as isize,
    Coast = bindings::c_SparkMax_IdleMode_c_SparkMax_kCoast as isize,
}

impl SparkMax {
    pub fn new(can_id: i32, motor_type: MotorType) -> SparkMax {
        SparkMax {
            handle: unsafe { bindings::c_SparkMax_Create(can_id, motor_type as u32) },
        }
    }

    pub fn set(&mut self, speed: f32) -> Result<(), REVError> {
        let error_code = unsafe {
            bindings::c_SparkMax_SetpointCommand(
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

    pub fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), REVError> {
        let error_code = unsafe { bindings::c_SparkMax_SetIdleMode(self.handle, idle_mode as u32) };

        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code.into())
        }
    }

    pub fn set_smart_current_limit(&mut self, limit: u8) -> Result<(), REVError> {
        let error_code =
            unsafe { bindings::c_SparkMax_SetSmartCurrentLimit(self.handle, limit, 0, 20000) };

        if error_code == 0 {
            Ok(())
        } else {
            Err(error_code.into())
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
        unsafe { bindings::c_SparkMax_Destroy(self.handle) }
    }
}
