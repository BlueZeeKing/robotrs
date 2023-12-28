use std::marker::PhantomData;

use uom::si::{Dimension, Quantity, Units};

use crate::{bindings::*, error::REVError, handle_error, FeedbackSensor};

use super::Encoder;

pub struct SparkMaxRelativeEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    handle: c_SparkMax_handle,
    phantom: PhantomData<(VelD, VelU, PosD, PosU)>,
}

impl<VelD, VelU, PosD, PosU> SparkMaxRelativeEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    pub(crate) fn new(handle: c_SparkMax_handle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }

    pub fn set_position(&mut self, value: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetEncoderPosition(self.handle, value)) }
    }
}

impl<VelD, VelU, PosD, PosU> Encoder<VelD, VelU, PosD, PosU>
    for SparkMaxRelativeEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    fn get_position(&self) -> Result<Quantity<PosD, PosU, f32>, REVError> {
        let mut pos = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderPosition(self.handle, &mut pos))?;
        }

        Ok(Quantity {
            value: pos,
            dimension: PhantomData,
            units: PhantomData,
        })
    }

    fn get_velocity(&self) -> Result<Quantity<VelD, VelU, f32>, REVError> {
        let mut velocity = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderPosition(self.handle, &mut velocity))?;
        }

        Ok(Quantity {
            value: velocity,
            dimension: PhantomData,
            units: PhantomData,
        })
    }

    fn set_position_conversion_factor(&mut self, factor: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetPositionConversionFactor(self.handle, factor)) }
    }

    fn set_velocity_conversion_factor(&mut self, factor: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetVelocityConversionFactor(self.handle, factor)) }
    }

    fn set_inverted(&mut self, inverted: bool) -> Result<(), REVError> {
        let mut motor_type: c_SparkMax_MotorType = 0;
        unsafe { handle_error!(c_SparkMax_GetMotorType(self.handle, &mut motor_type)) }?;

        if motor_type == c_SparkMax_MotorType_c_SparkMax_kBrushless {
            return Err(REVError::General);
        }

        unsafe {
            handle_error!(c_SparkMax_SetInverted(
                self.handle,
                if inverted { 1 } else { 0 }
            ))
        }
    }
}

impl<VelD, VelU, PosD, PosU> FeedbackSensor for SparkMaxRelativeEncoder<VelD, VelU, PosD, PosU>
where
    VelD: Dimension,
    VelU: Units<f32>,
    PosD: Dimension,
    PosU: Units<f32>,
{
    fn get_id() -> u32 {
        1
    }

    fn is_handle(&self, handle: c_SparkMax_handle) -> bool {
        self.handle == handle
    }
}
