use std::{
    env,
    ffi::CString,
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

use tracing::{debug, error, warn};

use crate::{
    ds,
    robot::AsyncRobot,
    time::{get_time, RawNotifier},
    waker::SimpleHandle,
    DsTracingWriter, PERIODIC_CHECKS,
};

use hal_sys::{
    HAL_HasMain, HAL_Initialize, HAL_ObserveUserProgramAutonomous, HAL_ObserveUserProgramDisabled,
    HAL_ObserveUserProgramStarting, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest,
};

static PERIOD: Duration = Duration::from_millis(20);

pub struct RobotScheduler<'a, R: AsyncRobot> {
    robot: &'a R,
    last_state: ds::State,

    enabled_task: Option<SimpleHandle<'a, anyhow::Result<()>>>,
    auto_task: Option<SimpleHandle<'a, anyhow::Result<()>>>,
    teleop_task: Option<SimpleHandle<'a, anyhow::Result<()>>>,
}

impl<'a, R: AsyncRobot> RobotScheduler<'a, R> {
    fn new(robot: &'a R) -> Self {
        Self {
            robot,
            last_state: ds::State::Disabled,

            enabled_task: None,
            auto_task: None,
            teleop_task: None,
        }
    }

    fn tick(&mut self) {
        let state = ds::State::from_control_word(&ds::get_control_word().unwrap());

        match state {
            ds::State::Auto => unsafe {
                HAL_ObserveUserProgramAutonomous();
            },
            ds::State::Teleop => unsafe {
                HAL_ObserveUserProgramTeleop();
            },
            ds::State::Test => unsafe {
                HAL_ObserveUserProgramTest();
            },
            ds::State::Disabled => unsafe {
                HAL_ObserveUserProgramDisabled();
            },
        }

        if state != self.last_state {
            match state {
                ds::State::Auto => {
                    self.auto_task = Some(SimpleHandle::spawn(self.robot.get_auto_future()));

                    debug!("Auto task started");

                    self.teleop_task = None;
                }
                ds::State::Teleop => {
                    self.teleop_task = Some(SimpleHandle::spawn(self.robot.get_teleop_future()));

                    debug!("Teleop task started");

                    self.auto_task = None;
                }
                ds::State::Test => {
                    self.teleop_task = None;
                    self.auto_task = None;
                }
                ds::State::Disabled => {
                    self.enabled_task = None;
                    self.teleop_task = None;
                    self.auto_task = None;
                }
            }

            if matches!(self.last_state, ds::State::Disabled) {
                self.enabled_task = Some(SimpleHandle::spawn(self.robot.get_enabled_future()));

                debug!("Enabled task started");
            }
        }

        self.last_state = state;

        if let Some(task) = &mut self.enabled_task {
            let err = task.poll();

            if let Some(Err(err)) = err {
                error!("An error occured in the enabled task: {}", err);
            }
        }

        if let Some(task) = &mut self.auto_task {
            let err = task.poll();

            if let Some(Err(err)) = err {
                error!("An error occured in the auto task: {}", err);
            }
        }

        if let Some(task) = &mut self.teleop_task {
            let err = task.poll();

            if let Some(Err(err)) = err {
                error!("An error occured in the teleop task: {}", err);
            }
        }

        for check in PERIODIC_CHECKS {
            check();
        }
    }

    /// This is the main entry function. It starts the robot and schedules all the tasks as well
    /// as sending out the proper DS messages that are required for startup.
    pub fn start_robot(robot: anyhow::Result<R>) -> ! {
        if unsafe { HAL_Initialize(500, 0) } == 0 {
            panic!("Could not start hal");
        }

        if unsafe { HAL_HasMain() } == 1 {
            // TODO: Fix this
            panic!("A main function was given and that is probably wrong (idk)");
        }

        tracing_subscriber::fmt()
            .with_writer(DsTracingWriter {})
            .init();

        if let Err(err) = set_version() {
            tracing::error!("An error occured while sending the version: {}", err);
        }

        unsafe {
            let nt_inst = nt::bindings::NT_GetDefaultInstance();

            // nt::bindings::NT_SubscribeMultiple(nt_inst, ptr::null(), 0, ptr::null()); // TODO: Make
            //                                                                           // sure this
            //                                                                           // is okay

            nt::bindings::NT_StartServer(
                nt_inst,
                CString::new("/home/lvuser/networktables.json")
                    .unwrap()
                    .into_raw(),
                CString::new("").unwrap().into_raw(),
                1735,
                5810,
            );

            let time = Instant::now();

            loop {
                let mode = nt::bindings::NT_GetNetworkMode(nt_inst);

                if mode != nt::bindings::NT_NetworkMode_NT_NET_MODE_STARTING {
                    break;
                } else if time.elapsed() > Duration::from_secs(1) {
                    tracing::error!("NT did not start in time");
                    panic!("NT did not start in time");
                }
            }
        }

        let robot = match robot {
            Ok(robot) => robot,
            Err(err) => {
                error!("An error has occured constructing the robot: {}", err);
                panic!("An error has occured constructing the robot: {}", err);
            }
        };

        let mut scheduler = RobotScheduler::new(&robot);

        let mut time = get_time().unwrap() + PERIOD;
        let mut notifier = RawNotifier::new(time).unwrap();

        unsafe { HAL_ObserveUserProgramStarting() };

        debug!(
            "Robot code started with period of {} milliseconds",
            PERIOD.as_millis()
        );

        loop {
            scheduler.tick();

            notifier = notifier.block_until_alarm().unwrap(); // add error handling
            time += PERIOD;
            if time < get_time().unwrap() {
                warn!(
                    "Loop over run by {} milliseconds",
                    (time - get_time().unwrap()).as_millis()
                );
            }
            notifier.set_time(time).unwrap();
        }
    }
}

fn set_version() -> anyhow::Result<()> {
    let mut version_path = File::create("/tmp/frc_versions/FRC_Lib_Version.ini")?;

    version_path.write_all("Rust ".as_bytes())?;
    version_path.write_all(env::var("WPI_VERSON")?.as_bytes())?;

    Ok(())
}
