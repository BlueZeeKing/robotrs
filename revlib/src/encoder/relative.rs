use std::marker::PhantomData;

use units::Unit;

use crate::{bindings::*, error::REVError, handle_error, FeedbackSensor};

use super::Encoder;

pub struct SparkMaxRelativeEncoder<P: Unit, V: Unit> {
    handle: c_SparkMax_handle,
    phantom: PhantomData<(P, V)>,
}

impl<P: Unit, V: Unit> SparkMaxRelativeEncoder<P, V> {
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

impl<P: Unit, V: Unit> Encoder<P, V> for SparkMaxRelativeEncoder<P, V> {
    fn get_position(&self) -> Result<P, REVError> {
        let mut pos = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderPosition(self.handle, &mut pos))?;
        }

        Ok(P::new(pos))
    }

    fn get_velocity(&self) -> Result<V, REVError> {
        let mut velocity = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderPosition(self.handle, &mut velocity))?;
        }

        Ok(V::new(velocity))
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

impl<P: Unit, V: Unit> FeedbackSensor for SparkMaxRelativeEncoder<P, V> {
    fn get_id() -> u32 {
        1
    }

    fn is_handle(&self, handle: c_SparkMax_handle) -> bool {
        self.handle == handle
    }
}
