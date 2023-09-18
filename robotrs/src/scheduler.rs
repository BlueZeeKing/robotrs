use std::{
    env,
    ffi::CString,
    fs::File,
    io::Write,
    rc::Rc,
    time::{Duration, Instant},
};

use anyhow::Result;
use futures::{
    executor::{LocalPool, LocalSpawner},
    future::RemoteHandle,
    task::LocalSpawnExt,
    Future, TryFutureExt,
};
use tracing::{debug, error, warn};

use crate::{
    ds,
    robot::AsyncRobot,
    time::{get_time, RawNotifier},
    DsTracingWriter, PERIODIC_CHECKS,
};

use hal_sys::{
    HAL_HasMain, HAL_Initialize, HAL_ObserveUserProgramAutonomous, HAL_ObserveUserProgramDisabled,
    HAL_ObserveUserProgramStarting, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest,
};

fn handle_error(location: &'static str) -> impl FnOnce(&anyhow::Error) {
    move |err| error!("An error occured in {location}: {err}")
}

pub struct Spawner {
    spawner: LocalSpawner,
}

impl Spawner {
    pub fn spawn<F: Future<Output = Result<()>> + 'static>(&self, fut: F) {
        self.spawner
            .spawn_local_with_handle(fut.inspect_err(handle_error("bindings")))
            .unwrap()
            .forget();
    }
}

static PERIOD: Duration = Duration::from_millis(20);

pub struct RobotScheduler<R: AsyncRobot> {
    robot: Rc<R>,
    last_state: ds::State,
    rt: LocalPool,

    enabled_task: Option<RemoteHandle<Result<()>>>,
    auto_task: Option<RemoteHandle<Result<()>>>,
    teleop_task: Option<RemoteHandle<Result<()>>>,
}

impl<R: AsyncRobot> RobotScheduler<R> {
    fn new(robot: R) -> Self {
        let scheduler = Self {
            robot: Rc::new(robot),
            last_state: ds::State::Disabled,
            rt: LocalPool::new(),

            enabled_task: None,
            auto_task: None,
            teleop_task: None,
        };

        let robot = scheduler.robot.clone();

        R::create_bindings(
            robot,
            &Spawner {
                spawner: scheduler.rt.spawner(),
            },
        );

        scheduler
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
                    self.auto_task = Some(
                        self.rt
                            .spawner()
                            .spawn_local_with_handle(
                                self.robot
                                    .clone()
                                    .get_auto_future()
                                    .inspect_err(handle_error("auto")),
                            )
                            .unwrap(),
                    );

                    debug!("Auto task started");

                    self.teleop_task = None;
                }
                ds::State::Teleop => {
                    self.teleop_task = Some(
                        self.rt
                            .spawner()
                            .spawn_local_with_handle(
                                self.robot
                                    .clone()
                                    .get_teleop_future()
                                    .inspect_err(handle_error("teleop")),
                            )
                            .unwrap(),
                    );

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
                self.enabled_task = Some(
                    self.rt
                        .spawner()
                        .spawn_local_with_handle(
                            self.robot
                                .clone()
                                .get_enabled_future()
                                .inspect_err(handle_error("enabled task")),
                        )
                        .unwrap(),
                );

                debug!("Enabled task started");
            }
        }

        self.last_state = state;

        self.rt.run_until_stalled();

        for check in PERIODIC_CHECKS {
            check();
        }
    }

    fn set_version() -> anyhow::Result<()> {
        let mut version_path = File::create("/tmp/frc_versions/FRC_Lib_Version.ini")?;

        version_path.write_all("Rust ".as_bytes())?;
        version_path.write_all(env::var("WPI_VERSON")?.as_bytes())?;

        Ok(())
    }

    /// This is the main entry function. It starts the robot and schedules all the tasks as well
    /// as sending out the proper DS messages that are required for startup.
    pub fn start_robot(robot: R) -> ! {
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

        if let Err(err) = Self::set_version() {
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

        let mut scheduler = RobotScheduler::new(robot);

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
