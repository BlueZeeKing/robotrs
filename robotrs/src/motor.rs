use impl_trait_for_tuples::impl_for_tuples;

use crate::control::ControlSafe;

#[derive(Clone, Copy)]
pub enum IdleMode {
    Brake,
    Coast,
}

pub trait MotorController: ControlSafe {
    type Error;

    fn set_percent(&mut self, value: f64) -> Result<(), Self::Error>;
}

#[impl_for_tuples(2, 6)]
#[tuple_types_custom_trait_bound(ControlSafe + MotorController)]
impl MotorController for Tuple {
    type Error = anyhow::Error;
    for_tuples!(where #(<Tuple as MotorController>::Error: std::error::Error + Send + Sync + 'static )* );

    fn set_percent(&mut self, value: f64) -> Result<(), Self::Error> {
        for_tuples!( #( Tuple.set_percent(value)?; )* );

        Ok(())
    }
}

pub trait SetIdleMode: MotorController {
    fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), Self::Error>;
}

#[impl_for_tuples(2, 6)]
#[tuple_types_custom_trait_bound(ControlSafe + MotorController + SetIdleMode)]
impl SetIdleMode for Tuple {
    for_tuples!(where #(<Tuple as MotorController>::Error: std::error::Error + Send + Sync + 'static )* );

    fn set_idle_mode(&mut self, idle_mode: IdleMode) -> Result<(), Self::Error> {
        for_tuples!( #( Tuple.set_idle_mode(idle_mode)?; )* );

        Ok(())
    }
}
