use std::marker::PhantomData;

use units::Unit;

use crate::{bindings::*, error::REVError, handle_error, FeedbackSensor};

use super::Encoder;

pub struct SparkMaxAbsoluteEncoder<P: Unit, V: Unit> {
    spark_max_handle: c_SparkMax_handle,
    phantom: PhantomData<(P, V)>,
}

impl<P: Unit, V: Unit> SparkMaxAbsoluteEncoder<P, V> {
    pub(crate) fn new(handle: c_SparkMax_handle) -> Self {
        Self {
            spark_max_handle: handle,
            phantom: PhantomData,
        }
    }
}

impl<P: Unit, V: Unit> Encoder<P, V> for SparkMaxAbsoluteEncoder<P, V> {
    fn get_position(&self) -> Result<P, REVError> {
        let mut pos = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetDutyCyclePosition(
                self.spark_max_handle,
                &mut pos
            ))
        }?;

        Ok(P::new(pos))
    }

    fn get_velocity(&self) -> Result<V, REVError> {
        let mut velocity = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetDutyCycleVelocity(
                self.spark_max_handle,
                &mut velocity
            ))
        }?;

        Ok(V::new(velocity))
    }

    fn set_position_conversion_factor(&mut self, factor: f32) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetDutyCyclePositionFactor(
                self.spark_max_handle,
                factor
            ))
        }
    }

    fn set_velocity_conversion_factor(&mut self, factor: f32) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetDutyCycleVelocityFactor(
                self.spark_max_handle,
                factor
            ))
        }
    }

    fn set_inverted(&mut self, inverted: bool) -> Result<(), REVError> {
        unsafe {
            handle_error!(c_SparkMax_SetDutyCycleInverted(
                self.spark_max_handle,
                if inverted { 1 } else { 0 }
            ))
        }
    }
}

impl<P: Unit, V: Unit> FeedbackSensor for SparkMaxAbsoluteEncoder<P, V> {
    fn get_id() -> u32 {
        6
    }

    fn is_handle(&self, handle: c_SparkMax_handle) -> bool {
        self.spark_max_handle == handle
    }
}
