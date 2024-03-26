use std::{marker::ConstParamTy, time::Duration};

use super::{ConstFloat, Controller, State};

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
    last_time: Option<Duration>,
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
        let Some(last_time) = self.last_time.take() else {
            self.last_time = Some(time);
            return 0.0;
        };
        self.last_time = Some(time);

        let decel_area = (current.velocity + target.velocity) / 2.0
            * (current.velocity - target.velocity).abs()
            / 2.0;

        let elapsed = (time - last_time).as_secs_f32();

        let target_state = if decel_area <= target.position - current.position {
            State {
                velocity: current.velocity - C.max_acceleration.get() * elapsed,
                position: current.position + current.velocity * elapsed
                    - C.max_acceleration.get() * elapsed.powi(2) / 2.0,
            }
        } else if current.velocity < C.max_velocity.get() {
            State {
                velocity: current.velocity + C.max_acceleration.get() * elapsed,
                position: current.position
                    + current.velocity * elapsed
                    + C.max_acceleration.get() * elapsed.powi(2) / 2.0,
            }
        } else {
            State {
                velocity: current.velocity,
                position: current.position + current.velocity * elapsed,
            }
        };

        self.controller
            .calculate_with_time(&current, &target_state, time)
    }
}

impl<const C: Constraints, Cont: Controller<State> + Default> Default
    for TrapezoidProfile<C, Cont>
{
    fn default() -> Self {
        Self {
            controller: Cont::default(),
            last_time: None,
        }
    }
}
