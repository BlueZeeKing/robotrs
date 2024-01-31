use std::marker::ConstParamTy;

use super::{feedforward::FullArm, ConstFloat, Controller, Gain, Position, State, P, PID};

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Constraints {
    max_velocity: ConstFloat,
    max_acceleration: ConstFloat,
}

impl Constraints {
    pub const fn new(max_velocity: f32, max_acceleration: f32) -> Self {
        Self {
            max_velocity: ConstFloat::new(max_velocity),
            max_acceleration: ConstFloat::new(max_acceleration),
        }
    }
}

pub struct TrapezoidProfile<const C: Constraints, Cont: Controller<State>> {
    controller: Cont,
}

impl<const C: Constraints, Cont: Controller<State>> Controller<State>
    for TrapezoidProfile<C, Cont>
{
    fn calculate_with_time(
        &mut self,
        current: &State,
        target: &State,
        time: std::time::Duration,
    ) -> f32 {
        todo!();
    }
}

impl<const C: Constraints, Cont: Controller<State> + Default> Default
    for TrapezoidProfile<C, Cont>
{
    fn default() -> Self {
        Self {
            controller: Cont::default(),
        }
    }
}
