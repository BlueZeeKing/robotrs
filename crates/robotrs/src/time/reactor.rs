use std::{collections::BinaryHeap, task::Waker, time::Duration};

use linkme::distributed_slice;
use parking_lot::Mutex;

use crate::PERIODIC_CHECKS;

use super::get_time;

/// The Ord implementation is reversed, which is needed for the heap
struct TimeItem {
    time: Duration,
    waker: Waker,
}

impl PartialEq for TimeItem {
    fn eq(&self, other: &Self) -> bool {
        other.time.eq(&self.time)
    }
}

impl Eq for TimeItem {}

impl PartialOrd for TimeItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

static QUEUE: Mutex<BinaryHeap<TimeItem>> = Mutex::new(BinaryHeap::new());

pub fn add_time(time: Duration, waker: Waker) {
    QUEUE.lock().push(TimeItem { time, waker });
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let mut queue = QUEUE.lock();

    let time = get_time();

    while let Some(item) = queue.peek() {
        if item.time > time {
            break;
        }

        queue.pop().unwrap_or_else(|| unreachable!()).waker.wake();
    }
}
