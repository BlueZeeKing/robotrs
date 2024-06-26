use std::ffi::c_void;

use robotrs::{
    control::ControlSafe,
    motor::{MotorController, SetIdleMode},
};

#[allow(warnings)]
mod bindings;

pub mod error;

type ControllerHandle = *mut c_void;

pub struct VictorSPX {
    handle: ControllerHandle,
}

#[derive(Clone, Copy)]
pub enum VictorSPXControlMode {
    PercentOutput = 0,
    Position = 1,
    Velocity = 2,
    Follower = 5,
    MotionProfile = 6,
    MotionMagic = 7,
    MotionProfileArc = 10,
    Disabled = 15,
}

#[derive(Clone, Copy)]
pub enum DemandType {
    Neutral = 0,
    AuxPID = 1,
    ArbitraryFeedForward = 2,
}

#[derive(Clone, Copy)]
pub enum IdleMode {
    EEPROMSetting = 0,
    Coast = 1,
    Brake = 2,
}

impl VictorSPX {
    pub fn new(id: i32) -> Self {
        Self {
            handle: unsafe {
                bindings::c_MotController_Create2(id, c"Victor SPX".as_ptr(), c"".as_ptr())
            },
        }
    }

    fn set_percent_ctre(&mut self, speed: f64) -> Result<(), error::Error> {
        self.set(speed, VictorSPXControlMode::PercentOutput)
    }

    pub fn set(&mut self, speed: f64, mode: VictorSPXControlMode) -> Result<(), error::Error> {
        self.set_with_demand(speed, mode, 0.0, DemandType::Neutral)
    }

    pub fn set_with_demand(
        &mut self,
        speed: f64,
        mode: VictorSPXControlMode,
        demand: f64,
        demand_type: DemandType,
    ) -> Result<(), error::Error> {
        let error = unsafe {
            bindings::c_MotController_Set_4(
                self.handle,
                mode as i32,
                speed,
                demand,
                demand_type as i32,
            )
        };

        error::to_result(error)
    }

    fn set_idle_mode_ctre(&mut self, idle_mode: IdleMode) {
        unsafe { bindings::c_MotController_SetNeutralMode(self.handle, idle_mode as i32) };
    }
}

impl MotorController for VictorSPX {
    type Error = error::Error;

    fn set_percent_raw(&mut self, value: f32) -> Result<(), Self::Error> {
        self.set_percent_ctre(value as f64)
    }

    fn set_voltage(&mut self, _value: f32) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl SetIdleMode for VictorSPX {
    fn set_idle_mode(&mut self, idle_mode: robotrs::motor::IdleMode) -> Result<(), Self::Error> {
        match idle_mode {
            robotrs::motor::IdleMode::Brake => self.set_idle_mode_ctre(IdleMode::Brake),
            robotrs::motor::IdleMode::Coast => self.set_idle_mode_ctre(IdleMode::Coast),
        }

        Ok(())
    }
}

impl Drop for VictorSPX {
    fn drop(&mut self) {
        unsafe {
            let error = bindings::c_MotController_Destroy(self.handle);

            match error::to_result(error) {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("Could not close motor due to {}", err);
                    panic!("Could not close motor due to {}", err);
                }
            }
        }
    }
}

impl ControlSafe for VictorSPX {
    fn stop(&mut self) {
        match self.set(0.0, VictorSPXControlMode::Disabled) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Could not close motor due to {}", err);
                panic!("Could not close motor due to {}", err); // not sure if this should panic
            }
        }
    }
}
