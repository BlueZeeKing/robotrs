use std::{
    marker::{ConstParamTy, PhantomData},
    time::Duration,
};

use super::{ConstFloat, Controller, State};

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Constraints {
    pub max_velocity: ConstFloat,
    pub max_acceleration: ConstFloat,
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

    target: State,

    start_max_speed_time: f32,
    end_max_speed_time: f32,
    stop_time: f32,

    max_velocity: f32,
    max_acceleration: f32,

    initial_time: Duration,
}

impl Trajectory {
    pub fn current(&self, current_time: Duration) -> State {
        let time = (current_time - self.initial_time).as_secs_f32();

        if time < self.start_max_speed_time {
            let current_velocity = self.start.velocity
                + time * self.max_acceleration * (self.max_velocity - self.start.velocity).signum();
            State {
                velocity: current_velocity,
                position: calculate_trapezoid_area(self.start.velocity, current_velocity, time)
                    + self.start.position,
            }
        } else if time < self.end_max_speed_time {
            let accel_displacement = calculate_trapezoid_area(
                self.start.velocity,
                self.max_velocity,
                self.start_max_speed_time,
            );

            State {
                velocity: self.max_velocity,
                position: accel_displacement
                    + (time - self.start_max_speed_time) * self.max_velocity
                    + self.start.position,
            }
        } else if time < self.stop_time {
            let accel_displacement = calculate_trapezoid_area(
                self.start.velocity,
                self.max_velocity,
                self.start_max_speed_time,
            );

            let full_speed_displacement =
                self.max_velocity * (self.end_max_speed_time - self.start_max_speed_time);

            let current_velocity = self.max_velocity
                + (time - self.end_max_speed_time)
                    * self.max_acceleration
                    * (self.target.velocity - self.max_velocity).signum();

            State {
                velocity: current_velocity,
                position: calculate_trapezoid_area(
                    self.max_velocity,
                    current_velocity,
                    time - self.end_max_speed_time,
                ) + accel_displacement
                    + full_speed_displacement
                    + self.start.position,
            }
        } else {
            self.target
        }
    }

    pub fn generate(
        constrains: Constraints,
        start: &State,
        target: &State,
        time: Duration,
    ) -> Self {
        let displacement_to_target = calculate_trapezoid_area_from_slope(
            start.velocity,
            target.velocity,
            constrains.max_acceleration.get(),
        );

        let displacement = target.position - start.position;

        let max_vel = if displacement > displacement_to_target {
            let dx_to_max = calculate_trapezoid_area_from_slope(
                start.velocity,
                constrains.max_velocity.get(),
                constrains.max_acceleration.get(),
            );

            let dx_from_max = calculate_trapezoid_area_from_slope(
                constrains.max_velocity.get(),
                target.velocity,
                constrains.max_acceleration.get(),
            );

            if displacement - dx_to_max - dx_from_max > 0.0 {
                constrains.max_velocity.get()
            } else {
                let extra_displacement = (displacement - dx_to_max - dx_from_max) / 2.0;

                let new_dx_to_max = dx_to_max + extra_displacement;

                (start.velocity.powi(2) + 2.0 * constrains.max_acceleration.get() * new_dx_to_max)
                    .sqrt()
            }
        } else {
            let dx_to_max = calculate_trapezoid_area_from_slope(
                start.velocity,
                -constrains.max_velocity.get(),
                constrains.max_acceleration.get(),
            );

            let dx_from_max = calculate_trapezoid_area_from_slope(
                -constrains.max_velocity.get(),
                target.velocity,
                constrains.max_acceleration.get(),
            );

            if displacement - dx_to_max - dx_from_max < 0.0 {
                -constrains.max_velocity.get()
            } else {
                let extra_displacement = (displacement - dx_to_max - dx_from_max) / 2.0;

                let new_dx_to_max = dx_to_max + extra_displacement;

                -1.0 * (start.velocity.powi(2)
                    + -2.0 * constrains.max_acceleration.get() * new_dx_to_max)
                    .sqrt()
                    .abs()
            }
        };

        let dt_to_max = (start.velocity - max_vel).abs() / constrains.max_acceleration.get();
        let dx_to_max = calculate_trapezoid_area(start.velocity, max_vel, dt_to_max);

        let dt_from_max = (max_vel - target.velocity).abs() / constrains.max_acceleration.get();
        let dx_from_max = calculate_trapezoid_area(max_vel, target.velocity, dt_from_max);

        let max_speed_dx = displacement - dx_to_max - dx_from_max;
        let max_speed_time = max_speed_dx / max_vel;

        Self {
            start: *start,
            target: *target,
            start_max_speed_time: dt_to_max,
            end_max_speed_time: dt_to_max + max_speed_time,
            stop_time: dt_to_max + max_speed_time + dt_from_max,
            max_velocity: max_vel,
            max_acceleration: constrains.max_acceleration.get(),
            initial_time: time,
        }
    }
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

pub fn calculate_trapezoid_area_from_slope(a: f32, b: f32, v: f32) -> f32 {
    calculate_trapezoid_area(a, b, (a - b).abs() / v)
}
