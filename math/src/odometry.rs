use std::{cell::Cell, rc::Rc};

use nalgebra::{matrix, Rotation2, Vector3};

use crate::kinematics::Kinematics;

#[derive(Clone)]
/// This object calculates the robots positon over time by integrating the speeds derived from
/// forward kinematics. This uses an [Rc] internally to allow it to be cloned and maintain the same
/// internal state.
pub struct Odometry<K: Kinematics> {
    inner: Rc<InnerOdometry<K>>,
}

pub(crate) struct InnerOdometry<K: Kinematics> {
    kinematics: K,
    pose: Cell<Vector3<f32>>,
}

impl<K: Kinematics> InnerOdometry<K> {
    pub(crate) fn new(kinematics: K, starting_pose: Vector3<f32>) -> Self {
        Self {
            kinematics,
            pose: Cell::new(starting_pose),
        }
    }

    pub(crate) fn update(&self, value: K::State, current_heading: f32) {
        let displacement = self.kinematics.forward(value);

        let delta_theta = displacement.z;

        let sin_theta = delta_theta.sin();
        let cos_theta = delta_theta.cos();

        let pose_exponential_matrix = if delta_theta.abs() > f32::EPSILON {
            let sin_theta_expr = sin_theta / delta_theta;

            matrix![sin_theta_expr, (cos_theta - 1.0) / delta_theta, 0.0;
                (1.0 - cos_theta) / delta_theta, sin_theta_expr, 0.0;
                0.0, 0.0, 1.0]
        } else {
            let sin_theta_expr = 1.0 - delta_theta.powi(2) / 6.0;

            matrix![sin_theta_expr, -delta_theta / 2.0, 0.0;
                delta_theta / 2.0, sin_theta_expr, 0.0;
                0.0, 0.0, 1.0]
        };

        let mut rotation_matrix = Rotation2::new(current_heading)
            .matrix()
            .fixed_resize::<3, 3>(0.0);

        *rotation_matrix.index_mut((2, 2)) = 1.0;

        let final_displacement = rotation_matrix * pose_exponential_matrix * displacement;

        self.pose.set(self.pose.get() + final_displacement);
    }

    pub(crate) fn get_pose(&self) -> Vector3<f32> {
        self.pose.get()
    }

    pub(crate) fn set_pose(&self, pose: Vector3<f32>) {
        self.pose.set(pose);
    }
}

impl<K: Kinematics> Odometry<K> {
    pub fn new(kinematics: K, starting_pose: Vector3<f32>) -> Self {
        Self {
            inner: Rc::new(InnerOdometry::new(kinematics, starting_pose)),
        }
    }

    /// The state in this case should represent displacement not speed. This updates the current
    /// estimate using the displacement of the drivetrain and the current heading.
    pub fn update(&self, value: K::State, current_heading: f32) {
        self.inner.update(value, current_heading)
    }

    /// Return the current pose as x, y, rotation
    pub fn get_pose(&self) -> Vector3<f32> {
        self.inner.get_pose()
    }

    /// Set the current pose as x, y, rotation
    pub fn set_pose(&self, pose: Vector3<f32>) {
        self.inner.set_pose(pose)
    }
}
