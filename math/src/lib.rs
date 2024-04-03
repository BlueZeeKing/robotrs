use std::f32::consts::PI;

pub mod kinematics;
pub mod odometry;

/// Constrain an angle to 0 and 2 pi. All angles are in radians
pub fn normalize_angle(angle: f32) -> f32 {
    if angle > 2.0 * PI {
        angle % (2.0 * PI)
    } else if angle < 0.0 {
        2.0 * PI - (-angle % (2.0 * PI))
    } else {
        angle
    }
}

/// Modify the angles a and b so that they are as close as possible. All angles are in radians
///
/// For example, 1 degree and 359 degrees could result in 361 degrees and 359 degrees
pub fn optimize_angle(a: f32, b: f32) -> (f32, f32) {
    let a = normalize_angle(a);
    let b = normalize_angle(b);

    let b1 = b + 2.0 * PI;
    let b2 = b - 2.0 * PI;

    let diff = (a - b).abs();
    let diff1 = (a - b1).abs();
    let diff2 = (a - b2).abs();

    if diff < diff1 && diff < diff2 {
        (a, b)
    } else if diff1 < diff && diff1 < diff2 {
        (a, b1)
    } else {
        (a, b2)
    }
}
