use std::{ops::DerefMut, task::Waker};

use linkme::distributed_slice;
use parking_lot::Mutex;

use super::{
    axis::{get_axis, AxisTarget},
    button::get_button,
    joystick::Joystick,
};
use crate::PERIODIC_CHECKS;

#[derive(Debug)]
struct JoystickQueueItem {
    pub joystick: Joystick,
    pub buttons: Vec<(u32, bool, Waker)>,
    pub axis: Vec<(u32, bool, AxisTarget, Waker)>,
}

static QUEUE: Mutex<[Option<JoystickQueueItem>; 6]> =
    Mutex::new([None, None, None, None, None, None]);

pub fn add_button(joystick: &Joystick, index: u32, pressed: bool, waker: Waker) {
    let mut queue = QUEUE.lock();

    if let Some(item) = &mut queue[joystick.get_num() as usize] {
        item.buttons.push((index, pressed, waker));
    } else {
        queue[joystick.get_num() as usize] = Some(JoystickQueueItem {
            joystick: joystick.clone(),
            buttons: vec![(index, pressed, waker)],
            axis: vec![],
        });
    }
}

pub fn add_axis(joystick: &Joystick, index: u32, initial: bool, target: AxisTarget, waker: Waker) {
    let mut queue = QUEUE.lock();

    if let Some(item) = &mut queue[joystick.get_num() as usize] {
        item.axis.push((index, initial, target, waker));
    } else {
        queue[joystick.get_num() as usize] = Some(JoystickQueueItem {
            joystick: joystick.clone(),
            buttons: vec![],
            axis: vec![(index, initial, target, waker)],
        })
    }
}

#[distributed_slice(PERIODIC_CHECKS)]
fn poll() {
    let mut queue = QUEUE.lock();

    for item in queue.deref_mut().iter_mut().filter_map(|val| val.as_mut()) {
        if let Ok(data) = item.joystick.get_button_data() {
            item.buttons.retain(|(index, pressed, waker)| {
                let Ok(button_val) = get_button(&data, *index) else {
                    waker.wake_by_ref();
                    return false;
                };

                if (button_val && *pressed) || (!button_val && !*pressed) {
                    waker.wake_by_ref();
                    false
                } else {
                    true
                }
            });
        } else {
            let wakers = std::mem::take(&mut item.buttons);
            wakers.into_iter().for_each(|(_, _, waker)| waker.wake());
        }

        if let Ok(data) = item.joystick.get_axes_data() {
            item.axis.retain(|(index, initial, target, waker)| {
                let Ok(value) = get_axis(&data, *index) else {
                    waker.wake_by_ref();
                    return false;
                };

                let active = target.is_active(value);

                if (active && *initial) || (!active && !*initial) {
                    waker.wake_by_ref();
                    false
                } else {
                    true
                }
            });
        } else {
            let wakers = std::mem::take(&mut item.axis);
            wakers.into_iter().for_each(|(_, _, _, waker)| waker.wake());
        }
    }
}
