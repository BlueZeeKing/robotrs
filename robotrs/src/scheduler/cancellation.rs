use std::{
    cell::{Cell, OnceCell, RefCell},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

use futures::Future;
use pin_project::pin_project;

thread_local! {
    static CURRENT_TASK: OnceCell<RefCell<Option<CancellationHandle>>> = const { OnceCell::new() };
}

/// Start the runtime on the current thread
///
/// # Panics
///
/// Panics if the runtime is already initialized on the current thread
pub fn init_runtime() {
    CURRENT_TASK.with(|current_task| {
        if current_task.set(RefCell::new(None)).is_err() {
            panic!("Runtime already intialized");
        }
    });
}

/// Set the current cancellation scope, returning a guard that will reset it
fn set_task(handle: CancellationHandle) -> CurrentTaskGuard {
    CURRENT_TASK.with(|current_task| {
        let current_task = current_task.get().unwrap();

        let guard = CurrentTaskGuard {
            last_task: current_task.take(),
        };

        *current_task.borrow_mut() = Some(handle);

        guard
    })
}

/// Resets the current cancellation scope
struct CurrentTaskGuard {
    last_task: Option<CancellationHandle>,
}

impl Drop for CurrentTaskGuard {
    fn drop(&mut self) {
        CURRENT_TASK.with(|current_task| {
            *current_task
                .get()
                .expect("Runtime not started on this thread")
                .borrow_mut() = self.last_task.take();
        })
    }
}

/// A handle to a cancellation scope. This type uses an Rc, so it can be cloned freely
#[derive(Clone)]
pub struct CancellationHandle {
    state: Rc<(Cell<bool>, Cell<Option<Waker>>)>,
}

impl CancellationHandle {
    fn new() -> Self {
        let state = Rc::new((Cell::new(false), Cell::new(None)));
        Self { state }
    }

    /// Cancel this scope, it will not be restarted
    pub fn cancel(&self) {
        self.state.0.set(true);
        if let Some(waker) = self.state.1.take() {
            waker.wake();
        }
    }

    fn register_waker(&self, waker: &Waker) {
        self.state.1.set(Some(waker.clone()));
    }

    fn is_canceled(&self) -> bool {
        self.state.0.get()
    }

    /// Get the current cancellation scope
    pub fn get_handle() -> Option<Self> {
        CURRENT_TASK.with(|current_task| {
            current_task
                .get()
                .expect("Runtime not initialized")
                .borrow()
                .clone()
        })
    }
}

/// This represents a scope that can be canceled
#[pin_project]
pub struct CancellationFuture<F: Future> {
    #[pin]
    future: F,
    handle: CancellationHandle,
}

impl<F: Future> Future for CancellationFuture<F> {
    type Output = Result<F::Output, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.project();

        let task = set_task(inner.handle.clone());

        let res = if let Poll::Ready(val) = inner.future.poll(cx) {
            Poll::Ready(Ok(val))
        } else if inner.handle.is_canceled() {
            Poll::Ready(Err(()))
        } else {
            inner.handle.register_waker(cx.waker());
            Poll::Pending
        };

        drop(task);

        res
    }
}

/// Create a new cancellation scope with the given future
pub fn guard<F: Future>(fut: F) -> CancellationFuture<F> {
    CancellationFuture {
        future: fut,
        handle: CancellationHandle::new(),
    }
}
