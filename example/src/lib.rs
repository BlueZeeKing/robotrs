#![feature(try_blocks)]

use std::time::Duration;

use anyhow::anyhow;
use robotrs::{
    control::ControlSafe,
    hid::{
        any::AnyTriggerTarget, axis::AxisTarget, controller::XboxController, ext::ReleaseTriggerExt,
    },
    motor::IdleMode,
    robot::AsyncRobot,
    time::delay,
    yield_now, Deadzone,
};
use subsystems::{Arm, Drivetrain, Intake};
use utils::{periodic, subsystem::Subsystem, tracing::error};

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
        periodic!(drivetrain = &self.drivetrain => 0, async {
            let err: anyhow::Result<()> = try {
                if self.controller.b().value().ok_or(anyhow!("Could not get b button"))? {
                    drivetrain.set_idle_mode(IdleMode::Brake)?;
                } else {
                    drivetrain.set_idle_mode(IdleMode::Coast)?;
                }

                drivetrain.arcade_drive(
                    self.controller.left_y().ok_or(anyhow!("Could not get left y"))?.deadzone(0.1),
                    self.controller.right_x().ok_or(anyhow!("Could not get left x"))?.deadzone(0.1),
                )?;
            };

            if let Err(err) = err {
                error!("controller error: {:?}", err);
            }
        });
    }

    fn configure_bindings(
        &'static self,
        _scheduler: &robotrs::scheduler::RobotScheduler<Self>,
    ) -> anyhow::Result<()> {
        (
            self.controller.wait_left_trigger(AxisTarget::Away(0.1)),
            self.controller.wait_right_trigger(AxisTarget::Away(0.1)),
        )
            .any()
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

        self.controller
            .y()
            .while_pressed(|| Arm::start_raise_subsystem(&self.arm, 1));

        self.controller
            .a()
            .while_pressed(|| Arm::start_lower_subsystem(&self.arm, 1));

        self.controller
            .wait_right_y(AxisTarget::Up(0.65))
            .while_pressed(|| Intake::intake_cube_subsystem(&self.intake, 1));

        self.controller
            .wait_right_y(AxisTarget::Down(0.65))
            .while_pressed(|| Intake::intake_cone_subsystem(&self.intake, 1));

        (
            self.controller.left_bumper(),
            self.controller.right_bumper(),
        )
            .any()
            .while_pressed(|| Intake::start_release_subsystem(&self.intake, 1));

        Ok(())
    }
}

impl Robot {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            drivetrain: Subsystem::new(Drivetrain::new()?),
            arm: Subsystem::new(Arm::new()?),
            intake: Subsystem::new(Intake::new()?),
            controller: XboxController::new(0),
        })
    }
}
