use std::time::Duration;

use robotrs::{
    control::ControlSafe,
    hid::{axis::AxisTarget, controller::XboxController},
    motor::IdleMode,
    robot::AsyncRobot,
    scheduler::guard,
    time::delay,
    yield_now, Deadzone, FailableDefault,
};
use subsystems::{Arm, Drivetrain, Intake};
use utils::{subsystem::Subsystem, trigger::TriggerExt, wait};

pub mod subsystems;

const SLOW_TURN_MODIFIER: f32 = 0.7;

pub enum GamePiece {
    Cube,
    Cone,
}

pub struct Robot {
    drivetrain: Subsystem<Drivetrain>,
    arm: Subsystem<Arm>,
    intake: Subsystem<Intake>,
    controller: XboxController,
}

impl AsyncRobot for Robot {
    async fn get_auto_future(&'static self) -> anyhow::Result<()> {
        let mut arm = self.arm.lock(0).await;
        let mut intake = self.intake.lock(0).await;

        arm.raise().await?;

        intake.release_cube().await?;

        drop(intake);

        arm.lower().await?;

        drop(arm);

        let mut drivetrain = self.drivetrain.lock(0).await;

        drivetrain.drive(-1.0)?;

        delay(Duration::from_secs(2)).await;

        drivetrain.stop(); // this is not really needed because this is called when the guard
                           // is dropped
        anyhow::Ok(())
    }

    async fn get_enabled_future(&'static self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_teleop_future(&'static self) -> anyhow::Result<()> {
        // The periodic runs every 20ms because thats how fast the executor ticks
        loop {
            guard(async {
                let mut drivetrain = self.drivetrain.lock(0).await;

                loop {
                    if self.controller.b().value()? {
                        drivetrain.set_idle_mode(IdleMode::Brake)?;
                    } else {
                        drivetrain.set_idle_mode(IdleMode::Coast)?;
                    }

                    drivetrain.arcade_drive(
                        self.controller.left_y().unwrap().deadzone(0.1),
                        self.controller.right_x().unwrap().deadzone(0.1),
                    )?;

                    yield_now().await;
                }

                #[allow(unreachable_code)]
                anyhow::Ok(())
            })
            .await
            .unwrap_err();
        }
    }

    fn configure_bindings(
        &'static self,
        _scheduler: &robotrs::scheduler::RobotScheduler<Self>,
    ) -> anyhow::Result<()> {
        self.controller
            .wait_left_trigger(AxisTarget::Away(0.1))
            .or(self.controller.wait_right_trigger(AxisTarget::Away(0.1)))
            .while_pressed(|| async {
                let mut drivetrain = self.drivetrain.lock(1).await;

                loop {
                    drivetrain.arcade_drive(
                        0.0,
                        (self.controller.left_trigger().unwrap().deadzone(0.1) * -1.0
                            + self.controller.right_trigger().unwrap().deadzone(0.1))
                            * SLOW_TURN_MODIFIER,
                    )?;

                    yield_now().await;
                }

                #[allow(unreachable_code)]
                anyhow::Ok(())
            });

        self.controller.y().while_pressed(|| async {
            let mut arm = self.arm.lock(1).await;
            arm.start_raise()?;

            wait!();

            anyhow::Ok(())
        });

        self.controller.a().while_pressed(|| async {
            let mut arm = self.arm.lock(1).await;
            arm.start_lower()?;

            wait!();

            anyhow::Ok(())
        });

        self.controller
            .wait_right_y(AxisTarget::Up(0.65))
            .while_pressed(|| async {
                let mut intake = self.intake.lock(1).await;
                intake.intake_cube()?;

                wait!();

                anyhow::Ok(())
            });

        self.controller
            .wait_right_y(AxisTarget::Down(0.65))
            .while_pressed(|| async {
                let mut intake = self.intake.lock(1).await;
                intake.intake_cone()?;

                wait!();

                anyhow::Ok(())
            });

        self.controller
            .left_bumper()
            .or(self.controller.right_bumper())
            .while_pressed(|| async {
                let mut intake = self.intake.lock(1).await;
                intake.start_release()?;

                wait!();

                anyhow::Ok(())
            });

        Ok(())
    }
}

impl FailableDefault for Robot {
    fn failable_default() -> anyhow::Result<Self> {
        Ok(Self {
            drivetrain: Subsystem::new(FailableDefault::failable_default()?),
            arm: Subsystem::new(FailableDefault::failable_default()?),
            intake: Subsystem::new(FailableDefault::failable_default()?),
            controller: XboxController::new(0)?,
        })
    }
}
