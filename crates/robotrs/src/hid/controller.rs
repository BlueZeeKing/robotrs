use super::{
    axis::{get_axis, AxisFuture, AxisTarget, Initial},
    button::{ButtonFuture, Pressed},
    joystick::Joystick,
    pov::PovFuture,
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
macro_rules! define_povs {
    ($class:ident, $($name:ident = $index:expr => $dir:expr),+) => {
        impl $class {
            $(
                pub fn $name (&self) -> PovFuture<Pressed> {
                    PovFuture::<Pressed>::new(
                        self.joystick.clone(),
                        $index,
                        $dir
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

#[derive(Clone, Copy)]
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

define_povs!(
    XboxController,
    up = 0 => 0,
    up_right = 0 => 45,
    right = 0 => 90,
    down_right = 0 => 135,
    down = 0 => 180,
    down_left = 0 => 225,
    left = 0 => 270,
    up_left = 0 => 315
);

#[derive(Clone, Copy)]
pub struct PS4Controller {
    joystick: Joystick,
}

impl PS4Controller {
    pub fn new(num: u32) -> Result<Self> {
        Ok(Self {
            joystick: Joystick::new(num)?,
        })
    }
}

define_buttons!(
    PS4Controller,
    square = 0,
    cross = 1,
    circle = 2,
    triangle = 3,
    l1 = 4,
    r1 = 5,
    l2_button = 6,
    r2_button = 7,
    share = 8,
    options = 9,
    l3 = 10,
    r3 = 11,
    ps = 12,
    touchpad = 13
);

define_axes!(
    PS4Controller,
    wait_left_x / left_x = 0,
    wait_left_y / left_y = 1,
    wait_right_x / right_x = 2,
    wait_right_y / right_y = 5,
    wait_l2_axis / l2_axis = 3,
    wait_r2_axis / r2_axis = 4
);

define_povs!(
    PS4Controller,
    up = 0 => 0,
    up_right = 0 => 45,
    right = 0 => 90,
    down_right = 0 => 135,
    down = 0 => 180,
    down_left = 0 => 225,
    left = 0 => 270,
    up_left = 0 => 315
);
