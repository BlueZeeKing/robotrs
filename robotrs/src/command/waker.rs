use futures::task::ArcWake;
use std::sync::atomic::AtomicBool;

pub struct SingleWaker(AtomicBool);

impl ArcWake for SingleWaker {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        arc_self.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

impl SingleWaker {
    pub fn is_woken(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for SingleWaker {
    fn default() -> Self {
        Self(AtomicBool::new(true))
    }
}
