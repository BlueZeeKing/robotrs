use std::{
    mem::MaybeUninit,
    ops::DerefMut,
    pin::Pin,
    task::{Poll, Waker},
};

use futures::Future;
use hal_sys::{HAL_GetAllJoystickData, HAL_JoystickAxes, HAL_JoystickButtons, HAL_JoystickPOVs};
use linkme::distributed_slice;
use parking_lot::Mutex;
use slab::Slab;

use super::{
    axis::{get_axis, AxisTarget},
    button::{get_button, ButtonTarget},
    joystick::Joystick,
    pov::{get_pov, PovTarget},
};
use crate::PERIODIC_CHECKS;

pub enum Target {
    Button(ButtonTarget),
    Axis(AxisTarget),
    Pov(PovTarget),
}

impl From<ButtonTarget> for Target {
    fn from(value: ButtonTarget) -> Self {
        Self::Button(value)
    }
}

impl From<AxisTarget> for Target {
    fn from(value: AxisTarget) -> Self {
        Self::Axis(value)
    }
}

impl From<PovTarget> for Target {
    fn from(value: PovTarget) -> Self {
        Self::Pov(value)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Triggered,
    Release,
    OutOfRange,
    Unknown,
}

struct JoystickQueueItem {
    joystick: Joystick,
    idx: u32,
    target: Target,
    waker: Option<Waker>,
    state: State,
    triggered: bool,
    released: bool,
}

static QUEUE: Mutex<Slab<JoystickQueueItem>> = Mutex::new(Slab::new());

pub fn add_trigger(joystick: &Joystick, index: u32, target: Target) -> usize {
    QUEUE.lock().insert(JoystickQueueItem {
        joystick: *joystick,
        idx: index,
        target,
        waker: None,
        state: State::Unknown,
        triggered: false,
        released: false,
    })
}

pub fn remove_trigger(idx: usize) {
    QUEUE.lock().remove(idx);
}

pub fn set_target(idx: usize, target: Target) {
    let mut lock = QUEUE.lock();
    let item = lock.get_mut(idx).unwrap();
    item.target = target;
    item.state = State::Unknown;
}

fn register_waker(idx: usize, waker: Waker) {
    QUEUE.lock().get_mut(idx).unwrap().waker = Some(waker);
}

pub async fn wait_for_triggered(idx: usize) -> Result<(), ()> {
    ReactorFuture {
        idx,
        first_run: true,
        on_release: false,
    }
    .await
}

pub async fn wait_for_released(idx: usize) -> Result<(), ()> {
    ReactorFuture {
        idx,
        first_run: true,
        on_release: true,
    }
    .await
}

struct ReactorFuture {
    idx: usize,
    first_run: bool,
    on_release: bool,
}

impl Future for ReactorFuture {
    type Output = Result<(), ()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let inner = Pin::into_inner(self);

        let first_run = inner.first_run;
        inner.first_run = false;

        let mut lock = QUEUE.lock();
        let item = lock.get_mut(inner.idx).unwrap();

        if !item.triggered && item.state == State::Triggered && !inner.on_release {
            item.triggered = true;
            Poll::Ready(Ok(()))
        } else if !item.released && item.state == State::Release && inner.on_release {
            item.released = true;
            Poll::Ready(Ok(()))
        } else if item.state == State::OutOfRange && first_run {
            crate::queue_waker(cx.waker().clone());
            Poll::Pending
        } else if item.state == State::OutOfRange {
            Poll::Ready(Err(()))
        } else {
            register_waker(inner.idx, cx.waker().clone());
            Poll::Pending
        }
    }
}

fn get_all_joystick_data() -> (
    [HAL_JoystickAxes; 6],
    [HAL_JoystickPOVs; 6],
    [HAL_JoystickButtons; 6],
) {
    let mut axis: MaybeUninit<[HAL_JoystickAxes; 6]> = MaybeUninit::uninit();
    let mut povs: MaybeUninit<[HAL_JoystickPOVs; 6]> = MaybeUninit::uninit();
    let mut buttons: MaybeUninit<[HAL_JoystickButtons; 6]> = MaybeUninit::uninit();

    unsafe {
        HAL_GetAllJoystickData(
            axis.as_mut_ptr() as *mut HAL_JoystickAxes,
            povs.as_mut_ptr() as *mut HAL_JoystickPOVs,
            buttons.as_mut_ptr() as *mut HAL_JoystickButtons,
        )
    }

    unsafe {
        (
            axis.assume_init(),
            povs.assume_init(),
            buttons.assume_init(),
        )
    }
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let mut queue = QUEUE.lock();

    let data = get_all_joystick_data();

    for (_, item) in queue.deref_mut() {
        let new_state = match item.target {
            Target::Button(target) => {
                let buttons = &data.2[item.joystick.get_num() as usize];
                if let Some(value) = get_button(buttons, item.idx) {
                    if target.is_active(value) {
                        State::Triggered
                    } else {
                        State::Release
                    }
                } else {
                    State::OutOfRange
                }
            }
            Target::Axis(target) => {
                let axis = &data.0[item.joystick.get_num() as usize];
                if let Some(value) = get_axis(axis, item.idx) {
                    if target.is_active(value) {
                        State::Triggered
                    } else {
                        State::Release
                    }
                } else {
                    State::OutOfRange
                }
            }
            Target::Pov(target) => {
                let povs = &data.1[item.joystick.get_num() as usize];
                if let Some(value) = get_pov(povs, item.idx) {
                    if target.is_active(value) {
                        State::Triggered
                    } else {
                        State::Release
                    }
                } else {
                    State::OutOfRange
                }
            }
        };

        if new_state != item.state {
            item.state = new_state;
            if let Some(waker) = item.waker.take() {
                waker.wake();
            }
            if item.state != State::Triggered {
                item.triggered = false;
            }
            if item.state != State::Release {
                item.released = false;
            }
        }
    }
}
