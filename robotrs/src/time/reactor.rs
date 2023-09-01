use std::{cell::RefCell, ops::DerefMut, task::Waker, time::Duration};

use linkme::distributed_slice;

use crate::PERIODIC_CHECKS;

use super::get_time;

thread_local! {
    static QUEUE: RefCell<Vec<(Waker, Duration)>> = RefCell::new(Vec::new());
}

pub fn add_time(end_time: Duration, waker: Waker) {
    QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

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
    })
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

        if let Ok(time) = get_time() {
            while let Some(item) = queue.last() {
                if item.1 > time {
                    break;
                }

                queue.pop().unwrap_or_else(|| unreachable!()).0.wake();
            }
        } else {
            let items = std::mem::take(queue.deref_mut());

            for item in items {
                item.0.wake();
            }
        }
    });
}
