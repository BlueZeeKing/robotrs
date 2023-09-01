use anyhow::Result;
use revlib::{IdleMode, SparkMax};
use robotrs::{control::ControlSafe, time::Alarm};
use std::time::Duration;

const ARM_SPEED: f32 = 0.4;
const AMP_LIMIT: u8 = 20;

pub struct Arm {
    motor: SparkMax,
}

impl Arm {
    pub fn new() -> Result<Self> {
        let mut motor = SparkMax::new(5, revlib::MotorType::Brushless);

        motor.set_idle_mode(IdleMode::Brake)?;
        motor.set_smart_current_limit(AMP_LIMIT)?;

        Ok(Self { motor })
    }

    pub fn start_raise(&mut self) -> Result<()> {
        self.motor.set(-ARM_SPEED)?;

        Ok(())
    }

    pub fn start_lower(&mut self) -> Result<()> {
        self.motor.set(ARM_SPEED)?;

        Ok(())
    }

    pub async fn raise(&mut self) -> Result<()> {
        self.start_raise()?;

        Alarm::new(Duration::from_secs(2)).await?;

        self.stop();

        Ok(())
    }

    pub async fn lower(&mut self) -> Result<()> {
        self.start_lower()?;

        Alarm::new(Duration::from_secs(2)).await?;

        self.stop();

        Ok(())
    }
}

impl ControlSafe for Arm {
    fn stop(&mut self) {
        self.motor.stop();
    }
}

impl Default for Arm {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
