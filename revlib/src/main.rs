use revlib::SparkMax;

fn main() {
    let mut motor = SparkMax::new(1, revlib::MotorType::Brushed);
    motor.set(1.0).unwrap();
}
