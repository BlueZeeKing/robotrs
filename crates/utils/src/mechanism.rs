use flume::{Receiver, Sender};
use futures::{channel::oneshot, select, FutureExt};
use robotrs::{
    control::ControlSafe, math::Controller, motor::MotorController, scheduler, yield_now,
};
use std::fmt::Debug;
use tracing::{debug, error, warn};

struct MechanismRequest<I> {
    state: I,
    response: oneshot::Sender<()>,
}

#[derive(Debug)]
pub enum MechanismState<O> {
    Value(O),
    Stop,
}

pub struct Mechanism<I: 'static, E: 'static + Debug> {
    sender: Sender<MechanismRequest<I>>,
    errors: Receiver<E>,
    stop: Sender<()>,
    name: &'static str,
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
        name: &'static str,
    ) -> Self {
        let (sender, receiver): (Sender<MechanismRequest<I>>, Receiver<MechanismRequest<I>>) =
            flume::unbounded();

        let (errors_sender, errors_receiver): (Sender<E>, Receiver<E>) = flume::bounded(5);

        let (stop_sender, stop_receiver) = flume::bounded(1);

        scheduler::spawn(async move {
            loop {
                select! { _ = async {
                        let Ok(request) = receiver.recv_async().await else {
                            debug!("Stopping {} because sender is gone", name);
                            return;
                        };

                        let mut response = Some(request.response);

                        loop {
                            let result: Result<(), E> = try {
                                let current_state = supplier()?;

                                consumer(MechanismState::Value(controller.calculate(
                                    &current_state,
                                    &request.state,
                                )))?;

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
                                if matches!(errors_sender.try_send(err), Err(flume::TrySendError::Disconnected(_))) {
                                    debug!("Stopping {} because error sender is gone", name);
                                    break;
                                }
                            }

                            yield_now().await;
                        }
                    }.fuse() => {
                        debug!("Stopping {} because main loop stopped (error should be above)", name);
                        break;
                    },
                    _ = stop_receiver.recv_async() => {
                        if let Err(err) = consumer(MechanismState::Stop) {
                            error!(
                                "A mechanism has encountered an error while stopping: {:?}",
                                err
                            );
                            if matches!(errors_sender.try_send(err), Err(flume::TrySendError::Disconnected(_))) {
                                debug!("Stopping {} because error sender is gone", name);
                                break;
                            }
                        }
                    }
                };
            }
        })
        .detach();

        Self {
            sender,
            errors: errors_receiver,
            stop: stop_sender,
            name,
        }
    }

    pub async fn set(&mut self, state: I) {
        self.stop();

        let (response_sender, response_receiver) = oneshot::channel();

        self.sender
            .send(MechanismRequest {
                state,
                response: response_sender,
            })
            .unwrap_or_else(|_| panic!("Could not send request in {}", self.name));

        response_receiver
            .await
            .unwrap_or_else(|_| panic!("Failed waiting to hit setpoint in {}", self.name));
    }

    pub async fn errors(&self) -> &Receiver<E> {
        &self.errors
    }
}

impl<I: 'static, E: Debug + 'static> ControlSafe for Mechanism<I, E> {
    fn stop(&mut self) {
        self.stop
            .try_send(())
            .unwrap_or_else(|_| panic!("Failed stopping {}", self.name));
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
