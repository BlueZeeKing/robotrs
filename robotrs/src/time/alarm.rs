use std::{future::Future, pin::Pin, task::Poll, time::Duration};

use crate::error::Result;

use super::get_time;

pub struct Alarm {
    pub(super) end_time: Option<Duration>,
    pub(super) duration: Duration,
}

impl Alarm {
    pub fn new(duration: Duration) -> Self {
        Self {
            end_time: None,
            duration,
        }
    }

    fn poll(&mut self) -> Result<Option<Duration>> {
        let end_time = if let Some(end_time) = self.end_time {
            end_time
        } else {
            let time = get_time()? + self.duration;
            self.end_time = Some(time);

            time
        };

        if get_time()? > end_time {
            Ok(None)
        } else {
            Ok(Some(end_time))
        }
    }
}

impl Future for Alarm {
    type Output = Result<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match Pin::into_inner(self).poll() {
            Ok(time) => {
                if let Some(end_time) = time {
                    super::reactor::add_time(end_time, cx.waker().clone());
                    Poll::Pending
                } else {
                    Poll::Ready(Ok(()))
                }
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}
