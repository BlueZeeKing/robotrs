use ctre::VictorSPX;
use robotrs::motor::MotorController;

fn main() {
    let mut motor = VictorSPX::new(1);

    motor.set_percent(0.5).unwrap();
}
