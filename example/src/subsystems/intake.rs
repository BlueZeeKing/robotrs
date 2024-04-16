use anyhow::Result;
use macros::subsystem_task;
use revlib::SparkMax;
use robotrs::{
    control::ControlSafe,
    motor::{IdleMode, SetIdleMode},
    time::delay,
};
use std::time::Duration;

use crate::GamePiece;

const AMP_LIMIT: u8 = 20;

pub struct Intake {
    motor: SparkMax,
    last_item: Option<GamePiece>,
}

impl Intake {
    pub fn new() -> Result<Self> {
        let mut motor = SparkMax::new(6, revlib::MotorType::Brushless)?;

        motor.set_idle_mode(IdleMode::Brake)?;
        motor.set_smart_current_limit(AMP_LIMIT)?;

        Ok(Self {
            motor,
            last_item: None,
        })
    }

    #[subsystem_task]
    pub async fn release_cube(&mut self) -> Result<()> {
        self.motor.set(-1.0)?;

        delay(Duration::from_secs(1)).await;

        self.motor.stop();

        Ok(())
    }

    #[subsystem_task(wait)]
    pub fn intake_cube(&mut self) -> Result<()> {
        self.motor.set(0.66)?;

        self.last_item = Some(GamePiece::Cube);

        Ok(())
    }

    #[subsystem_task(wait)]
    pub fn intake_cone(&mut self) -> Result<()> {
        self.motor.set(-1.0)?;

        self.last_item = Some(GamePiece::Cone);

        Ok(())
    }

    #[subsystem_task(wait)]
    pub fn start_release(&mut self) -> Result<()> {
        if let Some(item) = &self.last_item {
            self.motor.set(match item {
                GamePiece::Cube => -1.0,
                GamePiece::Cone => 1.0,
            })?;
        }

        Ok(())
    }
}

impl ControlSafe for Intake {
    fn stop(&mut self) {
        self.motor.stop();
    }
}
