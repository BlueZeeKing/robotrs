use math::{
    kinematics::{module_positions_from_dimensions, Kinematics, SwerveKinematics},
    odometry::Odometry,
};
use nalgebra::Vector3;

fn main() {
    let drive = SwerveKinematics::new(module_positions_from_dimensions(1.0, 1.0));

    let odometry = Odometry::new(drive.clone(), Vector3::new(0.0, 0.0, 0.0), 0.0);

    odometry.update(drive.inverse(Vector3::new(1.0, 1.0, 0.0)), 0.0);

    dbg!(odometry.get_pose());
}
