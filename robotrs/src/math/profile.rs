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

pub struct TrapezoidProfile<const C: Constraints, Cont: Controller<State, O>, O = f32> {
    controller: Cont,
    trajectory: Option<Trajectory>,
    last_target: Option<State>,
    phantom: PhantomData<O>,
}

impl<const C: Constraints, Cont: Controller<State, O> + Default, O> Default
    for TrapezoidProfile<C, Cont, O>
{
    fn default() -> Self {
        Self {
            controller: Default::default(),
            trajectory: None,
            last_target: None,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Trajectory {
    start: State,

    final_target: State,

    start_max_speed: f32,
    end_max_speed: f32,
    stop: f32,

    max_velocity: f32,
    max_acceleration: f32,

    initial_time: Duration,
}

impl Trajectory {
    pub fn current(&self, current_time: Duration) -> State {
        let time = (current_time - self.initial_time).as_secs_f32();

        if time < self.start_max_speed {
            let current_velocity = self.start.velocity
                + time * self.max_acceleration * (self.max_velocity - self.start.velocity).signum();
            State {
                velocity: current_velocity,
                position: calculate_trapezoid_area(self.start.velocity, current_velocity, time)
                    + self.start.position,
            }
        } else if time < self.end_max_speed {
            let accel_displacement = calculate_trapezoid_area(
                self.start.velocity,
                self.max_velocity,
                self.start_max_speed,
            );

            State {
                velocity: self.max_velocity,
                position: accel_displacement
                    + (time - self.start_max_speed) * self.max_velocity
                    + self.start.position,
            }
        } else if time < self.stop {
            let accel_displacement = calculate_trapezoid_area(
                self.start.velocity,
                self.max_velocity,
                self.start_max_speed,
            );

            let full_speed_displacement =
                self.max_velocity * (self.end_max_speed - self.start_max_speed);

            let current_velocity = if self.max_velocity > 0.0 {
                self.max_velocity - self.max_acceleration * (time - self.end_max_speed)
            } else {
                self.max_velocity + self.max_acceleration * (time - self.end_max_speed)
            };

            State {
                velocity: current_velocity,
                position: calculate_trapezoid_area(
                    self.max_velocity,
                    current_velocity,
                    time - self.end_max_speed,
                ) + accel_displacement
                    + full_speed_displacement
                    + self.start.position,
            }
        } else {
            self.final_target
        }
    }

    pub fn generate(
        constrains: Constraints,
        start: &State,
        target: &State,
        time: Duration,
    ) -> Self {
        let displacement = target.position - start.position;

        let constraint_max_velocity =
            constrains.max_velocity.get() * (target.position - start.position).signum();
        let no_constraint_max_velocity =
            ((2.0 * constrains.max_acceleration.get() * displacement.abs()
                + start.velocity.powi(2)
                + target.velocity.powi(2))
                / 2.0)
                .sqrt()
                * (target.position - start.position).signum();

        let max_velocity = if farther(no_constraint_max_velocity, constraint_max_velocity) {
            constraint_max_velocity
        } else {
            no_constraint_max_velocity
        };

        let accel_duration =
            (max_velocity - start.velocity).abs() / constrains.max_acceleration.get();

        let accel_displacement =
            calculate_trapezoid_area(start.velocity, max_velocity, accel_duration);

        let decel_duration =
            (target.velocity - max_velocity).abs() / constrains.max_acceleration.get();

        let decel_displacement =
            calculate_trapezoid_area(target.velocity, max_velocity, decel_duration);

        let full_speed_displacement = displacement - accel_displacement - decel_displacement;
        let full_speed_time = full_speed_displacement / max_velocity;

        Self {
            start: start.clone(),
            final_target: target.clone(),

            start_max_speed: accel_duration,
            end_max_speed: accel_duration + full_speed_time,
            stop: accel_duration + full_speed_time + decel_duration,

            max_velocity,
            max_acceleration: constrains.max_acceleration.get(),
            initial_time: time,
        }
    }
}

fn farther(lhs: f32, rhs: f32) -> bool {
    (lhs > 0.0 && lhs > rhs) || (lhs < 0.0 && lhs < rhs)
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
        if self.trajectory.is_none()
            || self
                .last_target
                .is_some_and(|last_target| last_target != *target)
        {
            self.trajectory = Some(Trajectory::generate(C, current, target, time));
            self.last_target = Some(*target);
        }

        self.controller.calculate_with_time(
            current,
            &self.trajectory.as_ref().unwrap().current(time),
            time,
        )
    }
}

pub fn calculate_trapezoid_area(a: f32, b: f32, delta_x: f32) -> f32 {
    (a + b) / 2.0 * delta_x
}
