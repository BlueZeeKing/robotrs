use crate::{bindings::*, error::REVError, handle_error, FeedbackSensor};

use super::Encoder;

pub struct SparkMaxRelativeEncoder {
    handle: c_SparkMax_handle,
}

impl SparkMaxRelativeEncoder {
    pub(crate) fn new(handle: c_SparkMax_handle) -> Self {
        Self { handle }
    }

    pub fn set_position(&mut self, value: f32) -> Result<(), REVError> {
        unsafe { handle_error!(c_SparkMax_SetEncoderPosition(self.handle, value)) }
    }
}

impl Encoder for SparkMaxRelativeEncoder {
    fn get_position(&self) -> Result<f32, REVError> {
        let mut pos = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderPosition(self.handle, &mut pos))?;
        }

        Ok(pos)
    }

    fn get_velocity(&self) -> Result<f32, REVError> {
        let mut velocity = 0.0;

        unsafe {
            handle_error!(c_SparkMax_GetEncoderVelocity(self.handle, &mut velocity))?;
        }

        Ok(velocity)
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

impl FeedbackSensor for SparkMaxRelativeEncoder {
    fn get_id() -> u32 {
        1
    }

    fn is_handle(&self, handle: c_SparkMax_handle) -> bool {
        self.handle == handle
    }
}
