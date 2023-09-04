use std::{
    cell::{RefCell, UnsafeCell},
    ops::{Deref, DerefMut},
};

use impl_trait_for_tuples::impl_for_tuples;

use crate::create_future;

pub struct ControlLock<T: ControlSafe> {
    locked: RefCell<bool>,
    data: UnsafeCell<T>,
}

impl<T: ControlSafe> ControlLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: RefCell::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Return an RAII guard to the data within. Not guaranteed to be fair.
    pub async fn lock(&self) -> ControlGuard<T> {
        if *self.locked.borrow_mut() {
            create_future(move || {}, move || !*self.locked.borrow(), move || {}).await;
        }
        assert_eq!(*self.locked.borrow(), false);

        *self.locked.borrow_mut() = true;

        ControlGuard { lock: &self }
    }

    /// Gain mutable access to the data for a brief period. The data may be locked by another
    /// thread but it is gauranteed that data will not be accessed at the same exact time. Be
    /// careful when using this.
    pub unsafe fn steal<F: FnOnce(&mut T)>(&self, function: F) {
        function(unsafe { &mut *self.data.get() });

        if !*self.locked.borrow() {
            unsafe { &mut *self.data.get() }.stop();
        }
    }
}

pub struct ControlGuard<'a, T: ControlSafe> {
    lock: &'a ControlLock<T>,
}

impl<'a, T: ControlSafe> Deref for ControlGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: ControlSafe> DerefMut for ControlGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T: ControlSafe> Drop for ControlGuard<'a, T> {
    fn drop(&mut self) {
        self.stop();
        *self.lock.locked.borrow_mut() = false;
    }
}

impl<T: ControlSafe + Default> Default for ControlLock<T> {
    fn default() -> Self {
        Self::new(T::default())
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
