use std::{
    cell::{Cell, RefCell, RefMut},
    cmp::Ordering,
    collections::BinaryHeap,
    ops::{Deref, DerefMut},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use futures::{join, task::AtomicWaker, Future};
use pin_project::pin_project;
use robotrs::control::ControlSafe;

pub struct Subsystem<T: ControlSafe> {
    value: RefCell<T>,
    tasks: RefCell<BinaryHeap<LockRequest>>,
    current_priority: Cell<u32>,
    current_task: AtomicWaker,
}

struct LockRequest {
    priority: u32,
    waker: Rc<AtomicWaker>,
}

impl Ord for LockRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for LockRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Eq for LockRequest {}

impl PartialEq for LockRequest {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl<T: ControlSafe> Subsystem<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
            tasks: RefCell::new(BinaryHeap::new()),
            current_priority: Cell::new(0),
            current_task: AtomicWaker::new(),
        }
    }

    pub fn lock(&self, priority: u32) -> LockFuture<'_, T> {
        let waker = Rc::new(AtomicWaker::new());

        self.tasks.borrow_mut().push(LockRequest {
            priority,
            waker: waker.clone(),
        });

        LockFuture {
            lock: self,
            waker,
            priority,
        }
    }

    pub async fn run<'a, Func, Fut>(&'a self, func: Func, priority: u32) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(ControlSafeGuard<'a, T>) -> Fut,
        Fut: Future + 'a,
    {
        let mut lock = self.lock(priority).await;

        let requested = LockRequested {
            lock: self,
            priority,
        };

        let res = requested.run_with(func(lock.guard.take().unwrap())).await;

        res
    }
}

pub struct LockFuture<'a, T: ControlSafe> {
    lock: &'a Subsystem<T>,
    waker: Rc<AtomicWaker>,
    priority: u32,
}

impl<'a, T: ControlSafe> Future for LockFuture<'a, T> {
    type Output = LockGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::into_inner(self);

        let mut tasks = inner.lock.tasks.borrow_mut();

        if Rc::ptr_eq(
            &tasks.peek().expect("No registered lock request").waker,
            &inner.waker,
        ) {
            if let Ok(guard) = inner.lock.value.try_borrow_mut() {
                let task = tasks
                    .pop()
                    .expect("No registered lock request, this is impossible");

                inner.lock.current_priority.set(task.priority);

                Poll::Ready(LockGuard {
                    lock: inner.lock,
                    priority: task.priority,
                    guard: Some(ControlSafeGuard { guard }),
                })
            } else {
                if inner.lock.current_priority.get() > inner.priority {
                    inner.lock.current_task.wake();
                }
                Poll::Pending
            }
        } else {
            inner.waker.register(cx.waker());
            Poll::Pending
        }
    }
}

pub struct ControlSafeGuard<'a, T: ControlSafe> {
    guard: RefMut<'a, T>,
}

impl<'a, T: ControlSafe> Deref for ControlSafeGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<'a, T: ControlSafe> DerefMut for ControlSafeGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<'a, T: ControlSafe> Drop for ControlSafeGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.stop();
    }
}

pub struct LockGuard<'a, T: ControlSafe> {
    lock: &'a Subsystem<T>,
    guard: Option<ControlSafeGuard<'a, T>>,
    priority: u32,
}

impl<'a, T: ControlSafe> LockGuard<'a, T> {
    pub async fn run<Func, Fut>(mut self, func: Func) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(ControlSafeGuard<'a, T>) -> Fut,
        Fut: Future + 'a,
    {
        let requested = LockRequested {
            lock: self.lock,
            priority: self.priority,
        };

        let res = requested.run_with(func(self.guard.take().unwrap())).await;

        res
    }
}

impl<'a, T: ControlSafe> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(val) = self.lock.tasks.borrow().peek() {
            val.waker.wake();
        }
    }
}

pub struct LockRequested<'a, T: ControlSafe> {
    lock: &'a Subsystem<T>,
    priority: u32,
}

impl<'a, T: ControlSafe> Future for LockRequested<'a, T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::into_inner(self);

        let tasks = inner.lock.tasks.borrow();

        if tasks
            .peek()
            .map(|item| item.priority > inner.priority)
            .unwrap_or(false)
        {
            Poll::Ready(())
        } else {
            inner.lock.current_task.register(cx.waker());

            Poll::Pending
        }
    }
}

impl<'a, T: ControlSafe> LockRequested<'a, T> {
    pub fn run_with<F: Future + 'a>(self, fut: F) -> SubsystemTask<'a, T, F> {
        SubsystemTask {
            fut,
            lock_requested: self,
        }
    }
}

#[pin_project]
pub struct SubsystemTask<'a, T: ControlSafe, F: Future + 'a> {
    #[pin]
    fut: F,
    #[pin]
    lock_requested: LockRequested<'a, T>,
}

impl<'a, T: ControlSafe, F: Future + 'a> Future for SubsystemTask<'a, T, F> {
    type Output = Result<F::Output, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projection = self.project();

        if let Poll::Ready(val) = projection.fut.poll(cx) {
            Poll::Ready(Ok(val))
        } else if let Poll::Ready(()) = projection.lock_requested.poll(cx) {
            Poll::Ready(Err(()))
        } else {
            Poll::Pending
        }
    }
}

pub trait SubsystemGroup {
    type Output<'a>;

    fn run<'a, Func, Fut>(
        &'a self,
        func: Func,
        priority: u32,
    ) -> impl Future<Output = Result<Fut::Output, ()>>
    where
        Func: FnOnce(Self::Output<'a>) -> Fut + 'a,
        Fut: Future + 'a;
}

impl<T1: 'static + ControlSafe> SubsystemGroup for &Subsystem<T1> {
    type Output<'a> = ControlSafeGuard<'a, T1>;

    async fn run<'a, Func, Fut>(&'a self, func: Func, priority: u32) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(Self::Output<'a>) -> Fut + 'a,
        Fut: Future + 'a,
    {
        self.run(func, priority).await
    }
}

impl<T1: 'static + ControlSafe, T2: 'static + ControlSafe> SubsystemGroup
    for (&Subsystem<T1>, &Subsystem<T2>)
{
    type Output<'a> = (ControlSafeGuard<'a, T1>, ControlSafeGuard<'a, T2>);

    async fn run<'a, Func, Fut>(&'a self, func: Func, priority: u32) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(Self::Output<'a>) -> Fut + 'a,
        Fut: Future + 'a,
    {
        let (first, second) = join!(self.0.lock(priority), self.1.lock(priority));

        let res = first
            .run(|first| async move { second.run(|second| func((first, second))).await })
            .await;

        match res {
            Ok(Ok(val)) => Ok(val),
            _ => Err(()),
        }
    }
}

impl<T1: 'static + ControlSafe, T2: 'static + ControlSafe, T3: 'static + ControlSafe> SubsystemGroup
    for (&Subsystem<T1>, &Subsystem<T2>, &Subsystem<T3>)
{
    type Output<'a> = (
        ControlSafeGuard<'a, T1>,
        ControlSafeGuard<'a, T2>,
        ControlSafeGuard<'a, T3>,
    );

    async fn run<'a, Func, Fut>(&'a self, func: Func, priority: u32) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(Self::Output<'a>) -> Fut + 'a,
        Fut: Future + 'a,
    {
        let (first, second, third) = join!(
            self.0.lock(priority),
            self.1.lock(priority),
            self.2.lock(priority)
        );

        let res = first
            .run(|first| async move {
                second.run(
                    |second| async move { third.run(|third| func((first, second, third))).await },
                ).await
            })
            .await;

        match res {
            Ok(Ok(Ok(val))) => Ok(val),
            _ => Err(()),
        }
    }
}

impl<
        T1: 'static + ControlSafe,
        T2: 'static + ControlSafe,
        T3: 'static + ControlSafe,
        T4: 'static + ControlSafe,
    > SubsystemGroup
    for (
        &Subsystem<T1>,
        &Subsystem<T2>,
        &Subsystem<T3>,
        &Subsystem<T4>,
    )
{
    type Output<'a> = (
        ControlSafeGuard<'a, T1>,
        ControlSafeGuard<'a, T2>,
        ControlSafeGuard<'a, T3>,
        ControlSafeGuard<'a, T4>,
    );

    async fn run<'a, Func, Fut>(&'a self, func: Func, priority: u32) -> Result<Fut::Output, ()>
    where
        Func: FnOnce(Self::Output<'a>) -> Fut + 'a,
        Fut: Future + 'a,
    {
        let (first, second, third, fourth) = join!(
            self.0.lock(priority),
            self.1.lock(priority),
            self.2.lock(priority),
            self.3.lock(priority)
        );

        let res = first
            .run(|first| async move {
                second
                    .run(|second| async move {
                        third
                            .run(|third| async move {
                                fourth
                                    .run(|fourth| func((first, second, third, fourth)))
                                    .await
                            })
                            .await
                    })
                    .await
            })
            .await;

        match res {
            Ok(Ok(Ok(Ok(val)))) => Ok(val),
            _ => Err(()),
        }
    }
}
