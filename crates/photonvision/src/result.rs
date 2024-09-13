use std::{io::Read, time::Duration};

use nalgebra::{Quaternion, Translation3};

use crate::decode::{decode_f64, decode_i16, decode_i32, decode_u8};

#[derive(Debug)]
pub struct PipelineResult {
    pub timestamp: f64,
    pub latency: f64,
    pub targets: Vec<Target>,
    pub position_estimate: PositionEstimateResult,
}

impl PipelineResult {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            latency: decode_f64(&mut reader)?,
            targets: (0..decode_u8(&mut reader)?)
                .map(|_| Target::decode(&mut reader))
                .collect::<Result<_, _>>()?,
            position_estimate: PositionEstimateResult::decode(&mut reader)?,
            timestamp: 0.0,
        })
    }

    pub(crate) fn set_timestamp(&mut self, timestamp: f64) {
        self.timestamp = timestamp;
    }

    pub fn timestamp(&self) -> Duration {
        Duration::from_secs_f64(self.timestamp)
    }
}

#[derive(Debug)]
pub struct Target {
    pub yaw: f64,
    pub pitch: f64,
    pub area: f64,
    pub skew: f64,
    pub apriltag_id: i32,
    pub best_camera_to_target: Transform,
    pub alt_camera_to_target: Transform,
    pub pose_ambiguity: f64,
    pub bounding_box_corners: Vec<Corner>,
    pub detected_corners: Vec<Corner>,
}

impl Target {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            yaw: decode_f64(&mut reader)?,
            pitch: decode_f64(&mut reader)?,
            area: decode_f64(&mut reader)?,
            skew: decode_f64(&mut reader)?,
            apriltag_id: decode_i32(&mut reader)?,
            best_camera_to_target: Transform::decode(&mut reader)?,
            alt_camera_to_target: Transform::decode(&mut reader)?,
            pose_ambiguity: decode_f64(&mut reader)?,
            bounding_box_corners: (0..4)
                .map(|_| Corner::decode(&mut reader))
                .collect::<Result<_, _>>()?,
            detected_corners: (0..decode_u8(&mut reader)?)
                .map(|_| Corner::decode(&mut reader))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct Corner {
    pub x: f64,
    pub y: f64,
}

impl Corner {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            x: decode_f64(&mut reader)?,
            y: decode_f64(&mut reader)?,
        })
    }
}

#[derive(Debug)]
pub struct Transform {
    pub translation: Translation3<f64>,
    pub rotation: Quaternion<f64>,
}

impl Transform {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            translation: Translation3::new(
                decode_f64(&mut reader)?,
                decode_f64(&mut reader)?,
                decode_f64(&mut reader)?,
            ),
            rotation: Quaternion::new(
                decode_f64(&mut reader)?,
                decode_f64(&mut reader)?,
                decode_f64(&mut reader)?,
                decode_f64(&mut reader)?,
            ),
        })
    }
}

#[derive(Debug)]
pub struct PositionEstimateResult {
    pub apriltag_ids: Vec<i16>,
    pub estimate: Option<PositionEstimate>,
}

#[derive(Debug)]
pub struct PositionEstimate {
    pub best: Transform,
    pub best_reprojection_err: f64,
    pub alt: Transform,
    pub alt_reprojection_err: f64,
    pub ambiguity: f64,
}

impl PositionEstimate {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            best: Transform::decode(&mut reader)?,
            alt: Transform::decode(&mut reader)?,
            best_reprojection_err: decode_f64(&mut reader)?,
            alt_reprojection_err: decode_f64(&mut reader)?,
            ambiguity: decode_f64(&mut reader)?,
        })
    }
}

impl PositionEstimateResult {
    pub fn decode(mut reader: impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            estimate: {
                if dbg!(decode_u8(&mut reader)?) == 1 {
                    Some(PositionEstimate::decode(&mut reader)?)
                } else {
                    None
                }
            },
            apriltag_ids: (0..32)
                .map(|_| decode_i16(&mut reader))
                .filter(|val| val.as_ref().map(|val| *val != -1).unwrap_or(true))
                .collect::<Result<_, _>>()?,
        })
    }
}
