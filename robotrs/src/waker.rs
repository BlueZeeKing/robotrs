use futures::{
    task::{waker, ArcWake},
    Future,
};
use std::{
    marker::PhantomData,
    pin::Pin,
    sync::{atomic::AtomicBool, Arc},
    task::{Context, Poll},
};

pub struct SingleWaker(AtomicBool);

impl ArcWake for SingleWaker {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        arc_self.0.store(true, std::sync::atomic::Ordering::Release);
    }
}

impl SingleWaker {
    pub fn is_woken(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

impl Default for SingleWaker {
    fn default() -> Self {
        Self(AtomicBool::new(true))
    }
}

pub struct SimpleHandle<'a, O> {
    waker: Arc<SingleWaker>,
    fut: Pin<Box<dyn Future<Output = O> + 'a>>,
    done: bool,
    _phantom: PhantomData<&'a str>,
}

impl<'a, O> SimpleHandle<'a, O> {
    pub fn spawn<F: Future<Output = O> + 'a>(fut: F) -> Self {
        Self {
            waker: Default::default(),
            fut: Box::pin(fut),
            done: false,
            _phantom: PhantomData,
        }
    }

    pub fn poll(&mut self) -> Option<O> {
        if !self.done && self.waker.is_woken() {
            let res = unsafe { Pin::new_unchecked(&mut self.fut) }
                .poll(&mut Context::from_waker(&waker(self.waker.clone())));

            if let Poll::Ready(val) = res {
                self.done = true;
                return Some(val);
            }
        }

        None
    }
}
