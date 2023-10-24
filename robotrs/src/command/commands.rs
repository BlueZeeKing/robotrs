use std::time::Duration;

use crate::{time::delay, ErrorFutureWrapper};

use super::{Command, Func, ToCommand};

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

/// Run somthing at the start and end. This function is very hard to use and is not recomended
#[deprecated]
pub fn start_end<F1: Func<()>, F2: Func<()>>(start: F1, end: F2) -> impl Command {
    FuncCommand {
        start,
        execute: (),
        end,
        is_finished: false,
    }
}

/// Run somthing continously and at the end. This function is very hard to use and is not recomended
#[deprecated]
pub fn run_end<F1: Func<()>, F2: Func<()>>(execute: F1, end: F2) -> impl Command {
    FuncCommand {
        start: (),
        execute,
        end,
        is_finished: false,
    }
}

/// Create a command from a set of callbacks. This function is very hard to use and is not recomended
#[deprecated]
pub fn create_command<Start, Execute, End, Finished>(
    // FIXME: This is actually a massive pain in the ass
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
) -> impl Command
where
    Start: Func<()>,
    Execute: Func<()>,
    End: Func<()>,
    Finished: Func<bool>,
{
    FuncCommand {
        start,
        execute,
        end,
        is_finished,
    }
}

/// Create a command that waits a certain amount of time
pub fn wait(amount: Duration) -> impl Command {
    ErrorFutureWrapper(delay(amount)).to_command()
}

/// A command that does nothing and runs indefinetly
pub fn noop() -> impl Command {
    FuncCommand {
        start: (),
        execute: (),
        end: (),
        is_finished: false,
    }
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
