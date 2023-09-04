use anyhow::Result;
use ctre::VictorSPX;
use revlib::{IdleMode, SparkMax};
use robotrs::control::ControlSafe;

pub struct Drivetrain {
    right_motor: SparkMax,
    left_motor: SparkMax,
    right_motor_victor: VictorSPX,
    left_motor_victor: VictorSPX,
}

impl Drivetrain {
    pub fn new() -> Result<Self> {
        Ok(Self {
            right_motor: SparkMax::new(2, revlib::MotorType::Brushed),
            left_motor: SparkMax::new(1, revlib::MotorType::Brushed),
            right_motor_victor: VictorSPX::new(4),
            left_motor_victor: VictorSPX::new(3),
        })
    }

    pub fn arcade_drive(&mut self, forward: f32, turn: f32) -> Result<()> {
        self.left_motor.set((forward + turn).clamp(-1.0, 1.0))?;
        self.right_motor.set((forward - turn).clamp(-1.0, 1.0))?;

        self.left_motor_victor
            .set_percent((forward + turn).clamp(-1.0, 1.0) as f64)?;
        self.right_motor_victor
            .set_percent((forward - turn).clamp(-1.0, 1.0) as f64)?;

        Ok(())
    }

    pub fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<()> {
        self.right_motor.set_idle_mode(idle_mode)?;
        self.left_motor.set_idle_mode(idle_mode)?;

        let victor_idle_mode = match idle_mode {
            IdleMode::Brake => ctre::IdleMode::Brake,
            IdleMode::Coast => ctre::IdleMode::Coast,
        };

        self.right_motor_victor.set_idle_mode(victor_idle_mode);
        self.left_motor_victor.set_idle_mode(victor_idle_mode);

        Ok(())
    }

    pub fn drive(&mut self, amount: f32) -> Result<()> {
        self.left_motor.set(amount.clamp(-1.0, 1.0))?;
        self.right_motor.set(amount.clamp(-1.0, 1.0))?;

        self.left_motor_victor
            .set_percent(amount.clamp(-1.0, 1.0) as f64)?;
        self.right_motor_victor
            .set_percent(amount.clamp(-1.0, 1.0) as f64)?;

        Ok(())
    }
}

impl ControlSafe for Drivetrain {
    fn stop(&mut self) {
        self.left_motor.stop();
        self.right_motor.stop();
    }
}

impl Default for Drivetrain {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
