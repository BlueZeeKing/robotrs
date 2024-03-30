use crate::scheduler::RobotScheduler;

pub trait AsyncRobot: Sized + 'static {
    async fn get_auto_future(&'static self) -> anyhow::Result<()>;

    async fn get_enabled_future(&'static self) -> anyhow::Result<()>;

    async fn get_teleop_future(&'static self) -> anyhow::Result<()>;

    fn configure_bindings<'a>(
        &'static self,
        scheduler: &RobotScheduler<Self>,
    ) -> anyhow::Result<()>;
}
