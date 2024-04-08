use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
};

use anyhow::bail;
use futures::{task::waker_ref, Future};

use crate::{queue_waker, waker::SingleWaker};

pub mod ext;

/// A composable action
pub trait Command {
    /// Runs once at the beginning
    fn start(&mut self) -> anyhow::Result<()>;
    /// Runs for every tick of the scheduler
    fn execute(&mut self) -> anyhow::Result<()>;
    /// Runs when the command ends
    fn end(&mut self) -> anyhow::Result<()>;
    /// Runs for every tick and returns true if the command is complete
    fn is_finished(&mut self) -> anyhow::Result<bool>;
}

/// Convert a type to a future, used for commands
pub trait ToFuture {
    type Fut: Future;

    fn to_future(self) -> Self::Fut;
}

impl<C: Command + Unpin> ToFuture for C {
    type Fut = CommandFuture<C>;

    fn to_future(self) -> CommandFuture<C> {
        CommandFuture {
            command: self,
            started: false,
            stopped: false,
        }
    }
}

/// A future that runs a command
pub struct CommandFuture<C: Command + Unpin> {
    command: C,
    started: bool,
    stopped: bool,
}

impl<C: Command + Unpin> CommandFuture<C> {
    fn failable_poll(&mut self, waker: Waker) -> anyhow::Result<Poll<()>> {
        if !self.started {
            self.command.start()?;
        }

        self.command.execute()?;

        if self.command.is_finished()? {
            self.command.end()?;
            self.stopped = true;

            Ok(Poll::Ready(()))
        } else {
            queue_waker(waker);

            Ok(Poll::Pending)
        }
    }
}

impl<C: Command + Unpin> Future for CommandFuture<C> {
    type Output = anyhow::Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let command = Pin::into_inner(self);

        match command.failable_poll(cx.waker().clone()) {
            Ok(val) => {
                if val == Poll::Pending {
                    Poll::Pending
                } else {
                    Poll::Ready(Ok(()))
                }
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl<C: Command + Unpin> Drop for CommandFuture<C> {
    fn drop(&mut self) {
        if self.started && !self.stopped {
            self.command.end().unwrap();
        }
    }
}

/// A command that runs a future
pub struct FutureCommand<F: Future<Output = anyhow::Result<()>>> {
    future: Pin<Box<F>>,
    should_wake: Arc<SingleWaker>,
    done: bool,
}

impl<F: Future<Output = anyhow::Result<()>>> Command for FutureCommand<F> {
    fn start(&mut self) -> anyhow::Result<()> {
        if self.done {
            bail!("Started an already complete future command");
        }

        Ok(())
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if self.done {
            bail!("Ran while done");
        }

        if !self.should_wake.is_woken() {
            return Ok(());
        }

        let res = Pin::as_mut(&mut self.future)
            .poll(&mut Context::from_waker(&waker_ref(&self.should_wake)));

        match res {
            Poll::Ready(Err(err)) => Err(err),
            Poll::Ready(Ok(())) => {
                self.done = true;
                Ok(())
            }
            Poll::Pending => Ok(()),
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(self.done)
    }
}

/// A trait to convert into a command.
/// Used for converting futures to commands
pub trait ToCommand {
    type Command: Command;

    fn to_command(self) -> Self::Command;
}

impl<F: Future<Output = anyhow::Result<()>>> ToCommand for F {
    type Command = FutureCommand<F>;

    fn to_command(self) -> Self::Command {
        FutureCommand {
            future: Box::pin(self),
            should_wake: Default::default(),
            done: false,
        }
    }
}
