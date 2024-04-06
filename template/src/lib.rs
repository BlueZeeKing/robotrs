use robotrs::robot::AsyncRobot;

pub struct Robot {}

impl Robot {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl AsyncRobot for Robot {
    async fn get_auto_future(&'static self) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_enabled_future(&'static self) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_teleop_future(&'static self) -> anyhow::Result<()> {
        todo!()
    }

    fn configure_bindings(
        &'static self,
        scheduler: &robotrs::scheduler::RobotScheduler<Self>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
