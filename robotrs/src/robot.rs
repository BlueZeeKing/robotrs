use crate::scheduler::RobotScheduler;

pub trait AsyncRobot: Sized {
    async fn get_auto_future(&self) -> anyhow::Result<()>;

    async fn get_enabled_future(&self) -> anyhow::Result<()>;

    async fn get_teleop_future(&self) -> anyhow::Result<()>;

    fn configure_bindings<'a>(
        &'a self,
        scheduler: &'a RobotScheduler<'a, Self>,
    ) -> anyhow::Result<()>;
}
