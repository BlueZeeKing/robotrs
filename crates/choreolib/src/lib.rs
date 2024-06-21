use std::{future::Future, time::Duration};

use defer_lite::defer;
use math::{get_time, Controller, State};
use nalgebra::Vector3;

pub use choreo_macros::choreo;

/// This represents an entire path
#[derive(Debug)]
pub struct Path<'a> {
    pub samples: &'a [TrajectoryPoint],
}

/// One singular point along a [Path]
#[derive(Debug)]
pub struct TrajectoryPoint {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub angular_velocity: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub timestamp: Duration,
}

fn linear_interpolate(
    first: f32,
    first_time: f32,
    second_time: f32,
    second: f32,
    time: f32,
) -> f32 {
    (first * (second_time - time) + second * (first_time - time)) / (second_time - first_time)
}

fn interpolate_point(a: &TrajectoryPoint, b: &TrajectoryPoint, time: Duration) -> TrajectoryPoint {
    let time_secs = time.as_secs_f32();
    let first_time = a.timestamp.as_secs_f32();
    let second_time = b.timestamp.as_secs_f32();

    TrajectoryPoint {
        x: linear_interpolate(a.x, first_time, second_time, b.x, time_secs),
        y: linear_interpolate(a.y, first_time, second_time, b.y, time_secs),
        heading: linear_interpolate(a.heading, first_time, second_time, b.heading, time_secs),
        angular_velocity: linear_interpolate(
            a.angular_velocity,
            first_time,
            second_time,
            b.angular_velocity,
            time_secs,
        ),
        velocity_x: linear_interpolate(
            a.velocity_x,
            first_time,
            second_time,
            b.velocity_x,
            time_secs,
        ),
        velocity_y: linear_interpolate(
            a.velocity_y,
            first_time,
            second_time,
            b.velocity_y,
            time_secs,
        ),
        timestamp: time,
    }
}

/// Follow the given path and pass the points into a closure. Consumer gets called every loop,
/// which defaults to every 20ms. The points are generated using linear interpolation from the
/// path. See [simple_controller] for an easy way to consume the points.
///
/// # Example
///
/// ```rust
/// follow_path(
///     &paths::Test::PATH,
///     simple_controller(
///         || odometry.get_pose(),
///         LinearController::default(),
///         LinearController::default(),
///         AngularController::default(),
///         |x, y, heading| drivetrain.set_input_raw(Vector2::new(x, y), heading),
///     ),
/// )
/// .await
///```
pub async fn follow_path<O, E, Func, Fut>(
    path: &Path<'_>,
    mut consumer: O,
    start_time: Option<Duration>,
    mut wait: Func,
) -> Result<(), E>
where
    O: FnMut(&TrajectoryPoint) -> Result<(), E>,
    Func: FnMut() -> Fut,
    Fut: Future,
{
    let mut current_idx = 0;
    let start_time = start_time.unwrap_or_else(get_time);

    loop {
        let time = get_time() - start_time;

        if time == path.samples[current_idx].timestamp {
            consumer(&path.samples[current_idx])?;
            wait().await;
            continue;
        }

        if current_idx == path.samples.len() - 1 {
            break Ok(());
        }

        if time >= path.samples[current_idx].timestamp {
            current_idx += 1;
            continue;
        }

        consumer(&interpolate_point(
            &path.samples[current_idx],
            &path.samples[current_idx + 1],
            time,
        ))?;
        wait().await;
    }
}

#[cfg(feature = "frc")]
pub async fn follow_path_subsystem<T, O, E>(
    path: &Path<'_>,
    mut consumer: O,
    subsystem: &utils::subsystem::Subsystem<T>,
    priority: impl utils::subsystem::AsPriority + Clone,
) -> Result<(), E>
where
    O: FnMut(&mut T, &TrajectoryPoint) -> Result<(), E>,
    T: robotrs::control::ControlSafe,
{
    use robotrs::{scheduler::guard, yield_now};

    let mut elapsed = Duration::new(0, 0);

    loop {
        let guard_result = guard(async {
            let mut subsystem = subsystem.lock(priority.clone()).await;

            let start_time = get_time();
            let ajusted_start_time = Some(start_time - elapsed);

            defer! {
                elapsed += get_time() - start_time;
            }

            follow_path(
                path,
                |point| consumer(&mut subsystem, point),
                ajusted_start_time,
                yield_now,
            )
            .await?;

            Ok(())
        })
        .await;

        if let Ok(result) = guard_result {
            break result;
        }
    }
}

fn deriver(mut initial: Option<f32>) -> impl FnMut(f32) -> f32 {
    let mut last_time = get_time();

    move |value| {
        let curr_time = get_time();
        let Some(last_val) = initial else {
            last_time = curr_time;
            initial = Some(value);
            return 0.0;
        };

        let res = (value - last_val) / (curr_time.as_secs_f32() - last_time.as_secs_f32());

        last_time = curr_time;
        initial = Some(value);

        res
    }
}

/// This function takes in a closure that returns the robot's current pose, 3 controllers for each
/// axis, and a closure that consumes the final 3 control outputs. See [follow_path] for example.
pub fn simple_controller<E, O1, O2, O3>(
    mut pose: impl FnMut() -> Vector3<f32>,
    mut x_controller: impl Controller<State, O1>,
    mut y_controller: impl Controller<State, O2>,
    mut angle_controller: impl Controller<State, O3>,
    mut consumer: impl FnMut(O1, O2, O3) -> Result<(), E>,
) -> impl FnMut(&TrajectoryPoint) -> Result<(), E> {
    let mut x_velocity = deriver(None);
    let mut y_velocity = deriver(None);
    let mut angle_velocity = deriver(None);

    move |point| {
        let current_pose = pose();
        let x = current_pose.x;
        let y = current_pose.y;
        let heading = current_pose.z;

        let x_vel = x_velocity(x);
        let y_vel = y_velocity(x);
        let angle_vel = angle_velocity(x);

        consumer(
            x_controller.calculate(
                &State {
                    position: x,
                    velocity: x_vel,
                },
                &State {
                    position: point.x,
                    velocity: point.velocity_x,
                },
            ),
            y_controller.calculate(
                &State {
                    position: y,
                    velocity: y_vel,
                },
                &State {
                    position: point.y,
                    velocity: point.velocity_y,
                },
            ),
            angle_controller.calculate(
                &State {
                    position: heading,
                    velocity: angle_vel,
                },
                &State {
                    position: point.heading,
                    velocity: point.angular_velocity,
                },
            ),
        )
    }
}
