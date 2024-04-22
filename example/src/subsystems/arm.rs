use anyhow::Result;
use macros::subsystem_task;
use revlib::SparkMax;
use robotrs::{
    control::ControlSafe,
    motor::{IdleMode, SetIdleMode},
    time::delay,
};
use std::time::Duration;

const ARM_SPEED: f32 = 0.4;
const AMP_LIMIT: u8 = 20;

pub struct Arm {
    motor: SparkMax,
}

impl Arm {
    pub fn new() -> Result<Self> {
        let mut motor = SparkMax::new(5, revlib::MotorType::Brushless)?;

        motor.set_idle_mode(IdleMode::Brake)?;
        motor.set_smart_current_limit(AMP_LIMIT)?;

        Ok(Self { motor })
    }

    #[subsystem_task(wait)]
    pub fn start_raise(#[subsystem] &mut self) -> Result<()> {
        self.motor.set(-ARM_SPEED)?;

        Ok(())
    }

    #[subsystem_task(wait)]
    pub fn start_lower(#[subsystem] &mut self) -> Result<()> {
        self.motor.set(ARM_SPEED)?;

        Ok(())
    }

    pub async fn raise(&mut self) -> Result<()> {
        self.start_raise()?;

        delay(Duration::from_secs(2)).await;

        self.stop();

        Ok(())
    }

    pub async fn lower(&mut self) -> Result<()> {
        self.start_lower()?;

        delay(Duration::from_secs(2)).await;

        self.stop();

        Ok(())
    }
}

impl ControlSafe for Arm {
    fn stop(&mut self) {
        self.motor.stop();
    }
}
