#![feature(try_blocks)]

pub mod error;
pub mod mechanism;
pub mod subsystem;
pub mod trigger;

pub use tracing;

/// A macro that creates a future that will never resolve. This is useful when using [trigger::TriggerExt::while_pressed].
#[macro_export]
macro_rules! wait {
    () => {
        ::futures::future::pending::<()>().await;
    };
}

/// Create a periodic task that will run forever.
///
/// # Example
///
/// ```rust
/// periodic!([drivetrain = &self.drivetrain => 0], async {
///     drivetrain.drive(0.0, 0.0);
/// });
/// ```
#[macro_export]
macro_rules! periodic {
    ([$($name:ident = $subsystem:expr => $priority:expr),*], $task:expr) => {
        loop {
            let Ok(val) = robotrs::scheduler::guard(async {
                $(
                    let mut $name = $subsystem.lock($priority).await
                )*;

                loop {
                    $task.await
                }
            }).await else {
                continue;
            };

            $crate::tracing::warn!("Periodic failed: {:?}", val);
        }
    };
    ($name:ident = $subsystem:expr => $priority:expr, $task:expr) => {
        $crate::periodic!([$name = $subsystem => $priority], $task);
    };
}
