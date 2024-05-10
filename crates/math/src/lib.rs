#![allow(async_fn_in_trait, incomplete_features)]
#![feature(adt_const_params, const_float_bits_conv)]

use std::{marker::ConstParamTy, ops::Add, time::Duration, u32};

use impl_trait_for_tuples::impl_for_tuples;

use std::f32::consts::PI;

pub mod feedforward;
pub mod filter;
pub mod profile;

pub mod kinematics;
pub mod odometry;

#[cfg(feature = "std")]
fn get_time() -> Duration {
    use std::{sync::OnceLock, time::Instant};

    static START: OnceLock<Instant> = OnceLock::new();

    START.get_or_init(|| Instant::now()).elapsed()
}

#[cfg(feature = "frc")]
pub fn get_time() -> Duration {
    let mut status = 0;

    let time = unsafe { hal_sys::HAL_GetFPGATime(&mut status) };

    if status != 0 {
        panic!("Could not get time");
    }

    Duration::from_micros(time)
}

#[cfg(not(all(feature = "std", feature = "frc")))]
fn get_time() -> Duration {
    use std::{sync::OnceLock, time::Instant};

    static START: OnceLock<Instant> = OnceLock::new();

    START.get_or_init(|| Instant::now()).elapsed()
}

/// Constrain an angle to 0 and 2 pi. All angles are in radians
pub fn normalize_angle(angle: f32) -> f32 {
    if angle > 2.0 * PI {
        angle % (2.0 * PI)
    } else if angle < 0.0 {
        2.0 * PI - (-angle % (2.0 * PI))
    } else {
        angle
    }
}

/// Modify the angles a and b so that they are as close as possible. All angles are in radians
///
/// For example, 1 degree and 359 degrees could result in 361 degrees and 359 degrees
pub fn optimize_angle(a: f32, b: f32) -> (f32, f32) {
    let a = normalize_angle(a);
    let b = normalize_angle(b);

    let b1 = b + 2.0 * PI;
    let b2 = b - 2.0 * PI;

    let diff = (a - b).abs();
    let diff1 = (a - b1).abs();
    let diff2 = (a - b2).abs();

    if diff < diff1 && diff < diff2 {
        (a, b)
    } else if diff1 < diff && diff1 < diff2 {
        (a, b1)
    } else {
        (a, b2)
    }
}

pub trait Controller<State, Output = f32> {
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> Output;

    fn calculate(&mut self, current: &State, target: &State) -> Output {
        self.calculate_with_time(current, target, get_time())
    }
}

#[impl_for_tuples(1, 8)]
impl<State, Output: Add<Output = Output>> Controller<State, Output> for Tuple {
    #[inline]
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
    #[inline]
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
    #[inline]
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

pub type D<const K: Gain> = Derive<P<K>>;

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
    #[inline]
    fn calculate_with_time(&mut self, current: &State, target: &State, time: Duration) -> f32 {
        self.0
            .calculate_with_time(&current.velocity, &target.velocity, time)
    }
}

pub struct Position<C: Controller<f32>>(C);

impl<C: Controller<f32>> Controller<State> for Position<C> {
    #[inline]
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
    #[inline]
    fn calculate_with_time(&mut self, current: &f32, target: &f32, time: Duration) -> f32 {
        if let Some(last_time) = self.last_time {
            let target_vel =
                (target - self.last_target) / (time.as_secs_f32() - last_time.as_secs_f32());

            let current_vel =
                (current - self.last_current) / (time.as_secs_f32() - last_time.as_secs_f32());

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
