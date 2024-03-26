use std::time::Duration;

use futures::future::pending;
use revlib::{
    encoder::{relative::SparkMaxRelativeEncoder, Encoder},
    SparkMax,
};
use robotrs::{
    control::ControlLock,
    hid::{axis::AxisTarget, controller::XboxController},
    robot::AsyncRobot,
    time::delay,
    yield_now, FailableDefault,
};
use utils::trigger::TriggerExt;

fn main() {
    robotrs::scheduler::RobotScheduler::start_robot(|| {
        let mut motor = SparkMax::new(17, revlib::MotorType::Brushless);

        println!("Created motor");
        Ok(Robot {
            encoder: motor.get_relative_encoder()?,
            motor: ControlLock::new(motor),
            controller: XboxController::new(0).unwrap(),
        })
    });
}

pub struct Robot {
    motor: ControlLock<SparkMax>,
    controller: XboxController,
    encoder: SparkMaxRelativeEncoder,
}

impl AsyncRobot for Robot {
    async fn get_auto_future(&self) -> anyhow::Result<()> {
        println!("auto");

        Ok(())
    }

    async fn get_enabled_future(&self) -> anyhow::Result<()> {
        println!("enabled");

        Ok(())
    }

    async fn get_teleop_future(&self) -> anyhow::Result<()> {
        loop {
            dbg!(self.encoder.get_position().unwrap());
            yield_now().await;
        }
    }

    fn configure_bindings<'a>(
        &'a self,
        scheduler: &'a robotrs::scheduler::RobotScheduler<'a, Self>,
    ) -> anyhow::Result<()> {
        self.controller.a().while_pressed(scheduler, || async {
            let mut intake = self.motor.lock().await;
            intake.set(0.5).unwrap();

            pending::<()>().await;
        });

        Ok(())
    }
}
