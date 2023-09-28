#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]

use std::{
    cell::RefCell,
    io::Write,
    ops::DerefMut,
    pin::Pin,
    task::{Poll, Waker},
};

use futures::Future;
use hal_sys::HAL_SendConsoleLine;
use linkme::distributed_slice;
use tracing_subscriber::fmt::MakeWriter;

pub mod control;
pub mod ds;
pub mod error;
pub mod hid;
pub mod math;
pub mod motor;
pub mod robot;
pub mod scheduler;
pub mod time;

#[distributed_slice]
pub static PERIODIC_CHECKS: [fn()] = [..];

thread_local! {
    static WAKERS: RefCell<Vec<Waker>> = RefCell::new(Vec::new());
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    WAKERS.with(|wakers| {
        for waker in std::mem::take(wakers.borrow_mut().deref_mut()) {
            waker.wake();
        }
    });
}

pub(crate) fn queue_waker(waker: Waker) {
    WAKERS.with(|wakers| {
        wakers.borrow_mut().push(waker);
    })
}

struct DsTracingWriter {}

impl<'a> MakeWriter<'a> for DsTracingWriter {
    type Writer = DsTracingWriter;

    fn make_writer(&'a self) -> Self::Writer {
        Self {}
    }

    fn make_writer_for(&'a self, _meta: &tracing::Metadata<'_>) -> Self::Writer {
        Self {}
    }
}

impl Write for DsTracingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut data = buf.to_vec();

        data.push(0);

        let error_code = unsafe { HAL_SendConsoleLine(data[..].as_ptr()) };

        if error_code == 0 {
            Ok(buf.len())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                crate::error::Error::HalError(error::HalError(error_code)),
            ))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

pub struct CustomFuture<StartFn, ExecuteFn, EndFn>
where
    StartFn: FnMut() + Unpin,
    ExecuteFn: FnMut() -> bool + Unpin,
    EndFn: FnMut() + Unpin,
{
    start: StartFn,
    execute: ExecuteFn,
    end: EndFn,
    started: bool,
}

/// Creates a future from 3 callbacks. The first callback runs once at the beginning. The second
/// callback runs every tick of the scheduler (default is every 20ms). It returns a boolean
/// representing if the future is finished. The last callback should clean up all used resources.
/// This returns a future that can be awaited in asynchronous tasks.
pub fn create_future<StartFn, ExecuteFn, EndFn>(
    start: StartFn,
    execute: ExecuteFn,
    end: EndFn,
) -> impl Future
where
    StartFn: FnMut() + Unpin,
    ExecuteFn: FnMut() -> bool + Unpin,
    EndFn: FnMut() + Unpin,
{
    CustomFuture {
        start,
        execute,
        end,
        started: false,
    }
}

impl<StartFn, ExecuteFn, EndFn> Future for CustomFuture<StartFn, ExecuteFn, EndFn>
where
    StartFn: FnMut() + Unpin,
    ExecuteFn: FnMut() -> bool + Unpin,
    EndFn: FnMut() + Unpin,
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let values = Pin::into_inner(self);

        if !values.started {
            values.started = true;
            (values.start)();
        }

        if (values.execute)() {
            Poll::Ready(())
        } else {
            queue_waker(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<StartFn, ExecuteFn, EndFn> Drop for CustomFuture<StartFn, ExecuteFn, EndFn>
where
    StartFn: FnMut() + Unpin,
    ExecuteFn: FnMut() -> bool + Unpin,
    EndFn: FnMut() + Unpin,
{
    fn drop(&mut self) {
        if self.started {
            (self.end)();
        }
    }
}

pub fn yield_now() -> Yield {
    Yield::default()
}

pub struct Yield {
    yielded: bool,
}

impl Default for Yield {
    fn default() -> Self {
        Self { yielded: false }
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if Pin::into_inner(self).yielded {
            Poll::Ready(())
        } else {
            queue_waker(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub trait Deadzone {
    fn deadzone(self, value: Self) -> Self;
}

impl Deadzone for f32 {
    fn deadzone(self, value: Self) -> Self {
        if self.abs() < value {
            0.0
        } else {
            self
        }
    }
}
