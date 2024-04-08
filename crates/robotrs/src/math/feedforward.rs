use super::{Controller, Derive, Gain, State, Velocity as VelocityExtractor};

pub struct Static<const K: Gain>;

impl<const K: Gain> Controller<State> for Static<K> {
    #[inline]
    fn calculate_with_time(
        &mut self,
        _current: &State,
        target: &State,
        _time: std::time::Duration,
    ) -> f32 {
        target.velocity.signum() * K.get()
    }
}

pub struct TargetProportional<const K: Gain>;

impl<const K: Gain> Controller<f32> for TargetProportional<K> {
    #[inline]
    fn calculate_with_time(
        &mut self,
        _current: &f32,
        target: &f32,
        _time: std::time::Duration,
    ) -> f32 {
        target * K.get()
    }
}

impl<const K: Gain> Default for TargetProportional<K> {
    fn default() -> Self {
        Self
    }
}

impl<const K: Gain> Default for Static<K> {
    fn default() -> Self {
        Self
    }
}

pub type Velocity<const K: Gain> = VelocityExtractor<TargetProportional<K>>;

pub type Acceleration<const K: Gain> = VelocityExtractor<Derive<TargetProportional<K>>>;

pub struct Elevator<const K: Gain>;

impl<const K: Gain> Controller<State> for Elevator<K> {
    #[inline]
    fn calculate_with_time(
        &mut self,
        _current: &State,
        _target: &State,
        _time: std::time::Duration,
    ) -> f32 {
        K.get()
    }
}

pub struct Arm<const K: Gain>;

impl<const K: Gain> Controller<State> for Arm<K> {
    #[inline]
    fn calculate_with_time(
        &mut self,
        _current: &State,
        target: &State,
        _time: std::time::Duration,
    ) -> f32 {
        K.get() * target.position.cos()
    }
}

impl<const K: Gain> Default for Arm<K> {
    fn default() -> Self {
        Self
    }
}

impl<const K: Gain> Default for Elevator<K> {
    fn default() -> Self {
        Self
    }
}

pub type FullArm<const KS: Gain, const KG: Gain, const KV: Gain, const KA: Gain> =
    (Static<KS>, Arm<KG>, Velocity<KV>, Acceleration<KA>);

pub type FullElevator<const KS: Gain, const KG: Gain, const KV: Gain, const KA: Gain> =
    (Static<KS>, Elevator<KG>, Velocity<KV>, Acceleration<KA>);
