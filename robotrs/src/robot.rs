use std::{pin::Pin, rc::Rc};

use futures::{executor::LocalSpawner, Future};

pub type Fut = Pin<Box<dyn Future<Output = ()> + 'static>>;

// possibly use a custom runtime and non static futures to avoid possible consfusion with rcs
// TODO: Use async in trait when added
pub trait AsyncRobot {
    fn get_auto_future(self: Rc<Self>) -> Fut;

    fn get_enabled_future(self: Rc<Self>) -> Fut;

    fn get_teleop_future(self: Rc<Self>) -> Fut;

    fn create_bindings(self: Rc<Self>, executor: &LocalSpawner);
}
