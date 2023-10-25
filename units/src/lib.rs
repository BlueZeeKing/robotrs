use std::ops::{Add, Sub};

use ratio::Ratio;

pub mod angle;
pub mod length;
pub mod rate;
pub mod ratio;
pub mod time;

#[macro_export]
macro_rules! define_unit {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name(f32);

        impl $name {
            pub const fn constant(val: f32) -> Self {
                Self(val)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl crate::Unit for $name {
            fn raw(self) -> f32 {
                self.0
            }

            fn new(num: f32) -> Self {
                $name(num)
            }

            fn scale<R: crate::ratio::Ratio>(self, ratio: R) -> Self {
                Self(self.0 * ratio.get_ratio())
            }

            fn name() -> &'static str {
                stringify!($name)
            }
        }

        impl<U: crate::Unit> std::ops::Div<U> for $name {
            type Output = crate::rate::Rate<$name, U>;

            fn div(self, rhs: U) -> Self::Output {
                crate::rate::Rate(self, rhs)
            }
        }

        impl<R: crate::ratio::Ratio> std::ops::Mul<R> for $name {
            type Output = $name;

            fn mul(self, rhs: R) -> Self::Output {
                crate::Unit::new(crate::Unit::raw(self) * rhs.get_ratio())
            }
        }
    };
}

#[macro_export]
macro_rules! define_conversion {
    ($a:ident, $b:ident, $factor:literal) => {
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                Self(value.0 * $factor)
            }
        }

        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                Self(value.0 / $factor)
            }
        }
    };
}

pub trait Unit: Clone + Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> {
    fn raw(self) -> f32;
    fn new(val: f32) -> Self;
    fn scale<R: Ratio>(self, ratio: R) -> Self;

    fn name() -> &'static str;
}

define_unit!(Raw);
