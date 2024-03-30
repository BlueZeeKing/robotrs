use std::time::Duration;

use robotrs::{
    control::ControlSafe,
    hid::{axis::AxisTarget, controller::XboxController},
    motor::IdleMode,
    robot::AsyncRobot,
    time::delay,
    yield_now, Deadzone, FailableDefault,
};
use subsystems::{Arm, Drivetrain, Intake};
use utils::{
    subsystem::{Subsystem, SubsystemGroup},
    trigger::TriggerExt,
    wait, while_pressed_subsystem,
};

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
        (&self.arm, &self.intake, &self.drivetrain)
            .run(
                |(mut arm, mut intake, mut drivetrain)| async move {
                    arm.raise().await?;

                    intake.release_cube().await?;

                    arm.lower().await?;

                    drivetrain.drive(-1.0)?;

                    delay(Duration::from_secs(2)).await?;

                    drivetrain.stop(); // this is not really needed because this is called when the guard
                                       // is dropped
                    anyhow::Ok(())
                },
                0,
            )
            .await
            .expect("Could not run auto")
    }

    async fn get_enabled_future(&'static self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_teleop_future(&'static self) -> anyhow::Result<()> {
        // The periodic runs every 20ms because thats how fast the executor ticks
        loop {
            self.drivetrain
                .run(
                    |mut drivetrain| async move {
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
                    },
                    0,
                )
                .await
                .expect_err("drivetrain failed");
        }
    }

    fn configure_bindings(
        &'static self,
        _scheduler: &robotrs::scheduler::RobotScheduler<Self>,
    ) -> anyhow::Result<()> {
        while_pressed_subsystem!(
            self.controller
                .wait_left_trigger(AxisTarget::Away(0.1))
                .or(self.controller.wait_right_trigger(AxisTarget::Away(0.1))),
            &self.drivetrain,
            1,
            |mut drivetrain| async move {
                drivetrain.arcade_drive(
                    0.0,
                    (self.controller.left_trigger().unwrap().deadzone(0.1) * -1.0
                        + self.controller.right_trigger().unwrap().deadzone(0.1))
                        * SLOW_TURN_MODIFIER,
                )?;

                anyhow::Ok(())
            }
        );
        while_pressed_subsystem!(self.controller.y(), &self.arm, 1, |mut arm| async move {
            arm.raise().await?;

            wait!();

            anyhow::Ok(())
        });

        while_pressed_subsystem!(self.controller.a(), &self.arm, 1, |mut arm| async move {
            arm.lower().await?;

            wait!();

            anyhow::Ok(())
        });

        while_pressed_subsystem!(
            self.controller.wait_right_y(AxisTarget::Up(0.65)),
            &self.intake,
            1,
            |mut intake| async move {
                intake.intake_cube()?;

                wait!();

                anyhow::Ok(())
            }
        );

        while_pressed_subsystem!(
            self.controller.wait_right_y(AxisTarget::Down(0.65)),
            &self.intake,
            1,
            |mut intake| async move {
                intake.intake_cone()?;

                wait!();

                anyhow::Ok(())
            }
        );

        while_pressed_subsystem!(
            self.controller
                .left_bumper()
                .or(self.controller.right_bumper()),
            &self.intake,
            1,
            |mut intake| async move {
                intake.start_release()?;

                wait!();

                anyhow::Ok(())
            }
        );

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
