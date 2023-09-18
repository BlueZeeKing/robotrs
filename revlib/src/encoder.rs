use crate::{error::REVError, FeedbackSensor};

pub mod absolute;
pub mod relative;

pub trait Encoder: FeedbackSensor {
    fn get_position(&self) -> Result<f32, REVError>;
    fn get_velocity(&self) -> Result<f32, REVError>;

    fn set_position_conversion_factor(&mut self, factor: f32) -> Result<(), REVError>;
    fn set_velocity_conversion_factor(&mut self, factor: f32) -> Result<(), REVError>;

    fn set_inverted(&mut self, inverted: bool) -> Result<(), REVError>;
}
