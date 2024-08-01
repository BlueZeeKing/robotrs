#[allow(warnings)]
pub mod bindings;
pub mod options;
pub mod payloads;

use std::{ffi::CString, marker::PhantomData};

use bindings::*;
use options::PubSubOptions;
use payloads::Payload;

pub struct Instance {
    handle: NT_Inst,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            handle: unsafe { NT_GetDefaultInstance() },
        }
    }
}

impl Instance {
    pub fn new() -> Self {
        Self {
            handle: unsafe { NT_CreateInstance() },
        }
    }

    pub fn topic(&self, name: &str) -> Topic {
        Topic {
            handle: unsafe {
                NT_GetTopic(
                    self.handle,
                    CString::new(name).unwrap().into_raw(),
                    name.len(),
                )
            },
        }
    }

    pub fn start_server(&self, persist: &str) {
        unsafe {
            NT_StartServer(
                self.handle,
                CString::new(persist).unwrap().into_raw(),
                c"".as_ptr(),
                1735,
                5810,
            )
        };
    }

    pub fn is_starting(&self) -> bool {
        let mode = unsafe { NT_GetNetworkMode(self.handle) };
        mode & NT_NetworkMode_NT_NET_MODE_STARTING != 0
    }
}

pub struct Topic {
    handle: NT_Topic,
}

impl Topic {
    pub fn publish<T: Payload>(&self, options: PubSubOptions) -> Publisher<T> {
        Publisher {
            handle: unsafe {
                NT_Publish(
                    self.handle,
                    T::DATA_TYPE.into(),
                    T::DATA_TYPE_NAME.as_ptr(),
                    &options.build(),
                )
            },
            payload: PhantomData,
        }
    }

    pub fn publish_with_type_str<T: Payload>(
        &self,
        options: PubSubOptions,
        type_str: &str,
    ) -> Publisher<T> {
        Publisher {
            handle: unsafe {
                NT_Publish(
                    self.handle,
                    T::DATA_TYPE.into(),
                    CString::new(type_str).unwrap().into_raw(),
                    &options.build(),
                )
            },
            payload: PhantomData,
        }
    }

    pub fn subscribe<T: Payload>(&self, options: PubSubOptions) -> Subscriber<T> {
        Subscriber {
            handle: unsafe {
                NT_Subscribe(
                    self.handle,
                    T::DATA_TYPE.into(),
                    T::DATA_TYPE_NAME.as_ptr(),
                    &options.build(),
                )
            },
            payload: PhantomData,
        }
    }

    pub fn subscribe_with_type_str<T: Payload>(
        &self,
        options: PubSubOptions,
        type_str: &str,
    ) -> Subscriber<T> {
        Subscriber {
            handle: unsafe {
                NT_Subscribe(
                    self.handle,
                    T::DATA_TYPE.into(),
                    CString::new(type_str).unwrap().into_raw(),
                    &options.build(),
                )
            },
            payload: PhantomData,
        }
    }
}

pub struct Publisher<T> {
    handle: NT_Publisher,
    payload: PhantomData<T>,
}

impl<T: Payload> Publisher<T> {
    pub fn set(&self, value: T) {
        value.to_entry(self.handle, unsafe { NT_Now() });
    }
}

pub struct Subscriber<T> {
    handle: NT_Subscriber,
    payload: PhantomData<T>,
}

impl<T: Payload> Subscriber<T> {
    pub fn get_with_default(&self, default: T) -> T {
        T::from_entry(self.handle, default)
    }

    pub fn get(&self) -> T
    where
        T: Default,
    {
        self.get_with_default(T::default())
    }
}

impl<T> Drop for Subscriber<T> {
    fn drop(&mut self) {
        unsafe { NT_Unsubscribe(self.handle) }
    }
}

impl<T> Drop for Publisher<T> {
    fn drop(&mut self) {
        unsafe { NT_Unpublish(self.handle) }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { NT_DestroyInstance(self.handle) }
    }
}
