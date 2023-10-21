use super::{Command, Predicate};

pub trait CommandExt: Command + Sized {
    fn until<F: Predicate>(self, is_finished: F) -> impl Command {
        UntilCommand {
            is_finished,
            command: self,
        }
    }

    fn fuse(self) -> impl Command + Fused {
        FusedCommand(self, false)
    }
}

impl<C: Command> CommandExt for C {}

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

pub struct FusedCommand<C: Command>(C, bool);

impl<C: Command> Command for FusedCommand<C> {
    fn start(&mut self) -> anyhow::Result<()> {
        if !self.1 {
            self.0.start()
        } else {
            Ok(())
        }
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if !self.1 {
            self.0.execute()
        } else {
            Ok(())
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        if !self.1 {
            self.1 = true;

            self.0.end()
        } else {
            Ok(())
        }
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        if !self.1 {
            self.0.is_finished()
        } else {
            Ok(true)
        }
    }
}

pub(super) unsafe trait Fused {}

unsafe impl<C: Command> Fused for FusedCommand<C> {}
