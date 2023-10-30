use std::ops::{Deref, DerefMut};

use async_lock::{Mutex, MutexGuard};
use impl_trait_for_tuples::impl_for_tuples;

use crate::FailableDefault;

pub struct ControlLock<T: ControlSafe> {
    inner: Mutex<T>,
}

impl<T: ControlSafe> ControlLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Mutex::new(data),
        }
    }

    /// Return an RAII guard to the data within. Not guaranteed to be fair.
    pub async fn lock(&self) -> ControlGuard<T> {
        ControlGuard { lock: self.inner.lock().await }
    }
}

pub struct ControlGuard<'a, T: ControlSafe> {
    lock: MutexGuard<'a, T>,
}

impl<'a, T: ControlSafe> Deref for ControlGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a, T: ControlSafe> DerefMut for ControlGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.lock.deref_mut()
    }
}

impl<'a, T: ControlSafe> Drop for ControlGuard<'a, T> {
    fn drop(&mut self) {
        self.stop();
    }
}

// FIXME: Also have a default impl somehow?
impl<T: ControlSafe + FailableDefault> FailableDefault for ControlLock<T> {
    fn failable_default() -> anyhow::Result<Self> {
        Ok(ControlLock::new(T::failable_default()?))
    }
}

/// Similar to the wpilib MotorSafety class. The easiest way to properly implement this is to
/// simply call the stop method of all motors and actuators used
#[impl_for_tuples(10)]
pub trait ControlSafe {
    /// Stop all motors and actuators. This method gets automatically called whenever a control
    /// guard goes out of scope.
    fn stop(&mut self);
}
