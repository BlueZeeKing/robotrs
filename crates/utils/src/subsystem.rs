use std::{
    cell::{Cell, RefCell, RefMut},
    cmp::Ordering,
    collections::BinaryHeap,
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::{Rc, Weak},
    task::{Context, Poll},
};

use futures::{task::AtomicWaker, Future};
use robotrs::{
    control::ControlSafe,
    ds::{self, get_state},
    scheduler::{spawn, CancellationHandle},
};
use tracing::warn;

/// A subsystem that allows for priority-based locking.
pub struct Subsystem<T: ControlSafe> {
    value: RefCell<T>,
    tasks: RefCell<BinaryHeap<LockRequest>>,
    current_priority: Cell<u32>,
    current_cancellation: Rc<RefCell<Option<CancellationHandle>>>,
}

struct LockRequest {
    priority: u32,
    waker: Weak<AtomicWaker>,
}

impl Ord for LockRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for LockRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Eq for LockRequest {}

impl PartialEq for LockRequest {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T: ControlSafe> Subsystem<T> {
    /// Create a new subsystem with the given value.
    pub fn new(value: T) -> Self {
        let current_cancellation: Rc<RefCell<Option<CancellationHandle>>> =
            Rc::new(RefCell::new(None));
        let current_cancellation2 = current_cancellation.clone();

        spawn(async move {
            loop {
                ds::wait_for_state_change().await;
                if get_state().disabled() {
                    if let Some(handle) = current_cancellation2.borrow().as_ref() {
                        handle.cancel();
                    }
                }
            }
        })
        .detach();

        Self {
            value: RefCell::new(value),
            tasks: RefCell::new(BinaryHeap::new()),
            current_priority: Cell::new(0),
            current_cancellation,
        }
    }

    /// Lock the subsystem with the given priority. This will cancel the scope of any locks that have a lower priority.
    pub fn lock(&self, priority: u32) -> LockFuture<'_, T> {
        let waker = Rc::new(AtomicWaker::new());

        self.tasks.borrow_mut().push(LockRequest {
            priority,
            waker: Rc::downgrade(&waker),
        });

        LockFuture {
            lock: self,
            waker,
            priority,
        }
    }
}

/// A future that resolves when the subsystem is locked.
pub struct LockFuture<'a, T: ControlSafe> {
    lock: &'a Subsystem<T>,
    waker: Rc<AtomicWaker>,
    priority: u32,
}

fn peek<'a>(val: &mut RefMut<'a, BinaryHeap<LockRequest>>) -> Option<Rc<AtomicWaker>> {
    while let Some(next) = val.peek() {
        if let Some(waker) = next.waker.upgrade() {
            return Some(waker);
        } else {
            val.pop().unwrap();
        }
    }

    None
}

impl<'a, T: ControlSafe> Future for LockFuture<'a, T> {
    type Output = LockGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::into_inner(self);

        let mut tasks = inner.lock.tasks.borrow_mut();

        if get_state().disabled() {
            ds::register_waker(cx.waker().clone());
            return Poll::Pending;
        }

        if Rc::ptr_eq(
            &peek(&mut tasks).expect("No registered lock request"),
            &inner.waker,
        ) {
            if let Ok(guard) = inner.lock.value.try_borrow_mut() {
                let task = tasks
                    .pop()
                    .expect("No registered lock request, this is impossible");

                let handle = CancellationHandle::get_handle();

                if handle.is_none() {
                    warn!("No cancellation handle available, this will prevent this lock from being preempted by a higher priority task.")
                }

                inner.lock.current_priority.set(task.priority);
                *inner.lock.current_cancellation.borrow_mut() = handle;

                Poll::Ready(LockGuard {
                    lock: inner.lock,
                    guard,
                })
            } else {
                if inner.lock.current_priority.get() <= inner.priority {
                    if let Some(handle) = inner.lock.current_cancellation.borrow().as_ref() {
                        handle.cancel();
                    }
                }
                inner.waker.register(cx.waker());
                Poll::Pending
            }
        } else {
            inner.waker.register(cx.waker());
            Poll::Pending
        }
    }
}

/// A guard that unlocks the subsystem when dropped and allows mutable access to the subsystem.
pub struct LockGuard<'a, T: ControlSafe> {
    lock: &'a Subsystem<T>,
    guard: RefMut<'a, T>,
}

impl<'a, T: ControlSafe> Deref for LockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<'a, T: ControlSafe> DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<'a, T: ControlSafe> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.stop();
        if let Some(val) = peek(&mut self.lock.tasks.borrow_mut()) {
            val.wake();
        }
    }
}
