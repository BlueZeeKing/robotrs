#![feature(try_blocks)]

pub mod mechanism;
pub mod subsystem;
pub mod trigger;

pub use tracing;

#[macro_export]
macro_rules! while_pressed_subsystem {
    ($trigger:expr, $subsystem:expr, $priority:expr, $body:expr) => {
        $trigger.while_pressed(move || async move {
            if $subsystem.run($body, $priority).await.is_err() {
                $crate::tracing::warn!("Trigger was interuppted");
            }
        });
    };
}

#[macro_export]
macro_rules! on_pressed_subsystem {
    ($trigger:expr, $subsystem:expr, $priority:expr, $body:expr) => {
        $trigger.on_pressed(move || async move {
            if $subsystem.run($body, $priority).await.is_err() {
                $crate::tracing::warn!("Trigger was interuppted");
            }
        });
    };
}

#[macro_export]
macro_rules! wait {
    () => {
        ::futures::future::pending::<()>().await;
    };
}
