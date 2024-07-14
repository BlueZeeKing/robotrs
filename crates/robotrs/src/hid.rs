use std::{cell::Cell, future::Future, rc::Rc};

use async_task::Task;
use event_listener::Event;

use crate::{scheduler::spawn, yield_now};

pub mod all;
pub mod any;
pub mod axis;
pub mod button;
pub mod controller;
pub mod ext;
pub mod joystick;
pub mod pov;
mod reactor;

/// A generic async trigger
pub trait Trigger {
    type Error;
    type Output;

    /// Wait for the rising edge of the trigger. This returns early if an error occurs
    fn wait_for_trigger(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

pub trait ReleaseTrigger: Trigger {
    /// Wait for the falling edge of the trigger. This returns early if an error occurs
    fn wait_for_release(&mut self) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

// TODO: Maybe make this clonable?
// Task can be wrapped in an Rc. The only issue is errors. Should they clone, or should they only
// be returned by one trigger?
pub struct PollFuture<E: 'static> {
    #[allow(dead_code)] // This is kept to keep the task alive
    task: Task<Option<()>>,

    state: Rc<Cell<bool>>,
    error: Rc<Cell<Option<E>>>,

    last_seen_state: bool,

    state_change: Rc<Event>,
}

impl<E: 'static> PollFuture<E> {
    /// This creates a new trigger that polls the given closure every period (default 20ms)
    pub fn new<F: FnMut() -> Result<bool, E> + 'static>(mut func: F) -> Self {
        let state = Rc::new(Cell::new(false));
        let error = Rc::new(Cell::new(None));
        let state_change = Rc::new(Event::new());

        let state2 = state.clone();
        let error2 = error.clone();
        let state_change2 = state_change.clone();

        let task = spawn(async move {
            let mut last_val = false;
            loop {
                match func() {
                    Ok(val) => {
                        state2.set(val);

                        if last_val != val {
                            state_change2.notify(usize::MAX);
                        }

                        last_val = val;
                    }
                    Err(err) => error2.set(Some(err)),
                }

                yield_now().await;
            }
        });

        Self {
            task,
            state,
            error,
            state_change,
            last_seen_state: false,
        }
    }
}

impl<E: 'static> Trigger for PollFuture<E> {
    type Error = E;
    type Output = ();

    async fn wait_for_trigger(&mut self) -> Result<Self::Output, Self::Error> {
        loop {
            if let Some(err) = self.error.take() {
                return Err(err);
            }

            if !self.last_seen_state && self.state.get() {
                self.last_seen_state = true;
                return Ok(());
            }

            self.last_seen_state = self.state.get();

            self.state_change.listen().await;
        }
    }
}

impl<E: 'static> ReleaseTrigger for PollFuture<E> {
    async fn wait_for_release(&mut self) -> Result<Self::Output, Self::Error> {
        loop {
            if let Some(err) = self.error.take() {
                return Err(err);
            }

            if self.last_seen_state && !self.state.get() {
                self.last_seen_state = false;
                return Ok(());
            }

            self.last_seen_state = self.state.get();

            self.state_change.listen().await;
        }
    }
}
