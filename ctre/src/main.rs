#[cxx::bridge(namespace = "ctre::phoenix::motorcontrol")]
mod ffi {
    unsafe extern "C++" {
        include!("ctre/include/ctre/phoenix/motorcontrol/can/VictorSPX.h");
        include!("ctre/include/main.hpp");

        #[namespace = "ctre::phoenix::motorcontrol::can"]
        type VictorSPX;

        #[namespace = ""]
        fn new_VictorSPX(id: i32) -> UniquePtr<VictorSPX>;

        // fn Set(&self, mode: VictorSPXControlMode, value: f64);
    }

    #[repr(i32)]
    enum VictorSPXControlMode {
        PercentOutput = 0,
        Position = 1,
        Velocity = 2,
        Follower = 5,
        MotionProfile = 6,
        MotionMagic = 7,
        MotionProfileArc = 10,
        Disabled = 15,
    }

    unsafe extern "C++" {
        include!("ctre/include/ctre/phoenix/motorcontrol/ControlMode.h");

        type VictorSPXControlMode;
    }
}

fn main() {
    let motor = ffi::new_VictorSPX(1);
    // motor.Set(ffi::VictorSPXControlMode { repr: 0 }, 1.0);
}
