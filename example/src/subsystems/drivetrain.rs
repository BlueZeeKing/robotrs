use anyhow::Result;
use revlib::{IdleMode, SparkMax};
use robotrs::control::ControlSafe;

// TODO: VictorSPXs
pub struct Drivetrain {
    right_motor: SparkMax,
    left_motor: SparkMax,
}

impl Drivetrain {
    pub fn new() -> Result<Self> {
        Ok(Self {
            right_motor: SparkMax::new(2, revlib::MotorType::Brushed),
            left_motor: SparkMax::new(1, revlib::MotorType::Brushed),
        })
    }

    pub fn arcade_drive(&mut self, forward: f32, turn: f32) -> Result<()> {
        self.left_motor.set((forward + turn).clamp(-1.0, 1.0))?;
        self.right_motor.set((forward - turn).clamp(-1.0, 1.0))?;

        Ok(())
    }

    pub fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<()> {
        self.right_motor.set_idle_mode(idle_mode)?;
        self.left_motor.set_idle_mode(idle_mode)?;

        Ok(())
    }

    pub fn drive(&mut self, amount: f32) -> Result<()> {
        self.left_motor.set(amount.clamp(-1.0, 1.0))?;
        self.right_motor.set(amount.clamp(-1.0, 1.0))?;

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
