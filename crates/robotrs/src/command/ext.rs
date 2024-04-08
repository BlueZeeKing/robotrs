use anyhow::anyhow;

use super::Command;

pub trait CommandExt: Command + Sized {
    /// Runs a command until a predicate is true
    fn until<F: FnMut() -> anyhow::Result<bool>>(self, is_finished: F) -> UntilCommand<Self, F> {
        UntilCommand {
            is_finished,
            command: self,
        }
    }

    /// Prevents a command from being executed after it is complete.
    /// Used internally for group commands
    fn fuse(self) -> FusedCommand<Self> {
        FusedCommand(self, false)
    }

    fn race<C: Command>(self, other: C) -> RaceCommand<Self, C> {
        RaceCommand {
            command1: self,
            command2: other,
        }
    }

    fn parallel<C: Command>(self, other: C) -> ParallelCommand<Self, C> {
        ParallelCommand {
            command1: self,
            command1_done: false,
            command2: other,
            command2_done: false,
        }
    }

    fn chain<C: Command>(self, other: C) -> ChainCommand<Self, C> {
        ChainCommand {
            command1: self,
            command2: other,
            command1_done: false,
        }
    }
}

pub struct RaceCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    command1: C1,
    command2: C2,
}

impl<C1, C2> Command for RaceCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    fn start(&mut self) -> anyhow::Result<()> {
        self.command1.start()?;
        self.command2.start()?;

        Ok(())
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        self.command1.execute()?;
        self.command2.execute()?;

        Ok(())
    }

    fn end(&mut self) -> anyhow::Result<()> {
        self.command1.end()?;
        self.command2.end()?;

        Ok(())
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(self.command1.is_finished()? || self.command2.is_finished()?)
    }
}

pub struct ParallelCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    command1: C1,
    command1_done: bool,
    command2: C2,
    command2_done: bool,
}

impl<C1, C2> Command for ParallelCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    fn start(&mut self) -> anyhow::Result<()> {
        if self.command1_done && self.command2_done {
            Err(anyhow!("Command already ended"))
        } else if self.command1_done {
            self.command2.start()
        } else if self.command2_done {
            self.command1.start()
        } else {
            match (self.command1.start(), self.command2.start()) {
                (Ok(_), Ok(_)) => Ok(()),
                (Ok(_), Err(_)) => {
                    self.command2_done = true;
                    Ok(())
                }
                (Err(_), Ok(_)) => {
                    self.command1_done = true;
                    Ok(())
                }
                (Err(_), Err(err)) => Err(err),
            }
        }
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if self.command1_done && self.command2_done {
            Err(anyhow!("Command already ended"))
        } else if self.command1_done {
            self.command2.execute()
        } else if self.command2_done {
            self.command1.execute()
        } else {
            match (self.command1.execute(), self.command2.execute()) {
                (Ok(_), Ok(_)) => Ok(()),
                (Ok(_), Err(_)) => {
                    self.command2_done = true;
                    Ok(())
                }
                (Err(_), Ok(_)) => {
                    self.command1_done = true;
                    Ok(())
                }
                (Err(_), Err(err)) => Err(err),
            }
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        if self.command1_done && self.command2_done {
            Err(anyhow!("Command already ended"))
        } else if self.command1_done {
            self.command2.end()
        } else if self.command2_done {
            self.command1.end()
        } else {
            match (self.command1.end(), self.command2.end()) {
                (Ok(_), Ok(_)) => Ok(()),
                (Ok(_), Err(_)) => {
                    self.command2_done = true;
                    Ok(())
                }
                (Err(_), Ok(_)) => {
                    self.command1_done = true;
                    Ok(())
                }
                (Err(_), Err(err)) => Err(err),
            }
        }
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        if self.command1_done && self.command2_done {
            Ok(true)
        } else if !self.command1_done && self.command1.is_finished().unwrap_or(true) {
            self.command1_done = true;
            if let Err(err) = self.command1.end() {
                if self.command2_done {
                    Err(err)
                } else {
                    Ok(false)
                }
            } else {
                Ok(self.command2_done)
            }
        } else if !self.command2_done && self.command2.is_finished().unwrap_or(true) {
            self.command2_done = true;
            if let Err(err) = self.command2.end() {
                if self.command1_done {
                    Err(err)
                } else {
                    Ok(false)
                }
            } else {
                Ok(self.command1_done)
            }
        } else {
            Ok(false)
        }
    }
}

pub struct ChainCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    command1: C1,
    command2: C2,
    command1_done: bool,
}

impl<C1, C2> Command for ChainCommand<C1, C2>
where
    C1: Command,
    C2: Command,
{
    fn start(&mut self) -> anyhow::Result<()> {
        self.command1.start()
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        if self.command1_done {
            self.command2.execute()
        } else if self.command1.is_finished()? {
            self.command1.end()?;
            self.command1_done = true;
            self.command2.start()
        } else {
            self.command1.execute()
        }
    }

    fn end(&mut self) -> anyhow::Result<()> {
        if self.command1_done {
            self.command2.end()
        } else {
            self.command1.end()
        }
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        Ok(self.command1.is_finished()? || self.command1.is_finished()?)
    }
}

impl<C: Command> CommandExt for C {}

/// Created through [CommandExt::until]
pub struct UntilCommand<C: Command, F: FnMut() -> anyhow::Result<bool>> {
    is_finished: F,
    command: C,
}

impl<C: Command, F: FnMut() -> anyhow::Result<bool>> Command for UntilCommand<C, F> {
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
        Ok(self.command.is_finished()? || (self.is_finished)()?)
    }
}

/// Created through [CommandExt::fuse]
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
