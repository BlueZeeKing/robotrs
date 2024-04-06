#![allow(async_fn_in_trait, incomplete_features)]
#![feature(adt_const_params, const_float_bits_conv)]

use std::{
    ffi::c_char,
    io::Write,
    ops::DerefMut,
    pin::Pin,
    task::{Poll, Waker},
};

use futures::Future;
use hal_sys::HAL_SendConsoleLine;
use linkme::distributed_slice;
use parking_lot::Mutex;
use pin_project::pin_project;
use tracing_subscriber::fmt::MakeWriter;

pub mod command;
pub mod control;
pub mod ds;
pub mod error;
pub mod hid;
pub mod math;
pub mod motor;
pub mod robot;
pub mod scheduler;
pub mod time;
pub(crate) mod waker;

#[distributed_slice]
pub static PERIODIC_CHECKS: [fn()] = [..];

static WAKERS: Mutex<Vec<Waker>> = Mutex::new(Vec::new());

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let wakers = std::mem::take(WAKERS.lock().deref_mut());
    for waker in wakers {
        waker.wake();
    }
}

pub(crate) fn queue_waker(waker: Waker) {
    WAKERS.lock().push(waker);
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

        let error_code = unsafe { HAL_SendConsoleLine(data[..].as_ptr() as *const c_char) };

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

pub fn yield_now() -> Yield {
    Yield::default()
}

pub struct Yield {
    yielded: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Yield {
    fn default() -> Self {
        Self { yielded: false }
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::into_inner(self);
        if inner.yielded {
            Poll::Ready(())
        } else {
            queue_waker(cx.waker().clone());
            inner.yielded = true;
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

pub trait FailableDefault: Sized {
    fn failable_default() -> anyhow::Result<Self>;
}

impl<D: Default> FailableDefault for D {
    fn failable_default() -> anyhow::Result<Self> {
        Ok(Default::default())
    }
}

#[pin_project]
pub struct ErrorFutureWrapper<O, E: Into<anyhow::Error>, F: Future<Output = Result<O, E>>>(
    #[pin] F,
);

impl<O, E: Into<anyhow::Error>, F: Future<Output = Result<O, E>>> Future
    for ErrorFutureWrapper<O, E, F>
{
    type Output = anyhow::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.project().0.poll(cx) {
            Poll::Ready(Ok(_)) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err.into())),
            Poll::Pending => Poll::Pending,
        }
    }
}
