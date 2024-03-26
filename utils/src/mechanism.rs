use flume::{Receiver, Sender};
use robotrs::{
    control::ControlSafe, math::Controller, motor::MotorController, scheduler, yield_now,
};
use std::fmt::Debug;
use tracing::{error, warn};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MechanismResult {
    Completed,
    Interrupted,
    Occupied,
}

impl MechanismResult {
    pub fn merge(self, other: MechanismResult) -> MechanismResult {
        if self == MechanismResult::Occupied || other == MechanismResult::Occupied {
            MechanismResult::Occupied
        } else if self == MechanismResult::Interrupted || other == MechanismResult::Interrupted {
            MechanismResult::Interrupted
        } else {
            MechanismResult::Completed
        }
    }
}

enum MechanismRequest<I> {
    Value(ValueMechanismRequest<I>),
    Stop,
}

struct ValueMechanismRequest<I> {
    state: I,
    priority: bool,
    response: oneshot::Sender<MechanismResult>,
}

pub enum MechanismState<O> {
    Value(O),
    Stop,
}

pub struct Mechanism<I: 'static, E: 'static + Debug> {
    sender: Sender<MechanismRequest<I>>,
    errors: Receiver<MechanismError<E>>,
}

#[derive(Debug)]
pub enum MechanismError<E: Debug> {
    User(E),
    Time(robotrs::error::Error),
}

impl<E: Debug> From<E> for MechanismError<E> {
    fn from(e: E) -> Self {
        Self::User(e)
    }
}

impl<I: 'static, E: Debug + 'static> Mechanism<I, E> {
    pub fn new<
        O,
        C: Controller<I, O> + 'static,
        Supply: FnMut() -> Result<I, E> + 'static,
        Consume: FnMut(MechanismState<O>) -> Result<(), E> + 'static,
        Check: FnMut(&I, &I) -> bool + 'static,
    >(
        mut controller: C,
        mut supplier: Supply,
        mut consumer: Consume,
        mut at_setpoint: Check,
        initial: Option<I>,
        hold_state: bool,
    ) -> Self {
        let (sender, receiver): (Sender<MechanismRequest<I>>, Receiver<MechanismRequest<I>>) =
            flume::unbounded();

        let (errors_sender, errors_receiver): (
            Sender<MechanismError<E>>,
            Receiver<MechanismError<E>>,
        ) = flume::unbounded();

        scheduler::spawn(async move {
            let mut priority = false;
            let mut response: Option<oneshot::Sender<MechanismResult>> = None;
            let mut target = initial;

            loop {
                let result: Result<(), MechanismError<E>> = try {
                    match receiver.try_recv() {
                        Ok(request) => match request {
                            MechanismRequest::Value(request) => {
                                if (request.priority && !priority)
                                    || (!request.priority && response.is_none())
                                {
                                    if let Some(response) = response.take() {
                                        if response.send(MechanismResult::Interrupted).is_err() {
                                            warn!("Mechanism response channel has been closed");
                                        }
                                    }
                                    target = Some(request.state);
                                    response = Some(request.response);
                                    priority = request.priority;
                                } else if priority {
                                    if request.response.send(MechanismResult::Occupied).is_err() {
                                        warn!("Mechanism response channel has been closed");
                                    }
                                }
                            }
                            MechanismRequest::Stop => {
                                target = None;

                                consumer(MechanismState::Stop)?;
                            }
                        },
                        Err(_) => break,
                    }

                    let current_state = supplier()?;

                    if hold_state || response.is_some() {
                        if let Some(target) = &target {
                            consumer(MechanismState::Value(
                                controller
                                    .calculate(&current_state, &target)
                                    .map_err(|err| MechanismError::Time(err))?,
                            ))?;
                        }
                    }

                    if target
                        .as_ref()
                        .map(|target| at_setpoint(&current_state, target))
                        .unwrap_or(false)
                    {
                        if let Some(response) = response.take() {
                            if response.send(MechanismResult::Completed).is_err() {
                                warn!("Mechanism response channel has been closed");
                            }
                        }

                        if !hold_state {
                            consumer(MechanismState::Stop)?;
                        }
                    }
                };

                if let Err(err) = result {
                    error!("A mechanism has encountered an error: {:?}", err);
                    if errors_sender.send(err).is_err() {
                        break;
                    }
                }

                yield_now().await;
            }
        })
        .detach();

        Self {
            sender,
            errors: errors_receiver,
        }
    }

    pub async fn set(&self, state: I, priority: bool) -> MechanismResult {
        let (response_sender, response_receiver) = oneshot::channel();

        self.sender
            .send(MechanismRequest::Value(ValueMechanismRequest {
                state,
                priority,
                response: response_sender,
            }))
            .expect("Mechanism task has crashed");

        response_receiver.await.expect("Mechanism task has crashed")
    }

    pub async fn errors(&self) -> &Receiver<MechanismError<E>> {
        &self.errors
    }
}

impl<I: 'static, E: Debug + 'static> ControlSafe for Mechanism<I, E> {
    fn stop(&mut self) {
        self.sender
            .send(MechanismRequest::Stop)
            .expect("Mechanism task has crashed");
    }
}

pub trait MechanismMotor: MotorController {
    fn set_mechanism_state(&mut self, state: MechanismState<f32>) -> Result<(), Self::Error> {
        match state {
            MechanismState::Value(voltage) => self.set_voltage(voltage),
            MechanismState::Stop => {
                self.stop();
                Ok(())
            }
        }
    }
}

impl<M: MotorController> MechanismMotor for M {}
