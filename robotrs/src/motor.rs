use impl_trait_for_tuples::impl_for_tuples;

use crate::control::ControlSafe;

/// What a motor should do when is is not being actively driven.
#[derive(Clone, Copy)]
pub enum IdleMode {
    /// Brake, slow down
    Brake,
    /// Coast, do not slow down
    Coast,
}

/// Defines a motor controller. This trait is implemented for both the `VictorSPX` and `SparkMax`. This
/// trait is also implemented for any tuple of `MotorController`s
pub trait MotorController: ControlSafe {
    /// The error type for all operations of the motor controller. Also use by [`SetIdleMode`]
    type Error;

    /// This should only be called with a value between `-1` and `1`
    fn set_percent_raw(&mut self, value: f64) -> Result<(), Self::Error>;

    /// Set the percent output of the motor. This automatically clams the value between `-1` and
    /// `1`
    fn set_percent(&mut self, value: f64) -> Result<(), Self::Error> {
        self.set_percent_raw(value.clamp(-1.0, 1.0))
    }
}

#[impl_for_tuples(2, 8)]
#[tuple_types_custom_trait_bound(ControlSafe + MotorController)]
impl MotorController for Tuple {
    type Error = anyhow::Error;
    for_tuples!(where #(<Tuple as MotorController>::Error: std::error::Error + Send + Sync + 'static )* );

    fn set_percent_raw(&mut self, value: f64) -> Result<(), Self::Error> {
        for_tuples!( #( Tuple.set_percent_raw(value)?; )* );

        Ok(())
    }
}

/// Defines a motor contoller that supports setting the idle mode. This is implemented for the
/// `VictorSPX` and `SparkMax`. It is also implemented for tuples.
pub trait SetIdleMode: MotorController {
    /// Set the idle mode of the motor to a specific [`IdleMode`]
    fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), Self::Error>;
}

#[impl_for_tuples(2, 8)]
#[tuple_types_custom_trait_bound(ControlSafe + MotorController + SetIdleMode)]
impl SetIdleMode for Tuple {
    for_tuples!(where #(<Tuple as MotorController>::Error: std::error::Error + Send + Sync + 'static )* );

    fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), Self::Error> {
        for_tuples!( #( Tuple.set_idle_mode(idle_mode)?; )* );

        Ok(())
    }
}
