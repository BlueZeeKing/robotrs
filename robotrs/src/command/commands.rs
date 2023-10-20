use std::time::Duration;

use crate::{
    time::{delay, Alarm},
    ErrorFutureWrapper,
};

use super::{Command, FutureCommand, Runnable, ToCommand};

pub fn run_once<F: Runnable>(func: F) -> FuncCommand<F, F, F> {
    FuncCommand {
        start: Some(func),
        execute: None,
        end: None,
    }
}

pub fn run<F: Runnable>(func: F) -> FuncCommand<F, F, F> {
    FuncCommand {
        start: None,
        execute: Some(func),
        end: None,
    }
}

pub fn start_end<F1: Runnable, F2: Runnable>(start: F1, end: F2) -> FuncCommand<F1, F1, F2> {
    FuncCommand {
        start: Some(start),
        execute: None,
        end: Some(end),
    }
}

pub fn run_end<F1: Runnable, F2: Runnable>(run: F1, end: F2) -> FuncCommand<F1, F1, F2> {
    FuncCommand {
        start: None,
        execute: Some(run),
        end: Some(end),
    }
}

pub fn wait(amount: Duration) -> FutureCommand<ErrorFutureWrapper<(), crate::error::Error, Alarm>> {
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

pub struct FuncCommand<F1, F2, F3>
where
    F1: Runnable,
    F2: Runnable,
    F3: Runnable,
{
    start: Option<F1>,
    execute: Option<F2>,
    end: Option<F3>,
}

impl<F1, F2, F3> Command for FuncCommand<F1, F2, F3>
where
    F1: Runnable,
    F2: Runnable,
    F3: Runnable,
{
    fn start(&mut self) -> anyhow::Result<()> {
        if let Some(func) = &mut self.start {
            func.run()
        } else {
            Ok(())
        }
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if let Some(func) = &mut self.execute {
            func.run()
        } else {
            Ok(())
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        if let Some(func) = &mut self.end {
            func.run()
        } else {
            Ok(())
        }
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(false)
    }
}
