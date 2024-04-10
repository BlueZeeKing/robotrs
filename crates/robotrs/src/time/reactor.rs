use std::{task::Waker, time::Duration};

use linkme::distributed_slice;
use parking_lot::Mutex;

use crate::PERIODIC_CHECKS;

use super::get_time;

static QUEUE: Mutex<Vec<(Waker, Duration)>> = Mutex::new(Vec::new());

pub fn add_time(end_time: Duration, waker: Waker) {
    let mut queue = QUEUE.lock();

    let index = {
        let mut final_index = 0;

        for (index, (_, item_duration)) in queue.iter().enumerate() {
            if item_duration < &end_time {
                final_index = index;
                break;
            }
        }

        final_index
    };

    queue.insert(index, (waker, end_time))
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let mut queue = QUEUE.lock();

    let time = get_time();

    while let Some(item) = queue.last() {
        if item.1 > time {
            break;
        }

        queue.pop().unwrap_or_else(|| unreachable!()).0.wake();
    }
}
