use uom::si::{Dimension, Quantity, Units};

use crate::{error::REVError, FeedbackSensor};

pub mod absolute;
pub mod relative;

pub trait Encoder<VelD: Dimension, VelU: Units<f32>, PosD: Dimension, PosU: Units<f32>>:
    FeedbackSensor
{
    fn get_position(&self) -> Result<Quantity<PosU, PosU, f32>, REVError>;
    fn get_velocity(&self) -> Result<Quantity<VelD, VelU, f32>, REVError>;

    fn set_position_conversion_factor(&mut self, factor: f32) -> Result<(), REVError>;
    fn set_velocity_conversion_factor(&mut self, factor: f32) -> Result<(), REVError>;

    fn set_inverted(&mut self, inverted: bool) -> Result<(), REVError>;
}
