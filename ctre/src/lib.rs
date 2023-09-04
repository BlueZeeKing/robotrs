use std::ffi::{c_void, CString};

use robotrs::control::ControlSafe;

#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod error;

type ControllerHandle = *mut c_void;

pub struct VictorSPX {
    handle: ControllerHandle,
}

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

pub enum DemandType {
    Neutral = 0,
    AuxPID = 1,
    ArbitraryFeedForward = 2,
}

pub enum IdleMode {
    EEPROMSetting = 0,
    Coast = 1,
    Brake = 2,
}

impl VictorSPX {
    pub fn new(id: i32) -> Self {
        let model = CString::new("Victor SPX").unwrap();
        let can_bus = CString::new("").unwrap();

        let model_ptr = model.as_ptr();
        let can_bus_ptr = can_bus.as_ptr();

        std::mem::forget(model);
        std::mem::forget(can_bus);

        Self {
            handle: unsafe { bindings::c_MotController_Create2(id, model_ptr, can_bus_ptr) },
        }
    }

    pub fn set_percent(&mut self, speed: f64) -> Result<(), error::Error> {
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

    pub fn set_idle_mode(&mut self, idle_mode: IdleMode) {
        unsafe { bindings::c_MotController_SetNeutralMode(self.handle, idle_mode as i32) };
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
