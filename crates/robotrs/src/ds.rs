use std::{
    mem::MaybeUninit,
    sync::LazyLock,
    task::{Poll, Waker},
};

use futures::{future::poll_fn, Stream};
use hal_sys::{
    HAL_AllianceStationID_HAL_AllianceStationID_kBlue1,
    HAL_AllianceStationID_HAL_AllianceStationID_kBlue2,
    HAL_AllianceStationID_HAL_AllianceStationID_kBlue3,
    HAL_AllianceStationID_HAL_AllianceStationID_kRed1,
    HAL_AllianceStationID_HAL_AllianceStationID_kRed2,
    HAL_AllianceStationID_HAL_AllianceStationID_kRed3, HAL_ControlWord, HAL_GetAllianceStation,
    HAL_GetControlWord, HAL_RefreshDSData,
};
use parking_lot::Mutex;
use tracing::{span, trace, Level, Span};

use crate::{
    error::{HalError, Result},
    status_to_result, PERIODIC_CHECKS,
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

static POLL_SPAN: LazyLock<Span> = LazyLock::new(|| span!(Level::TRACE, "ds state poll"));

#[linkme::distributed_slice(PERIODIC_CHECKS)]
fn check_state() {
    let _span_guard = POLL_SPAN.enter();
    let word = get_control_word().unwrap();
    let state = State::from_control_word(&word);
    let mut current_state = CURRENT_STATE.lock();

    if *current_state != state {
        trace!(old_state = ?current_state, new_state = ?state, "New state");
        *current_state = state;
        drop(current_state);
        for waker in std::mem::take(&mut *WAKERS.lock()) {
            waker.wake();
        }
    } else {
        *current_state = state;
    }
}

pub fn get_control_word() -> Result<HAL_ControlWord> {
    unsafe {
        HAL_RefreshDSData();
    }

    let mut word = MaybeUninit::uninit();

    let status = unsafe { HAL_GetControlWord(word.as_mut_ptr()) };

    if status != 0 {
        Err(HalError(status).into())
    } else {
        Ok(unsafe { word.assume_init() })
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

pub enum Alliance {
    Blue(u8),
    Red(u8),
}

#[allow(non_upper_case_globals)]
pub fn get_alliance() -> Option<Alliance> {
    let station = unsafe { status_to_result!(HAL_GetAllianceStation()) }.ok()?;

    match station {
        HAL_AllianceStationID_HAL_AllianceStationID_kBlue1 => Some(Alliance::Blue(1)),
        HAL_AllianceStationID_HAL_AllianceStationID_kBlue2 => Some(Alliance::Blue(2)),
        HAL_AllianceStationID_HAL_AllianceStationID_kBlue3 => Some(Alliance::Blue(3)),
        HAL_AllianceStationID_HAL_AllianceStationID_kRed1 => Some(Alliance::Red(1)),
        HAL_AllianceStationID_HAL_AllianceStationID_kRed2 => Some(Alliance::Red(2)),
        HAL_AllianceStationID_HAL_AllianceStationID_kRed3 => Some(Alliance::Red(3)),
        _ => None,
    }
}

impl Alliance {
    pub fn is_blue(&self) -> bool {
        matches!(self, Alliance::Blue(_))
    }

    pub fn is_red(&self) -> bool {
        !self.is_blue()
    }
}

pub async fn wait_for_enabled() {
    if get_state() != State::Disabled {
        return;
    }

    loop {
        if wait_for_state_change().await != State::Disabled {
            return;
        }
    }
}

pub async fn wait_for_disabled() {
    if get_state() == State::Disabled {
        return;
    }

    loop {
        if wait_for_state_change().await == State::Disabled {
            return;
        }
    }
}
