use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
};

use anyhow::bail;
use futures::{task::waker_ref, Future};

use crate::queue_waker;

use self::waker::SingleWaker;

pub mod commands;
pub mod ext;
pub mod group;
mod waker;

pub trait Command {
    fn start(&mut self) -> anyhow::Result<()>;
    fn execute(&mut self) -> anyhow::Result<()>;
    fn end(&mut self) -> anyhow::Result<()>;
    fn is_finished(&mut self) -> anyhow::Result<bool>;
}

pub trait ToFuture {
    type Fut: Future;

    fn into_future(self) -> Self::Fut;
}

impl<C: Command + Unpin> ToFuture for C {
    type Fut = CommandFuture<C>;

    fn into_future(self) -> CommandFuture<C> {
        CommandFuture {
            command: self,
            started: false,
            stopped: false,
        }
    }
}

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

pub trait Predicate {
    fn test(&mut self) -> anyhow::Result<bool>;
}

impl Predicate for bool {
    fn test(&mut self) -> anyhow::Result<bool> {
        Ok(*self)
    }
}

pub trait Runnable {
    fn run(&mut self) -> anyhow::Result<()>;
}

impl Runnable for () {
    fn run(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl<E: IntoErr<bool>, F: FnMut() -> E> Predicate for F {
    fn test(&mut self) -> anyhow::Result<bool> {
        self().into_err()
    }
}

impl<E: IntoErr<()>, F: FnMut() -> E> Runnable for F {
    fn run(&mut self) -> anyhow::Result<()> {
        self().into_err()
    }
}

pub trait IntoErr<V> {
    fn into_err(self) -> anyhow::Result<V>;
}

impl<V> IntoErr<V> for V {
    fn into_err(self) -> anyhow::Result<V> {
        Ok(self)
    }
}

impl<V> IntoErr<V> for anyhow::Result<V> {
    fn into_err(self) -> anyhow::Result<V> {
        self
    }
}
