use std::time::Duration;

use futures::{select, FutureExt};
use robotrs::{
    control::{ControlLock, ControlSafe},
    hid::{axis::AxisTarget, controller::XboxController},
    robot::{AsyncRobot, Fut},
    scheduler::Spawner,
    time::Alarm,
    Deadzone, Yield,
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
    fn get_auto_future(self: std::rc::Rc<Self>) -> Fut {
        Box::pin(async move {
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
        })
    }

    fn get_enabled_future(self: std::rc::Rc<Self>) -> Fut {
        Box::pin(async move { Ok(()) })
    }

    fn get_teleop_future(self: std::rc::Rc<Self>) -> Fut {
        Box::pin(async move {
            // The periodic runs every 20ms because thats how fast the executor ticks
            let mut drivetrain = self.drivetrain.lock().await;

            loop {
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

                Yield::default().await;
            }
        })
    }

    fn create_bindings(self: std::rc::Rc<Self>, executor: &Spawner) {
        // BRAKE

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = cloned_self.controller.b().await?;

                unsafe {
                    cloned_self.drivetrain.steal(|drivetrain| {
                        drivetrain.set_idle_mode(revlib::IdleMode::Brake).unwrap()
                    });
                }

                released.await?;

                unsafe {
                    cloned_self.drivetrain.steal(|drivetrain| {
                        drivetrain.set_idle_mode(revlib::IdleMode::Coast).unwrap()
                    });
                }
            }
        });

        // ARM RAISE

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = cloned_self.controller.y().await?;

                let mut arm_lock = cloned_self.arm.lock().await;

                arm_lock.start_raise()?;

                released.await?;

                arm_lock.stop();
            }
        });

        // ARM LOWER

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = cloned_self.controller.a().await?;

                let mut arm_lock = cloned_self.arm.lock().await;

                arm_lock.start_lower()?;

                released.await?;

                arm_lock.stop();
            }
        });

        // INTAKE CUBE

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = cloned_self
                    .controller
                    .wait_right_y(AxisTarget::Up(0.65))
                    .await?;

                let mut intake_lock = cloned_self.intake.lock().await;

                intake_lock.intake_cube()?;

                released.await?;

                intake_lock.stop();
            }
        });

        // INTAKE CONE

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = cloned_self
                    .controller
                    .wait_right_y(AxisTarget::Down(0.65))
                    .await?;

                let mut intake_lock = cloned_self.intake.lock().await;

                intake_lock.intake_cone()?;

                released.await?;

                intake_lock.stop();
            }
        });

        // INTAKE RELEASE

        let cloned_self = self.clone();

        executor.spawn(async move {
            loop {
                let released = select! {
                    released = cloned_self.controller.left_bumper().fuse() => released,
                    released = cloned_self.controller.right_bumper().fuse() => released
                }?;

                let mut intake_lock = cloned_self.intake.lock().await;

                intake_lock.start_release()?;

                released.await?;

                intake_lock.stop();
            }
        });
    }
}

impl Robot {
    pub fn new() -> Self {
        Self {
            drivetrain: Default::default(),
            arm: Default::default(),
            intake: Default::default(),
            controller: XboxController::new(0).unwrap(),
        }
    }
}