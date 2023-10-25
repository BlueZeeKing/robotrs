use crate::define_conversion;

#[derive(Debug, Clone, Copy)]
pub struct Percent(f32);

#[derive(Debug, Clone, Copy)]
pub struct Fraction(f32);

pub trait Ratio: Clone + Copy {
    fn get_ratio(self) -> f32;
}

impl Ratio for Percent {
    fn get_ratio(self) -> f32 {
        self.0 / 100.0
    }
}

impl Ratio for Fraction {
    fn get_ratio(self) -> f32 {
        self.0
    }
}

define_conversion!(Fraction, Percent, 100.0);
