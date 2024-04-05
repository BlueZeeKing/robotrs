use std::{marker::ConstParamTy, ops::Add, time::Duration, u32};

use impl_trait_for_tuples::impl_for_tuples;

use crate::time::get_time;

pub mod feedforward;
pub mod filter;
pub mod profile;

pub trait Controller<State, Output = f32> {
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> Output;

    fn calculate(&mut self, current: &State, target: &State) -> crate::error::Result<Output> {
        Ok(self.calculate_with_time(current, target, get_time()?))
    }
}

#[impl_for_tuples(1, 8)]
impl<State, Output: Add<Output = Output>> Controller<State, Output> for Tuple {
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> Output {
        for_tuples!( #( Tuple.calculate_with_time(current, target, time) )+* )
    }
}

#[derive(PartialEq, Eq, ConstParamTy)]
pub struct ConstFloat(u32);

impl ConstFloat {
    pub const fn new(val: f32) -> Self {
        Self(u32::from_ne_bytes(val.to_ne_bytes()))
    }

    pub const fn get(&self) -> f32 {
        f32::from_ne_bytes(self.0.to_ne_bytes())
    }
}

pub type Gain = ConstFloat;

pub struct P<const K: Gain>;

impl<const K: Gain> Controller<f32> for P<K> {
    fn calculate_with_time(&mut self, current: &f32, target: &f32, _time: Duration) -> f32 {
        (target - current) * K.get()
    }
}

impl<const K: Gain> Default for P<K> {
    fn default() -> Self {
        Self
    }
}

pub struct I<const K: Gain> {
    last_time: Option<Duration>,
    accum: f32,
}

impl<const K: Gain> Controller<f32> for I<K> {
    fn calculate_with_time(&mut self, current: &f32, target: &f32, time: Duration) -> f32 {
        if let Some(last_time) = self.last_time {
            self.accum += (target - current) * (time.as_secs_f32() - last_time.as_secs_f32());
            self.last_time = Some(time);
            self.accum * K.get()
        } else {
            self.last_time = Some(time);
            0.0
        }
    }
}

impl<const K: Gain> Default for I<K> {
    fn default() -> Self {
        Self {
            last_time: None,
            accum: 0.0,
        }
    }
}

type D<const K: Gain> = Derive<P<K>>;

pub type PID<const KP: Gain, const KI: Gain, const KD: Gain> = (P<KP>, I<KP>, D<KP>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub position: f32,
    pub velocity: f32,
}

impl State {
    pub fn new(position: f32, velocity: f32) -> Self {
        Self { position, velocity }
    }
}

pub struct Velocity<C: Controller<f32>>(C);

impl<C: Controller<f32>> Controller<State> for Velocity<C> {
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> f32 {
        self.0
            .calculate_with_time(&current.velocity, &target.velocity, time)
    }
}

pub struct Position<C: Controller<f32>>(C);

impl<C: Controller<f32>> Controller<State> for Position<C> {
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> f32 {
        self.0
            .calculate_with_time(&current.position, &target.position, time)
    }
}

impl<C: Controller<f32> + Default> Default for Velocity<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C: Controller<f32> + Default> Default for Position<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct Derive<C: Controller<f32>> {
    controller: C,
    last_time: Option<Duration>,
    last_current: f32,
    last_target: f32,
}

impl<C: Controller<f32> + Default> Default for Derive<C> {
    fn default() -> Self {
        Self {
            controller: C::default(),
            last_time: None,
            last_target: 0.0,
            last_current: 0.0,
        }
    }
}

impl<C: Controller<f32>> Controller<f32> for Derive<C> {
    fn calculate_with_time(&mut self, current: &f32, target: &f32, time: Duration) -> f32 {
        if let Some(last_time) = self.last_time {
            let target_vel =
                (target - self.last_target) / (time.as_secs_f32() / last_time.as_secs_f32());

            let current_vel =
                (current - self.last_current) / (time.as_secs_f32() / last_time.as_secs_f32());

            self.last_current = *current;
            self.last_target = *target;
            self.last_time = Some(time);

            self.controller
                .calculate_with_time(&current_vel, &target_vel, time)
        } else {
            self.last_current = *current;
            self.last_target = *target;
            self.last_time = Some(time);

            0.0
        }
    }
}
