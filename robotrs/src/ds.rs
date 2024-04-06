use std::task::{Poll, Waker};

use futures::{future::poll_fn, Stream};
use hal_sys::{HAL_ControlWord, HAL_GetControlWord, HAL_RefreshDSData};
use parking_lot::Mutex;

use crate::{
    error::{HalError, Result},
    PERIODIC_CHECKS,
};

static CURRENT_STATE: Mutex<State> = Mutex::new(State::Disabled);

static WAKERS: Mutex<Vec<Waker>> = Mutex::new(Vec::new());

pub fn get_state() -> State {
    *CURRENT_STATE.lock()
}

pub fn register_waker(waker: Waker) {
    WAKERS.lock().push(waker);
}

pub async fn wait_for_state_change() -> State {
    let state = get_state();
    poll_fn(move |cx| {
        let current_state = get_state();
        if current_state != state {
            Poll::Ready(current_state)
        } else {
            register_waker(cx.waker().clone());
            Poll::Pending
        }
    })
    .await
}

pub fn state_stream() -> impl Stream<Item = State> {
    let mut last_state = get_state();

    futures::stream::poll_fn(move |cx| {
        let current_state = get_state();
        if current_state != last_state {
            last_state = current_state;
            Poll::Ready(Some(current_state))
        } else {
            register_waker(cx.waker().clone());
            Poll::Pending
        }
    })
}

#[linkme::distributed_slice(PERIODIC_CHECKS)]
fn check_state() {
    let word = get_control_word().unwrap();
    let state = State::from_control_word(&word);
    *CURRENT_STATE.lock() = state;

    for waker in std::mem::take(&mut *WAKERS.lock()) {
        waker.wake();
    }
}

pub fn get_control_word() -> Result<HAL_ControlWord> {
    unsafe {
        HAL_RefreshDSData();
    }

    let mut word = HAL_ControlWord {
        _bitfield_align_1: [0; 0],
        _bitfield_1: HAL_ControlWord::new_bitfield_1(0, 0, 0, 0, 0, 0, 0),
    };

    let status = unsafe { HAL_GetControlWord(&mut word) };

    if status != 0 {
        Err(HalError(status).into())
    } else {
        Ok(word)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Teleop,
    Auto,
    Test,
    Disabled,
}

impl State {
    pub fn from_control_word(word: &HAL_ControlWord) -> Self {
        if word.enabled() == 0 {
            Self::Disabled
        } else if word.autonomous() == 1 {
            Self::Auto
        } else if word.test() == 1 {
            Self::Test
        } else {
            Self::Teleop
        }
    }

    pub fn disabled(&self) -> bool {
        *self == Self::Disabled
    }
}
