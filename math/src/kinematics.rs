use std::f32::{consts::PI, EPSILON};

use nalgebra::{matrix, ComplexField, Dim, SMatrix, Vector2, Vector3, VectorView2, U1, U2};

use crate::normalize_angle;

/// A trait that takes in a the requested robot speeds and returns the state the drivetrain
/// should be in to achieve those speeds.
///
/// The vector is \[x, y, rotation\] in the WPILib robot coordinate system
/// ([Link to WPILib docs](https://docs.wpilib.org/en/stable/docs/software/basic-programming/coordinate-system.html#wpilib-coordinate-system)).
pub trait Kinematics {
    type State;

    /// Convert from robot speeds to drive train state
    fn inverse(&self, robot_speeds: Vector3<f32>) -> Self::State;
    /// Convert from drive train state to robot speeds
    fn forward(&self, state: Self::State) -> Vector3<f32>;
}

/// An implementation of the `Kinematics` trait for a swerve drivetrain.
#[derive(Clone)]
pub struct SwerveKinematics {
    pub inverse_matrix: SMatrix<f32, 8, 3>,
    pub forward_matrix: SMatrix<f32, 3, 8>,
    pub positons: [Vector2<f32>; 4],
}

/// Calculates the positions of the modules on the robot given the track width and wheel base.
///
/// # Order:
///
/// 1: Front Left
/// 2: Front Right
/// 3: Back Left
/// 4: Back Right
pub fn module_positions_from_dimensions(track_width: f32, wheel_base: f32) -> [Vector2<f32>; 4] {
    let half_track_width = track_width / 2.0;
    let half_wheel_base = wheel_base / 2.0;

    [
        Vector2::new(half_wheel_base, half_track_width),
        Vector2::new(half_wheel_base, -half_track_width),
        Vector2::new(-half_wheel_base, half_track_width),
        Vector2::new(-half_wheel_base, -half_track_width),
    ]
}

impl SwerveKinematics {
    /// Creates a new `SwerveKinematics` struct with the given module positions.
    ///
    /// See [module_positions_from_dimensions] for an easy way to get these positions.
    pub fn new(positions: [Vector2<f32>; 4]) -> Self {
        let positions2 = positions.clone();

        let inverse_matrix = nalgebra::matrix![
            1.0, 0.0, -positions[0].y;
            0.0, 1.0, positions[0].x;
            1.0, 0.0, -positions[1].y;
            0.0, 1.0, positions[1].x;
            1.0, 0.0, -positions[2].y;
            0.0, 1.0, positions[2].x;
            1.0, 0.0, -positions[3].y;
            0.0, 1.0, positions[3].x
        ];

        let forward_matrix = inverse_matrix
            .pseudo_inverse(EPSILON)
            .expect("Could not calculate swerve forward kinematics matrix");

        Self {
            inverse_matrix,
            forward_matrix,
            positons: positions2,
        }
    }

    /// ALign the wheels in an X to prevent the robot from moving.
    pub fn brake(&self) -> [SwerveState; 4] {
        let mut states = self.positons.iter().map(|position| {
            let mut state = SwerveState::from(position.as_view::<_, _, U1, U2>());
            state.stop();
            state
        });

        [
            states.next().unwrap(),
            states.next().unwrap(),
            states.next().unwrap(),
            states.next().unwrap(),
        ]
    }

    /// Rescale the speeds of each module to not be higher than the maximum
    pub fn scale(mut states: [SwerveState; 4], max_speed: f32) -> [SwerveState; 4] {
        let max = states
            .iter()
            .map(|state| state.drive)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if max > max_speed {
            let scale = max_speed / max;

            states.iter_mut().for_each(|val| val.drive *= scale);
        }

        states
    }
}

impl Kinematics for SwerveKinematics {
    type State = [SwerveState; 4];

    fn inverse(&self, robot_speeds: Vector3<f32>) -> Self::State {
        let output = self.inverse_matrix * robot_speeds;

        [
            output.fixed_rows::<2>(0).into(),
            output.fixed_rows::<2>(2).into(),
            output.fixed_rows::<2>(4).into(),
            output.fixed_rows::<2>(6).into(),
        ]
    }

    fn forward(&self, state: Self::State) -> Vector3<f32> {
        let input = matrix![
            state[0].drive * state[0].angle.cos();
            state[0].drive * state[0].angle.sin();
            state[1].drive * state[1].angle.cos();
            state[1].drive * state[1].angle.sin();
            state[2].drive * state[2].angle.cos();
            state[2].drive * state[2].angle.sin();
            state[3].drive * state[3].angle.cos();
            state[3].drive * state[3].angle.sin()
        ];

        self.forward_matrix * input
    }
}

/// The state of an individual swerve module.
#[derive(Debug, Clone, Copy)]
pub struct SwerveState {
    /// The speed or displacment of the module in m/s or m
    pub drive: f32,
    /// The angle of the module in radians
    pub angle: f32,
}

impl<'a, R: Dim, C: Dim> From<VectorView2<'a, f32, R, C>> for SwerveState {
    fn from(vector: VectorView2<'a, f32, R, C>) -> Self {
        Self {
            drive: vector.magnitude(),
            angle: vector.index((1, 0)).atan2(*vector.index((0, 0))),
        }
    }
}

impl SwerveState {
    pub fn new(angle: f32, drive: f32) -> Self {
        Self { angle, drive }
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn get_drive(&self) -> f32 {
        self.drive
    }

    /// Stop the module from moving
    pub fn stop(&mut self) {
        self.drive = 0.0
    }

    /// Optimize the module state to prevent the module from spinning 180 degrees
    pub fn optimize(self, old: SwerveState) -> SwerveState {
        if self.drive.abs() < f32::EPSILON {
            return SwerveState {
                drive: 0.0,
                angle: old.angle,
            };
        }

        let new_angle = normalize_angle(self.angle);
        let old_angle = normalize_angle(old.angle);
        let diff = new_angle - old_angle;

        if diff.abs() < PI / 2.0 {
            self
        } else {
            Self {
                angle: normalize_angle(new_angle - PI),
                drive: -1.0 * self.drive,
            }
        }
    }
}
