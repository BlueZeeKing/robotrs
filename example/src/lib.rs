use std::time::Duration;

use futures::{join, select, FutureExt};
use robotrs::{
    control::{ControlLock, ControlSafe},
    hid::{axis::AxisTarget, controller::XboxController},
    motor::IdleMode,
    robot::AsyncRobot,
    time::Alarm,
    yield_now, Deadzone, FailableDefault,
};
use subsystems::{Arm, Drivetrain, Intake};

pub mod subsystems;

const SLOW_TURN_MODIFIER: f32 = 0.7;

pub enum GamePiece {
    Cube,
    Cone,
}

pub struct Robot {
    drivetrain: ControlLock<Drivetrain>,
    arm: ControlLock<Arm>,
    intake: ControlLock<Intake>,
    controller: XboxController,
}

impl AsyncRobot for Robot {
    async fn get_auto_future(&self) -> anyhow::Result<()> {
        let mut arm = self.arm.lock().await;

        arm.raise().await?;

        let mut intake = self.intake.lock().await;

        intake.release_cube().await?;

        drop(intake);

        arm.lower().await?;

        drop(arm);

        let mut drivetrain = self.drivetrain.lock().await;

        drivetrain.drive(-1.0)?;

        Alarm::new(Duration::from_secs(2)).await?;

        drivetrain.stop(); // this is not really needed because this is called when the guard
                           // is dropped

        drop(drivetrain);

        Ok(())
    }

    async fn get_enabled_future(&self) -> anyhow::Result<()> {
        let vals = join!(
            Self::raise(self),
            Self::lower(self),
            Self::cube(self),
            Self::cone(self),
            Self::release(self)
        );

        vals.0?;
        vals.1?;
        vals.2?;
        vals.3?;
        vals.4?;

        Ok(())
    }

    async fn get_teleop_future(&self) -> anyhow::Result<()> {
        // The periodic runs every 20ms because thats how fast the executor ticks
        let mut drivetrain = self.drivetrain.lock().await;

        loop {
            if self.controller.b().value()? {
                drivetrain.set_idle_mode(IdleMode::Brake)?;
            } else {
                drivetrain.set_idle_mode(IdleMode::Coast)?;
            }

            if self.controller.left_trigger().unwrap().deadzone(0.1) != 0.0
                || self.controller.right_trigger().unwrap().deadzone(0.1) != 0.0
            {
                drivetrain.arcade_drive(
                    0.0,
                    (self.controller.left_trigger().unwrap().deadzone(0.1) * -1.0
                        + self.controller.right_trigger().unwrap().deadzone(0.1))
                        * SLOW_TURN_MODIFIER,
                )?;
            } else {
                drivetrain.arcade_drive(
                    self.controller.left_y().unwrap().deadzone(0.1),
                    self.controller.right_x().unwrap().deadzone(0.1),
                )?;
            }

            yield_now().await;
        }
    }

    fn configure_bindings<'a>(
        &'a self,
        _scheduler: &'a robotrs::scheduler::RobotScheduler<'a, Self>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Robot {
    pub async fn raise(&self) -> anyhow::Result<()> {
        loop {
            let released = self.controller.y().await?;

            let mut arm_lock = self.arm.lock().await;

            arm_lock.start_raise()?;

            released.await?;

            arm_lock.stop();
        }
    }

    pub async fn lower(&self) -> anyhow::Result<()> {
        loop {
            let released = self.controller.a().await?;

            let mut arm_lock = self.arm.lock().await;

            arm_lock.start_lower()?;

            released.await?;

            arm_lock.stop();
        }
    }

    pub async fn cube(&self) -> anyhow::Result<()> {
        loop {
            let released = self.controller.wait_right_y(AxisTarget::Up(0.65)).await?;

            let mut intake_lock = self.intake.lock().await;

            intake_lock.intake_cube()?;

            released.await?;

            intake_lock.stop();
        }
    }

    pub async fn cone(&self) -> anyhow::Result<()> {
        loop {
            let released = self.controller.wait_right_y(AxisTarget::Down(0.65)).await?;

            let mut intake_lock = self.intake.lock().await;

            intake_lock.intake_cone()?;

            released.await?;

            intake_lock.stop();
        }
    }

    pub async fn release(&self) -> anyhow::Result<()> {
        loop {
            let released = select! {
                released = self.controller.left_bumper().fuse() => released,
                released = self.controller.right_bumper().fuse() => released
            }?;

            let mut intake_lock = self.intake.lock().await;

            intake_lock.start_release()?;

            released.await?;

            intake_lock.stop();
        }
    }
}

impl FailableDefault for Robot {
    fn failable_default() -> anyhow::Result<Self> {
        Ok(Self {
            drivetrain: FailableDefault::failable_default()?,
            arm: FailableDefault::failable_default()?,
            intake: FailableDefault::failable_default()?,
            controller: XboxController::new(0)?,
        })
    }
}
