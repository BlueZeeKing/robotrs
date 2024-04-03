use std::{
    marker::{ConstParamTy, PhantomData},
    time::Duration,
};

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

#[derive(Default)]
pub struct TrapezoidProfile<const C: Constraints, Cont: Controller<State, O>, O = f32> {
    controller: Cont,
    last_time: Option<Duration>,
    phantom: PhantomData<O>,
}

impl<const C: Constraints, Cont: Controller<State, O>, O> Controller<State, O>
    for TrapezoidProfile<C, Cont, O>
{
    fn calculate_with_time(
        &mut self,
        current: &State,
        target: &State,
        time: std::time::Duration,
    ) -> O {
        let Some(last_time) = self.last_time.take() else {
            self.last_time = Some(time);
            return self
                .controller
                .calculate_with_time(&current, &current, time);
        };

        self.last_time = Some(time);

        let delta_time = (time - last_time).as_secs_f32();

        // Handle slowing down

        let decel_displacement = {
            let delta_v = target.velocity - current.velocity;
            let time = delta_v.abs() / C.max_acceleration.get();

            calculate_trapezoid_area(target.velocity, current.velocity, time)
        };

        let target_displacement = target.position - current.position;

        let should_decel_positive = decel_displacement > 0.0
            && target_displacement > 0.0
            && decel_displacement >= target.position - current.position;
        let should_decel_negative = decel_displacement <= 0.0
            && target_displacement <= 0.0
            && decel_displacement <= target.position - current.position;

        if should_decel_positive || should_decel_negative {
            let new_velocity = current.velocity
                + C.max_acceleration.get()
                    * delta_time
                    * (target.velocity - current.velocity).signum();

            let mut current_target = State {
                velocity: new_velocity,
                position: current.position
                    + calculate_trapezoid_area(new_velocity, current.velocity, delta_time),
            };

            let max_vel_change = C.max_acceleration.get() * delta_time * 2.0;

            if current.position > target.position
                && current_target.position <= target.position
                && current_target.velocity.abs() <= max_vel_change
            {
                current_target.position = target.position;
                current_target.velocity = 0.0;
            } else if current.position < target.position
                && current_target.position >= target.position
                && current_target.velocity.abs() <= max_vel_change
            {
                current_target.position = target.position;
                current_target.velocity = 0.0;
            } else if current.position == target.position
                && current_target.velocity.abs() <= max_vel_change
            {
                current_target.position = target.position;
                current_target.velocity = 0.0;
            }

            return self
                .controller
                .calculate_with_time(current, &current_target, time);
        }

        let max_speed = C.max_velocity.get() * (target.position - current.position).signum();

        let mut new_velocity = current.velocity
            + C.max_acceleration.get() * delta_time * (max_speed - current.velocity).signum();

        if max_speed > 0.0 {
            new_velocity = new_velocity.min(max_speed);
        } else {
            new_velocity = new_velocity.max(max_speed);
        }

        return self.controller.calculate_with_time(
            current,
            &State {
                velocity: new_velocity,
                position: current.position
                    + calculate_trapezoid_area(current.velocity, new_velocity, delta_time),
            },
            time,
        );
    }
}

pub fn calculate_trapezoid_area(a: f32, b: f32, delta_x: f32) -> f32 {
    (a + b) / 2.0 * delta_x
}
