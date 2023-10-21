use std::time::Duration;

use crate::{
    time::{delay, Alarm},
    ErrorFutureWrapper,
};

use super::{Command, FutureCommand, Predicate, Runnable, ToCommand};

pub fn run_once<F: Runnable>(func: F) -> impl Command {
    FuncCommand {
        start: func,
        execute: (),
        end: (),
        is_finished: true,
    }
}

pub fn run<F: Runnable>(func: F) -> impl Command {
    FuncCommand {
        start: (),
        execute: func,
        end: (),
        is_finished: false,
    }
}

pub fn start_end<F1: Runnable, F2: Runnable>(start: F1, end: F2) -> impl Command {
    FuncCommand {
        start,
        execute: (),
        end,
        is_finished: false,
    }
}

pub fn run_end<F1: Runnable, F2: Runnable>(execute: F1, end: F2) -> impl Command {
    FuncCommand {
        start: (),
        execute,
        end,
        is_finished: false,
    }
}

pub fn create_command<Start, Execute, End, Finished>(
    // FIXME: This is actually a massive pain in the ass
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
) -> impl Command
where
    Start: Runnable,
    Execute: Runnable,
    End: Runnable,
    Finished: Predicate,
{
    FuncCommand {
        start,
        execute,
        end,
        is_finished,
    }
}

pub fn wait(amount: Duration) -> impl Command {
    ErrorFutureWrapper(delay(amount)).to_command()
}

pub fn noop() -> impl Command {
    NoopCommand
}

pub struct NoopCommand;

impl Command for NoopCommand {
    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn end(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(false)
    }
}

pub struct FuncCommand<Start, Execute, End, Finished>
where
    Start: Runnable,
    Execute: Runnable,
    End: Runnable,
    Finished: Predicate,
{
    start: Start,
    execute: Execute,
    end: End,
    is_finished: Finished,
}

impl<Start, Execute, End, Finished> Command for FuncCommand<Start, Execute, End, Finished>
where
    Start: Runnable,
    Execute: Runnable,
    End: Runnable,
    Finished: Predicate,
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
        self.is_finished.test()
    }
}
