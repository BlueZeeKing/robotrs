#![feature(try_blocks)]

pub mod error;
pub mod mechanism;
pub mod subsystem;
pub mod trigger;

use robotrs::ds::{get_state, wait_for_state_change, State};
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

pub async fn wait_for_enabled() {
    if get_state() != State::Disabled {
        return;
    }

    loop {
        if wait_for_state_change().await != State::Disabled {
            return;
        }
    }
}

pub async fn wait_for_disabled() {
    if get_state() == State::Disabled {
        return;
    }

    loop {
        if wait_for_state_change().await == State::Disabled {
            return;
        }
    }
}
