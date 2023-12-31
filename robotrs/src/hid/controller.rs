use super::{
    axis::{get_axis, AxisFuture, AxisTarget, Initial},
    button::{ButtonFuture, Pressed},
    joystick::Joystick,
};
use crate::error::Result;

#[macro_export]
macro_rules! define_buttons {
    ($class:ident, $($name:ident = $index:expr),+) => {
        impl $class {
            $(
                pub fn $name (&self) -> ButtonFuture<Pressed> {
                    ButtonFuture::<Pressed>::new(
                        self.joystick.clone(),
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

                pub fn $name (&self) -> Result<f32> {
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
    a = 0,
    b = 1,
    x = 2,
    y = 3,
    back = 6,
    start = 7,
    left_bumper = 4,
    right_bumper = 5,
    left_stick = 8,
    right_stick = 9
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
