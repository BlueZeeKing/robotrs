use super::{Command, Predicate};

pub trait CommandExt: Command + Sized {
    fn until<F: Predicate>(self, is_finished: F) -> UntilCommand<Self, F> {
        UntilCommand {
            is_finished,
            command: self,
        }
    }
}

pub struct UntilCommand<C: Command, F: Predicate> {
    is_finished: F,
    command: C,
}

impl<C: Command, F: Predicate> Command for UntilCommand<C, F> {
    fn start(&mut self) -> anyhow::Result<()> {
        self.command.start()
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        self.command.execute()
    }

    fn end(&mut self) -> anyhow::Result<()> {
        self.command.end()
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(self.command.is_finished()? || self.is_finished.test()?)
    }
}

impl<C: Command> CommandExt for C {}
