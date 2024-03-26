use std::{
    cell::OnceCell,
    fs::File,
    io::{self, Write},
    thread,
    time::Duration,
};

use async_task::{Runnable, Task};
use flume::{unbounded, Receiver, Sender};
use futures::{Future, TryFutureExt};
use tracing::{debug, error};

use crate::{ds, robot::AsyncRobot, DsTracingWriter, PERIODIC_CHECKS};

use hal_sys::{
    HAL_HasMain, HAL_Initialize, HAL_ObserveUserProgramAutonomous, HAL_ObserveUserProgramDisabled,
    HAL_ObserveUserProgramStarting, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest,
};

static PERIOD: Duration = Duration::from_millis(20);
thread_local! {
    static TASK_SENDER: OnceCell<Sender<Runnable>> = OnceCell::new();
}

/// Panics if not called after the robot is scheduled or during the robot create closure
pub fn spawn<O, F: Future<Output = O> + 'static>(fut: F) -> Task<O> {
    // SAFETY:
    //
    // Runnable never changes thread so F can be !Send
    // Future is 'static
    // schedule is send, sync, and 'static
    let (runnable, task) = unsafe {
        async_task::spawn_unchecked(fut, move |runnable| {
            TASK_SENDER.with(|sender| {
                sender
                    .get()
                    .unwrap()
                    .send(runnable)
                    .expect("Robot is not initialized or called from the wrong thread")
            });
        })
    };

    runnable.schedule();

    task
}

pub struct RobotScheduler<'a, R: AsyncRobot> {
    robot: &'a R,
    last_state: ds::State,

    task_sender: Sender<Runnable>,
    task_receiver: Receiver<Runnable>,

    enabled_task: Option<Task<anyhow::Result<()>>>,
    auto_task: Option<Task<anyhow::Result<()>>>,
    teleop_task: Option<Task<anyhow::Result<()>>>,
}

impl<'a, R: AsyncRobot> RobotScheduler<'a, R> {
    fn new(robot: &'a R, task_sender: Sender<Runnable>, task_receiver: Receiver<Runnable>) -> Self {
        Self {
            robot,
            last_state: ds::State::Disabled,

            task_sender,
            task_receiver,

            enabled_task: None,
            auto_task: None,
            teleop_task: None,
        }
    }

    pub fn add_binding<F: Fn() -> Fut, Fut: Future<Output = anyhow::Result<()>> + 'a>(
        &self,
        func: F,
    ) {
        self.schedule(async move {
            loop {
                if let Err(err) = func().await {
                    error!("An error occurred in a binding: {}", err);
                }
            }
        })
        .detach();
    }

    pub fn schedule<O, F: Future<Output = O> + 'a>(&self, fut: F) -> Task<O> {
        let sender = self.task_sender.to_owned();

        // SAFETY:
        //
        // Runnable never changes thread so F can be !Send
        // Future is forced to outlive 'a which is longer than self. Since runnable has same
        // lifetime as self, it will never outlive 'a
        // schedule is send, sync, and 'static
        let (runnable, task) = unsafe {
            async_task::spawn_unchecked(fut, move |runnable| sender.send(runnable).unwrap())
        };

        runnable.schedule();

        task
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
            dbg!(&state);
            match state {
                ds::State::Auto => {
                    self.auto_task = Some(self.schedule(self.robot.get_auto_future().inspect_err(
                        |err| error!("An error occurred in the autonomous task: {}", err),
                    )));

                    debug!("Auto task started");

                    self.teleop_task = None;
                }
                ds::State::Teleop => {
                    self.teleop_task = Some(self.schedule(
                        self.robot.get_teleop_future().inspect_err(|err| {
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
                self.enabled_task = Some(self.schedule(
                    self.robot.get_enabled_future().inspect_err(|err| {
                        error!("An error occurred in the enabled task: {}", err)
                    }),
                ));

                debug!("Enabled task started");
            }
        }

        self.last_state = state;

        for task in self.task_receiver.drain() {
            task.run();
        }

        for check in PERIODIC_CHECKS {
            check();
        }
    }

    /// This is the main entry function. It starts the robot and schedules all the tasks as well
    /// as sending out the proper DS messages that are required for startup.
    pub fn start_robot<F: Fn() -> anyhow::Result<R>>(robot: F) -> ! {
        if unsafe { HAL_Initialize(500, 0) } == 0 {
            panic!("Could not start hal");
        }

        if unsafe { HAL_HasMain() } == 1 {
            // TODO: Fix this
            panic!("A main function was given and that is probably wrong (idk)");
        }

        tracing_subscriber::fmt()
            .with_writer(DsTracingWriter {})
            .with_writer(io::stderr)
            .init();

        if let Err(err) = set_version() {
            tracing::error!("An error occurred while sending the version: {}", err);
        }

        let (task_sender, task_receiver) = unbounded();

        TASK_SENDER.with(|sender| {
            sender
                .set(task_sender.clone())
                .expect("Robot was already started")
        });

        println!("Starting robot");

        let robot = match robot() {
            Ok(robot) => robot,
            Err(err) => {
                error!("An error has occurred constructing the robot: {}", err);
                panic!("An error has occurred constructing the robot: {}", err);
            }
        };

        println!("Robot started");

        let mut scheduler = RobotScheduler::new(&robot, task_sender, task_receiver);

        robot
            .configure_bindings(&scheduler)
            .expect("An error occurred configuring bindings");

        // let mut time = get_time().unwrap() + PERIOD;
        // let mut notifier = RawNotifier::new(time).unwrap();

        unsafe { HAL_ObserveUserProgramStarting() };

        println!(
            "Robot code started with period of {} milliseconds",
            PERIOD.as_millis()
        );

        loop {
            scheduler.tick();

            // notifier = notifier.block_until_alarm().unwrap(); // add error handling
            // dbg!(get_time().unwrap().as_millis(), time);
            // time += PERIOD;
            thread::sleep(PERIOD);
            // if time < get_time().unwrap() {
            //     warn!(
            //         "Loop over run by {} milliseconds",
            //         (get_time().unwrap() - time).as_millis()
            //     );
            // }
            // notifier.set_time(time).unwrap();
        }
    }
}

fn set_version() -> anyhow::Result<()> {
    let mut version_path = File::create("/tmp/frc_versions/FRC_Lib_Version.ini")?;

    version_path.write_all("Rust ".as_bytes())?;
    version_path.write_all(hal_sys::WPI_VERSION.as_bytes())?;

    Ok(())
}
