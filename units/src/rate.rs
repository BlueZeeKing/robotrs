use std::{fmt::Debug, ops::Div};

use crate::{angle::Radian, length::Meter, time::Second, Unit};

#[derive(Clone, Copy)]
pub struct Rate<N: Unit, D: Unit>(pub(crate) N, pub(crate) D);

impl<N: Unit, D: Unit> Unit for Rate<N, D> {
    fn raw(self) -> f32 {
        self.0.raw() / self.1.raw()
    }

    fn name() -> &'static str {
        "Rate"
    }

    fn new(val: f32) -> Self {
        Self(N::new(val), D::new(1.0))
    }
}

impl<N: Unit, D: Unit> Debug for Rate<N, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rate ({} per {}): {}",
            N::name(),
            D::name(),
            (*self).raw()
        )
    }
}

impl<N: Unit, D: Unit, U: Unit> Div<U> for Rate<N, D> {
    type Output = Rate<Rate<N, D>, U>;

    fn div(self, rhs: U) -> Self::Output {
        Rate(self, rhs)
    }
}

pub type MeterPerSecond = Rate<Meter, Second>;
pub type MeterPerSecondSquared = Rate<MeterPerSecond, Second>;
pub type RadianPerSecond = Rate<Radian, Second>;
