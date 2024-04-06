use std::time::Duration;

use anyhow::bail;

use crate::{
    control::{ControlGuard, ControlLock, ControlSafe},
    time::delay,
    ErrorFutureWrapper,
};

use super::{Command, Func, StateFunc, ToCommand, ToFuture};

/// Create a command that runs once at the start
pub fn run_once<F: Func<()>>(func: F) -> impl Command {
    FuncCommand {
        start: func,
        execute: (),
        end: (),
        is_finished: true,
    }
}

/// Run indefinitely
pub fn run<F: Func<()>>(func: F) -> impl Command {
    FuncCommand {
        start: (),
        execute: func,
        end: (),
        is_finished: false,
    }
}

/// Run somthing at the start and end.
pub fn start_end<Start, End, State>(start: Start, end: End) -> impl Command
where
    Start: Func<State>,
    End: StateFunc<(), State>,
{
    StateFuncCommand {
        start,
        execute: (),
        end,
        is_finished: false,
        state: None,
    }
}

/// Run somthing continuously and at the end.
pub fn run_end<Start, Execute, End, State>(start: Start, execute: Execute, end: End) -> impl Command
where
    Start: Func<State>,
    Execute: StateFunc<(), State>,
    End: StateFunc<(), State>,
{
    StateFuncCommand {
        start,
        execute,
        end,
        is_finished: false,
        state: None,
    }
}

/// Create a command from a set of callbacks with state.
pub fn create_command<Start, Execute, End, Finished, State>(
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
) -> impl Command
where
    Start: Func<State>,
    Execute: StateFunc<(), State>,
    End: StateFunc<(), State>,
    Finished: StateFunc<bool, State>,
{
    StateFuncCommand {
        start,
        execute,
        end,
        is_finished,
        state: None,
    }
}

/// Create a command that waits a certain amount of time
pub fn wait(amount: Duration) -> impl Command {
    ErrorFutureWrapper(delay(amount)).to_command()
}

/// A command that does nothing and runs indefinitely
pub fn noop() -> impl Command {
    FuncCommand {
        start: (),
        execute: (),
        end: (),
        is_finished: false,
    }
}

pub fn control_lock_command<
    'a,
    C: Command + Unpin + 'a,
    F: 'a + Fn(ControlGuard<'a, T>) -> C,
    T: ControlSafe,
>(
    func: F,
    lock: &'a ControlLock<T>,
) -> impl Command + 'a {
    async move { func(lock.lock().await).to_future().await }.to_command()
}

pub struct FuncCommand<Start, Execute, End, Finished>
where
    Start: Func<()>,
    Execute: Func<()>,
    End: Func<()>,
    Finished: Func<bool>,
{
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
}

impl<Start, Execute, End, Finished> Command for FuncCommand<Start, Execute, End, Finished>
where
    Start: Func<()>,
    Execute: Func<()>,
    End: Func<()>,
    Finished: Func<bool>,
{
    fn start(&mut self) -> anyhow::Result<()> {
        self.start.run()
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        self.execute.run()
    }

    fn end(&mut self) -> anyhow::Result<()> {
        self.end.run()
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        self.is_finished.run()
    }
}

struct StateFuncCommand<Start, Execute, End, Finished, State>
where
    Start: Func<State>,
    Execute: StateFunc<(), State>,
    End: StateFunc<(), State>,
    Finished: StateFunc<bool, State>,
{
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
    state: Option<State>,
}

impl<Start, Execute, End, Finished, State> Command
    for StateFuncCommand<Start, Execute, End, Finished, State>
where
    Start: Func<State>,
    Execute: StateFunc<(), State>,
    End: StateFunc<(), State>,
    Finished: StateFunc<bool, State>,
{
    fn start(&mut self) -> anyhow::Result<()> {
        self.state = Some(self.start.run()?);

        Ok(())
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if let Some(state) = &mut self.state {
            self.execute.run(state)
        } else {
            bail!("State not initialized yet")
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        if let Some(state) = &mut self.state {
            self.end.run(state)
        } else {
            bail!("State not initialized yet")
        }
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        if let Some(state) = &mut self.state {
            self.is_finished.run(state)
        } else {
            bail!("State not initialized yet")
        }
    }
}
