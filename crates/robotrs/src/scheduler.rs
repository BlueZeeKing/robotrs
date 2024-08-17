use core::panic;
use std::{
    fs::File,
    io::Write,
    sync::OnceLock,
    thread::{self, ThreadId},
    time::Duration,
};

use anyhow::anyhow;
use async_task::{Runnable, Task};
use flume::{unbounded, Receiver, Sender};
use futures::{Future, FutureExt, TryFutureExt};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{ds, robot::AsyncRobot, status_to_result, time::RawNotifier, PERIODIC_CHECKS};

use hal_sys::*;

mod cancellation;

pub use cancellation::{guard, CancellationHandle};

static PERIOD: Duration = Duration::from_millis(20);
static TASK_SENDER: OnceLock<(Sender<Runnable>, ThreadId)> = OnceLock::new();

/// Panics if not called after the robot is scheduled or during the robot create closure
pub fn spawn<O, F: Future<Output = O> + 'static>(fut: F) -> Task<Option<O>> {
    spawn_inner(guard(fut))
}

fn spawn_inner<O, F: Future<Output = O> + 'static>(fut: F) -> Task<O> {
    let (sender, thread_id) = TASK_SENDER.get().unwrap();

    if *thread_id != thread::current().id() {
        panic!("Spawn must only be called from the robot thread");
    }

    // SAFETY:
    //
    // Runnable never changes thread so F can be !Send
    // Future is 'static
    // schedule is send, sync, and 'static
    let (runnable, task) = unsafe {
        async_task::spawn_unchecked(fut, move |runnable| {
            sender.send(runnable).expect("Robot is not initialized")
        })
    };

    runnable.schedule();

    task
}

pub struct RobotScheduler<R: AsyncRobot> {
    robot: &'static R,
    last_state: ds::State,

    task_receiver: Receiver<Runnable>,

    enabled_task: Option<Task<anyhow::Result<()>>>,
    auto_task: Option<Task<anyhow::Result<()>>>,
    teleop_task: Option<Task<anyhow::Result<()>>>,
}

impl<R: AsyncRobot> RobotScheduler<R> {
    fn new(robot: &'static R, task_receiver: Receiver<Runnable>) -> Self {
        Self {
            robot,
            last_state: ds::State::Disabled,

            task_receiver,

            enabled_task: None,
            auto_task: None,
            teleop_task: None,
        }
    }

    pub fn add_binding<
        F: Fn() -> Fut + 'static,
        Fut: Future<Output = anyhow::Result<()>> + 'static,
    >(
        &self,
        func: F,
    ) {
        spawn(async move {
            loop {
                match guard(func()).await {
                    Some(Err(err)) => {
                        error!("An error occurred in a binding: {}", err);
                    }
                    None => {
                        warn!("Binding was canceled");
                    }
                    _ => {}
                }
            }
        })
        .detach();
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
                    self.auto_task = Some(spawn_inner(
                        guard(self.robot.get_auto_future())
                            .map(|val| match val {
                                Some(val) => val,
                                None => Err(anyhow!("Task cancelled")),
                            })
                            .inspect_err(|err| {
                                error!("An error occurred in the autonomous task: {}", err)
                            }),
                    ));

                    debug!("Auto task started");

                    self.teleop_task = None;
                }
                ds::State::Teleop => {
                    self.teleop_task = Some(spawn_inner(
                        guard(self.robot.get_teleop_future())
                            .map(|val| match val {
                                Some(val) => val,
                                None => Err(anyhow!("Task cancelled")),
                            })
                            .inspect_err(|err| {
                                error!("An error occurred in the teleop task: {}", err)
                            }),
                    ));

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
                self.enabled_task = Some(spawn_inner(
                    guard(self.robot.get_enabled_future())
                        .map(|val| match val {
                            Some(val) => val,
                            None => Err(anyhow!("Task cancelled")),
                        })
                        .inspect_err(|err| {
                            error!("An error occurred in the enabled task: {}", err)
                        }),
                ));

                debug!("Enabled task started");
            }
        }

        self.last_state = state;

        for task in self.task_receiver.try_iter() {
            task.run();
        }

        for check in PERIODIC_CHECKS {
            check();
        }
    }

    /// This is the main entry function. It starts the robot and schedules all the tasks as well
    /// as sending out the proper DS messages that are required for startup.
    pub fn start_robot<F: Fn() -> anyhow::Result<R> + Send + 'static>(robot: F) -> ! {
        if unsafe { HAL_Initialize(500, 0) } == 0 {
            panic!("Could not start hal");
        }

        if let Err(err) = unsafe { status_to_result!(HAL_SetNotifierThreadPriority(1, 40)) } {
            panic!("Could not set notifier thread priority: {}", err);
        }

        tracing_subscriber::registry()
            .with(EnvFilter::builder().parse("trace").unwrap())
            .with(
                tracing_subscriber::fmt::layer().event_format(
                    tracing_subscriber::fmt::format()
                        .without_time()
                        .with_ansi(false),
                ),
            )
            .init();

        if let Err(err) = set_version() {
            tracing::error!("An error occurred while sending the version: {}", err);
        }

        thread::spawn(move || {
            let (task_sender, task_receiver) = unbounded();

            TASK_SENDER
                .set((task_sender.clone(), thread::current().id()))
                .expect("Robot was already started");

            info!("Starting robot");

            let robot = match robot() {
                Ok(robot) => robot,
                Err(err) => {
                    error!("An error has occurred constructing the robot: {}", err);
                    panic!("An error has occurred constructing the robot: {}", err);
                }
            };

            let robot = Box::leak::<'static>(Box::new(robot));

            info!("Robot started");

            let mut scheduler = RobotScheduler::new(robot, task_receiver);

            robot
                .configure_bindings(&scheduler)
                .expect("An error occurred configuring bindings");

            RawNotifier::set_thread_priority().unwrap();

            // let mut time = get_time() + PERIOD;
            // let mut notifier = RawNotifier::new(time).unwrap();

            RawNotifier::set_thread_priority().unwrap();

            unsafe { HAL_ObserveUserProgramStarting() };

            info!(
                "Robot code started with period of {} milliseconds",
                PERIOD.as_millis()
            );

            loop {
                scheduler.tick();

                thread::sleep(PERIOD);

                // notifier = notifier
                //     .block_until_alarm()
                //     .expect("Stopping because periodic notifier failed"); // add error handling
                // time += PERIOD;
                // if time < get_time() {
                //     warn!(
                //         "Loop over run by {} milliseconds",
                //         (get_time() - time).as_millis()
                //     );
                // }
                // notifier.set_time(time).unwrap();
            }
        });

        unsafe {
            HAL_RunMain();
        }
        unsafe {
            HAL_ExitMain();
        }

        panic!("Done!");
    }
}

fn set_version() -> anyhow::Result<()> {
    let mut version_path = File::create("/tmp/frc_versions/FRC_Lib_Version.ini")?;

    version_path.write_all("Rust ".as_bytes())?;
    version_path.write_all(hal_sys::WPI_VERSION.as_bytes())?;

    Ok(())
}
