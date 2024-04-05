use defer_lite::defer;
use flume::{select, Receiver, Sender};
use futures::{
    channel::oneshot,
    future::{select, Either},
    select, FutureExt,
};
use robotrs::{
    control::ControlSafe, math::Controller, motor::MotorController, scheduler, yield_now,
};
use std::fmt::Debug;
use tracing::{error, trace, warn};

use async_deadman::{Deadman, DeadmanReceiver};

struct MechanismRequest<I> {
    state: I,
    response: oneshot::Sender<()>,
    deadman: DeadmanReceiver,
}

#[derive(Debug)]
pub enum MechanismState<O> {
    Value(O),
    Stop,
}

pub struct Mechanism<I: 'static, E: 'static + Debug> {
    sender: Sender<MechanismRequest<I>>,
    errors: Receiver<MechanismError<E>>,
    stop: Sender<()>,
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
    ) -> Self {
        let (sender, receiver): (Sender<MechanismRequest<I>>, Receiver<MechanismRequest<I>>) =
            flume::unbounded();

        let (errors_sender, errors_receiver): (
            Sender<MechanismError<E>>,
            Receiver<MechanismError<E>>,
        ) = flume::unbounded();

        let (stop_sender, stop_receiver) = flume::bounded(1);

        scheduler::spawn(async move {
            loop {
                let Either::Left((request, _)) =
                    select(receiver.recv_async(), stop_receiver.recv_async()).await
                else {
                    if let Err(err) = consumer(MechanismState::Stop) {
                        error!(
                            "A mechanism has encountered an error while stopping: {:?}",
                            err
                        );
                        if errors_sender.send(MechanismError::User(err)).is_err() {
                            break;
                        }
                    }

                    continue;
                };

                let Ok(request) = request else {
                    break;
                };

                let mut response = Some(request.response);

                select! {
                    _ = async {
                        loop {
                            let result: Result<(), MechanismError<E>> = try {
                                let current_state = supplier()?;

                                consumer(MechanismState::Value(
                                    controller
                                        .calculate(&current_state, &request.state)
                                        .map_err(|err| MechanismError::Time(err))?,
                                ))?;

                                if at_setpoint(&current_state, &request.state) {
                                    if let Some(response) = response.take() {
                                        if response.send(()).is_err() {
                                            warn!("Mechanism response channel has been closed");
                                        }
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
                    }.fuse() => {
                        break;
                    }
                    _ = request.deadman.fuse() => {}
                    _ = stop_receiver.recv_async() => {}
                }

                if let Err(err) = consumer(MechanismState::Stop) {
                    error!(
                        "A mechanism has encountered an error while stopping: {:?}",
                        err
                    );
                    if errors_sender.send(MechanismError::User(err)).is_err() {
                        break;
                    }
                }
            }
        })
        .detach();

        Self {
            sender,
            errors: errors_receiver,
            stop: stop_sender,
        }
    }

    pub async fn set(&mut self, state: I) -> Deadman {
        // TODO: Link deadman and self lifetime
        let (response_sender, response_receiver) = oneshot::channel();
        let (deadman, deadman_receiver) = Deadman::new();

        self.sender
            .send(MechanismRequest {
                state,
                response: response_sender,
                deadman: deadman_receiver,
            })
            .expect("Mechanism task has crashed");

        response_receiver.await.expect("Mechanism task has crashed");

        deadman
    }

    pub async fn errors(&self) -> &Receiver<MechanismError<E>> {
        &self.errors
    }
}

impl<I: 'static, E: Debug + 'static> ControlSafe for Mechanism<I, E> {
    fn stop(&mut self) {
        self.stop
            .try_send(())
            .expect("Mechanism task has crashed or is taking too long to stop");
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
