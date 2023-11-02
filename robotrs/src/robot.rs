pub trait AsyncRobot {
    async fn get_auto_future(&self) -> anyhow::Result<()>;

    async fn get_enabled_future(&self) -> anyhow::Result<()>;

    async fn get_teleop_future(&self) -> anyhow::Result<()>;
}
