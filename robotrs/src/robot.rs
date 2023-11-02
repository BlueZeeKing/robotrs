use std::pin::Pin;

use futures::Future;

pub type Fut = Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'static>>;

pub trait AsyncRobot {
    async fn get_auto_future(&self) -> anyhow::Result<()>;

    async fn get_enabled_future(&self) -> anyhow::Result<()>;

    async fn get_teleop_future(&self) -> anyhow::Result<()>;
}
