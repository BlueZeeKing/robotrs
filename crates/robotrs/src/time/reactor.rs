use std::{collections::BinaryHeap, sync::LazyLock, task::Waker, time::Duration};

use linkme::distributed_slice;
use parking_lot::Mutex;
use tracing::{span, trace, Level, Span};

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

static POLL_SPAN: LazyLock<Span> = LazyLock::new(|| span!(Level::TRACE, "time poll"));

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let _span_guard = POLL_SPAN.enter();
    let mut queue = QUEUE.lock();

    let time = get_time();

    while let Some(item) = queue.peek() {
        if item.time > time {
            break;
        }

        trace!("Waking item from time reactor");

        queue.pop().unwrap_or_else(|| unreachable!()).waker.wake();
    }
}
