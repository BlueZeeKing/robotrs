use impl_trait_for_tuples::impl_for_tuples;

use super::{
    ext::{CommandExt, Fused},
    Command,
};

pub trait RaceGroup {
    fn start_r(&mut self) -> anyhow::Result<()>;
    fn execute_r(&mut self) -> anyhow::Result<()>;
    fn end_r(&mut self) -> anyhow::Result<()>;
    fn is_finished_r(&mut self) -> anyhow::Result<bool>;
}

#[impl_for_tuples(1, 8)]
#[tuple_types_custom_trait_bound(Command)]
impl RaceGroup for Tuple {
    fn start_r(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.start()?; )* );

        Ok(())
    }

    fn execute_r(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.execute()?; )* );

        Ok(())
    }

    fn end_r(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.end()?; )* );

        Ok(())
    }

    fn is_finished_r(&mut self) -> anyhow::Result<bool> {
        for_tuples!( #( if Tuple.is_finished()? { return Ok(true); } )* );

        Ok(false)
    }
}

pub struct RaceWrapper<R: RaceGroup>(R);

impl<R: RaceGroup> Command for RaceWrapper<R> {
    fn start(&mut self) -> anyhow::Result<()> {
        self.0.start_r()
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        self.0.execute_r()
    }

    fn end(&mut self) -> anyhow::Result<()> {
        self.0.end_r()
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        self.0.is_finished_r()
    }
}

pub trait ParallelGroup {
    fn start_p(&mut self) -> anyhow::Result<()>;
    fn execute_p(&mut self) -> anyhow::Result<()>;
    fn end_p(&mut self) -> anyhow::Result<()>;
    fn is_finished_p(&mut self) -> anyhow::Result<bool>;
}

#[impl_for_tuples(1, 8)]
#[tuple_types_custom_trait_bound(Fused + Command)]
impl ParallelGroup for Tuple {
    fn start_p(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.start()?; )* );

        Ok(())
    }

    fn execute_p(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.execute()?; )* );

        Ok(())
    }

    fn end_p(&mut self) -> anyhow::Result<()> {
        for_tuples!( #( Tuple.end()?; )* );

        Ok(())
    }

    fn is_finished_p(&mut self) -> anyhow::Result<bool> {
        for_tuples!( #( if !Tuple.is_finished()? { return Ok(false); } else { Tuple.end()?; } )* );

        Ok(true)
    }
}

pub struct ParallelWrapper<P: ParallelGroup>(P);

impl<P: ParallelGroup> Command for ParallelWrapper<P> {
    fn start(&mut self) -> anyhow::Result<()> {
        self.0.start_p()
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        self.0.execute_p()
    }

    fn end(&mut self) -> anyhow::Result<()> {
        self.0.end_p()
    }

    fn is_finished(&mut self) -> anyhow::Result<bool> {
        self.0.is_finished_p()
    }
}

pub trait GroupExt {
    fn race(self) -> impl Command;
    fn parallel(self) -> impl Command;
}

#[impl_for_tuples(1, 8)]
#[tuple_types_custom_trait_bound(Command + 'static)]
impl GroupExt for Tuple {
    fn race(self) -> impl Command {
        RaceWrapper(self)
    }

    fn parallel(self) -> impl Command {
        ParallelWrapper(for_tuples!( (#( Tuple.fuse() ),*) ))
    }
}
