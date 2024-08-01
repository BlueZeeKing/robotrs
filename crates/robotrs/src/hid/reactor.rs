use std::{
    mem::MaybeUninit,
    ops::DerefMut,
    pin::Pin,
    sync::LazyLock,
    task::{Poll, Waker},
};

use futures::Future;
use hal_sys::{HAL_GetAllJoystickData, HAL_JoystickAxes, HAL_JoystickButtons, HAL_JoystickPOVs};
use linkme::distributed_slice;
use parking_lot::Mutex;
use slab::Slab;
use tracing::{span, trace, Instrument, Level, Span};

use super::{
    axis::{get_axis, AxisTarget},
    button::{get_button, ButtonTarget},
    joystick::Joystick,
    pov::{get_pov, PovTarget},
};
use crate::PERIODIC_CHECKS;

#[derive(Debug)]
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

fn type_from_target(target: &Target) -> &'static str {
    match target {
        Target::Button(_) => "button",
        Target::Axis(_) => "axis",
        Target::Pov(_) => "pov",
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Triggered,
    Release,
    OutOfRange,
    Unknown,
}

#[derive(Debug)]
struct JoystickQueueItem {
    joystick: Joystick,
    idx: u32,
    target: Target,
    waker: Option<Waker>,
    state: State,
    triggered: bool,
    released: bool,
    span: Span,
}

static QUEUE: Mutex<Slab<JoystickQueueItem>> = Mutex::new(Slab::new());

pub fn add_trigger(joystick: &Joystick, index: u32, target: Target) -> usize {
    trace!(?joystick, index, ?target, "Add trigger");
    QUEUE.lock().insert(JoystickQueueItem {
        span: span!(
            Level::TRACE,
            "polling trigger",
            joystick = joystick.get_num(),
            input_type = type_from_target(&target),
            input_idx = index
        ),
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

async fn wait_for_reactor(idx: usize, on_release: bool) -> Result<(), ()> {
    let span = {
        let queue = QUEUE.lock();
        let item = queue.get(idx).unwrap();

        span!(
            Level::TRACE,
            "Reactor future",
            on_release,
            idx,
            joystick = item.joystick.get_num(),
            target = ?item.target,
        )
    };

    ReactorFuture {
        idx,
        on_release,
        first_run: true,
    }
    .instrument(span)
    .await
}

pub async fn wait_for_triggered(idx: usize) -> Result<(), ()> {
    wait_for_reactor(idx, false).await
}

pub async fn wait_for_released(idx: usize) -> Result<(), ()> {
    wait_for_reactor(idx, true).await
}

#[derive(Debug)]
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

        trace!(trigger_state = ?item.state, triggered = item.triggered, released = item.released);

        if !item.triggered && item.state == State::Triggered && !inner.on_release {
            trace!("Trigger not previously triggered, but is now triggered. Setting triggered to true and completing future");
            item.triggered = true;
            Poll::Ready(Ok(()))
        } else if !item.released && item.state == State::Release && inner.on_release {
            trace!("Trigger not previously released, but is now released. Setting released to true and completing future");
            item.released = true;
            Poll::Ready(Ok(()))
        } else if item.state == State::OutOfRange && first_run {
            trace!("Trigger out of range, queuing waker to be woken up next tick");
            crate::queue_waker(cx.waker().clone());
            Poll::Pending
        } else if item.state == State::OutOfRange {
            trace!("Trigger out of range, returning error");
            Poll::Ready(Err(()))
        } else {
            trace!("No changes, registering waker and returning pending");
            drop(lock);
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

static POLL_SPAN: LazyLock<Span> = LazyLock::new(|| span!(Level::TRACE, "hid poll"));

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let _span_guard = POLL_SPAN.enter();
    let mut queue = QUEUE.lock();

    let data = get_all_joystick_data();

    for (_, item) in queue.deref_mut() {
        let _inner_span_guard = item.span.enter();

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
            trace!(old_state = ?item.state, ?new_state, "Changing state");
            item.state = new_state;
            if item.state != State::Triggered {
                trace!(previous_triggered_value = item.triggered, "Not triggered");
                item.triggered = false;
            }
            if item.state != State::Release {
                trace!(previous_released_value = item.released, "Not released");
                item.released = false;
            }
            if let Some(waker) = item.waker.take() {
                waker.wake();
            } else {
                trace!("No waker found");
            }
        }
    }
}
