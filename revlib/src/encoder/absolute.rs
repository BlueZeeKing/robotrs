use std::marker::PhantomData;

use uom::si::{Dimension, Quantity, Units};

use crate::{bindings::*, error::REVError, handle_error, FeedbackSensor};

use super::Encoder;

pub struct SparkMaxAbsoluteEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    spark_max_handle: c_SparkMax_handle,
    phantom: PhantomData<(VelD, VelU, PosD, PosU)>,
}

impl<VelD, VelU, PosD, PosU> SparkMaxAbsoluteEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    pub(crate) fn new(handle: c_SparkMax_handle) -> Self {
        Self {
            spark_max_handle: handle,
            phantom: PhantomData,
        }
    }
}

impl<VelD, VelU, PosD, PosU> Encoder<VelD, VelU, PosD, PosU>
    for SparkMaxAbsoluteEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    fn get_position(&self) -> Result<Quantity<PosD, PosU, f32>, REVError> {
        let mut pos = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetDutyCyclePosition(
                self.spark_max_handle,
                &mut pos
            ))
        }?;

        Ok(Quantity {
            value: pos,
            dimension: PhantomData,
            units: PhantomData,
        })
    }

    fn get_velocity(&self) -> Result<Quantity<VelD, VelU, f32>, REVError> {
        let mut velocity = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetDutyCycleVelocity(
                self.spark_max_handle,
                &mut velocity
            ))
        }?;

        Ok(Quantity {
            value: velocity,
            dimension: PhantomData,
            units: PhantomData,
        })
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

impl<VelD, VelU, PosD, PosU> FeedbackSensor for SparkMaxAbsoluteEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    fn get_id() -> u32 {
        6
    }

    fn is_handle(&self, handle: c_SparkMax_handle) -> bool {
        self.spark_max_handle == handle
    }
}
