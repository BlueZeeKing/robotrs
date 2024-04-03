use revlib::SparkMax;

fn main() {
    let mut motor = SparkMax::new(1, revlib::MotorType::Brushed).unwrap();
    motor.set(1.0).unwrap();
}
