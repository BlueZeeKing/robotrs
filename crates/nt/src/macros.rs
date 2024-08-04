use std::sync::OnceLock;

use crate::{payloads::Payload, Instance, NT_Now, NT_Publisher};

#[macro_export]
macro_rules! nt {
    ($topic_name:literal, $value:expr) => {{
        static PUBLISHER: ::std::sync::OnceLock<$crate::bindings::NT_Publisher> =
            ::std::sync::OnceLock::new();

        $crate::macros::_internal_set(&PUBLISHER, $topic_name, $value);
    }};
}

pub fn _internal_set<T: Payload>(
    handle: &'static OnceLock<NT_Publisher>,
    topic_name: &str,
    value: T,
) {
    value.to_entry(
        *handle.get_or_init(|| {
            let publisher = Instance::default_instance()
                .topic(topic_name)
                .publish::<T>(Default::default());

            let handle = publisher.handle;

            std::mem::forget(publisher);

            handle
        }),
        unsafe { NT_Now() },
    );
}
