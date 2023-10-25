use super::{
    axis::{get_axis, AxisFuture, AxisTarget, Initial},
    button::{ButtonFuture, Pressed},
    joystick::Joystick,
};
use crate::error::Result;

#[macro_export]
macro_rules! define_buttons {
    ($class:ident, $($name:ident/$name_callback:ident = $index:expr),+) => {
        impl $class {
            $(
                pub fn $name (&self) -> ButtonFuture<Pressed> {
                    ButtonFuture::<Pressed>::new(
                        self.joystick.get_num(),
                        $index,
                    )
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! define_axes {
    ($class:ident, $($future_name:ident/$name:ident = $index:expr),+) => {
        impl $class {
            $(
                pub fn $future_name (&self, target: AxisTarget) -> AxisFuture<Initial> {
                    AxisFuture::new(
                        self.joystick.get_num(),
                        $index,
                        target
                    )
                }

                pub fn $name (&self) -> Result<units::ratio::Fraction> {
                    get_axis(&self.joystick.get_axes_data()?, $index)
                }
            )+
        }
    };
}

pub struct XboxController {
    joystick: Joystick,
}

impl XboxController {
    pub fn new(num: u32) -> Result<Self> {
        Ok(Self {
            joystick: Joystick::new(num)?,
        })
    }
}

define_buttons!(
    XboxController,
    a / on_a = 0,
    b / on_b = 1,
    x / on_x = 2,
    y / on_y = 3,
    back / on_back = 6,
    start / on_start = 7,
    left_bumper / on_left_bumper = 4,
    right_bumper / on_right_bumper = 5,
    left_stick / on_left_stick = 8,
    right_stick / on_right_stick = 9
);

define_axes!(
    XboxController,
    wait_left_x / left_x = 0,
    wait_left_y / left_y = 1,
    wait_right_x / right_x = 4,
    wait_right_y / right_y = 5,
    wait_left_trigger / left_trigger = 2,
    wait_right_trigger / right_trigger = 3
);
