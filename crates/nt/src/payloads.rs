use ::std::os::raw::c_char;
use std::{ffi::CStr, mem::MaybeUninit, slice, str};

use crate::bindings::*;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum DataType {
    Raw,
    Rpc,
    Float,
    Double,
    String,
    Boolean,
    Integer,
    Unassigned,
    FloatArray,
    DoubleArray,
    StringArray,
    BooleanArray,
    IntegerArray,
}

impl From<NT_Type> for DataType {
    fn from(value: NT_Type) -> Self {
        #[allow(non_upper_case_globals)]
        match value {
            NT_Type_NT_RAW => DataType::Raw,
            NT_Type_NT_RPC => DataType::Rpc,
            NT_Type_NT_FLOAT => DataType::Float,
            NT_Type_NT_DOUBLE => DataType::Double,
            NT_Type_NT_STRING => DataType::String,
            NT_Type_NT_BOOLEAN => DataType::Boolean,
            NT_Type_NT_INTEGER => DataType::Integer,
            NT_Type_NT_UNASSIGNED => DataType::Unassigned,
            NT_Type_NT_FLOAT_ARRAY => DataType::FloatArray,
            NT_Type_NT_DOUBLE_ARRAY => DataType::DoubleArray,
            NT_Type_NT_STRING_ARRAY => DataType::StringArray,
            NT_Type_NT_BOOLEAN_ARRAY => DataType::BooleanArray,
            NT_Type_NT_INTEGER_ARRAY => DataType::IntegerArray,
            _ => unreachable!(),
        }
    }
}

impl From<DataType> for NT_Type {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Raw => NT_Type_NT_RAW,
            DataType::Rpc => NT_Type_NT_RPC,
            DataType::Float => NT_Type_NT_FLOAT,
            DataType::Double => NT_Type_NT_DOUBLE,
            DataType::String => NT_Type_NT_STRING,
            DataType::Boolean => NT_Type_NT_BOOLEAN,
            DataType::Integer => NT_Type_NT_INTEGER,
            DataType::Unassigned => NT_Type_NT_UNASSIGNED,
            DataType::FloatArray => NT_Type_NT_FLOAT_ARRAY,
            DataType::DoubleArray => NT_Type_NT_DOUBLE_ARRAY,
            DataType::StringArray => NT_Type_NT_STRING_ARRAY,
            DataType::BooleanArray => NT_Type_NT_BOOLEAN_ARRAY,
            DataType::IntegerArray => NT_Type_NT_INTEGER_ARRAY,
        }
    }
}

pub trait Payload: Sized {
    const DATA_TYPE: DataType;
    const DATA_TYPE_NAME: &'static CStr;

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64);
    fn to_entry(self, handle: NT_Handle, time: i64);
    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        Self::from_entry_with_time(handle, default).0
    }
}

impl Payload for f32 {
    const DATA_TYPE: DataType = DataType::Float;
    const DATA_TYPE_NAME: &'static CStr = c"float";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut value = MaybeUninit::uninit();
        unsafe {
            NT_GetAtomicFloat(handle, default, value.as_mut_ptr());
            let mut value = value.assume_init();
            let res = (value.value, value.time);
            NT_DisposeTimestampedFloat(&mut value);
            res
        }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetFloat(handle, time, self) };
    }
}

impl Payload for f64 {
    const DATA_TYPE: DataType = DataType::Double;
    const DATA_TYPE_NAME: &'static CStr = c"double";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut value = MaybeUninit::uninit();
        unsafe {
            NT_GetAtomicDouble(handle, default, value.as_mut_ptr());
            let mut value = value.assume_init();
            let res = (value.value, value.time);
            NT_DisposeTimestampedDouble(&mut value);
            res
        }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetDouble(handle, time, self) };
    }
}

impl Payload for i64 {
    const DATA_TYPE: DataType = DataType::Integer;
    const DATA_TYPE_NAME: &'static CStr = c"int";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut value = MaybeUninit::uninit();
        unsafe {
            NT_GetAtomicInteger(handle, default, value.as_mut_ptr());
            let mut value = value.assume_init();
            let res = (value.value, value.time);
            NT_DisposeTimestampedInteger(&mut value);
            res
        }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetInteger(handle, time, self) };
    }
}

impl Payload for bool {
    const DATA_TYPE: DataType = DataType::Boolean;
    const DATA_TYPE_NAME: &'static CStr = c"boolean";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut value = MaybeUninit::uninit();
        unsafe {
            NT_GetAtomicBoolean(handle, if default { 1 } else { 0 }, value.as_mut_ptr());
            let mut value = value.assume_init();

            let res = (value.value == 1, value.time);

            NT_DisposeTimestampedBoolean(&mut value);

            res
        }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetBoolean(handle, time, if self { 1 } else { 0 }) };
    }
}

impl Payload for String {
    const DATA_TYPE: DataType = DataType::String;
    const DATA_TYPE_NAME: &'static CStr = c"string";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut string = unsafe {
            let mut string = MaybeUninit::uninit();

            NT_GetAtomicString(
                handle,
                default.as_ptr() as *const c_char,
                default.len(),
                string.as_mut_ptr(),
            );

            string.assume_init()
        };
        let safe_str =
            str::from_utf8(unsafe { slice::from_raw_parts(string.value as *const u8, string.len) });
        let value = safe_str.map(|val| val.to_string()).unwrap_or(default);
        let time = string.time;

        unsafe { NT_DisposeTimestampedString(&mut string) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetString(handle, time, self.as_ptr() as *const c_char, self.len()) };
    }
}

pub struct Json(pub String);

impl Payload for Json {
    const DATA_TYPE: DataType = DataType::String;
    const DATA_TYPE_NAME: &'static CStr = c"json";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let (value, time) = String::from_entry_with_time(handle, default.0);
        (Self(value), time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        self.0.to_entry(handle, time)
    }
}

impl Payload for Vec<f32> {
    const DATA_TYPE: DataType = DataType::FloatArray;
    const DATA_TYPE_NAME: &'static CStr = c"float[]";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicFloatArray(handle, default.as_ptr(), default.len(), array.as_mut_ptr());
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice.to_vec();
        let time = array.time;

        unsafe { NT_DisposeTimestampedFloatArray(&mut array) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetFloatArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<f64> {
    const DATA_TYPE: DataType = DataType::DoubleArray;
    const DATA_TYPE_NAME: &'static CStr = c"double[]";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicDoubleArray(handle, default.as_ptr(), default.len(), array.as_mut_ptr());
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice.to_vec();
        let time = array.time;

        unsafe { NT_DisposeTimestampedDoubleArray(&mut array) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetDoubleArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<i64> {
    const DATA_TYPE: DataType = DataType::IntegerArray;
    const DATA_TYPE_NAME: &'static CStr = c"int[]";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicIntegerArray(handle, default.as_ptr(), default.len(), array.as_mut_ptr());
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice.to_vec();
        let time = array.time;

        unsafe { NT_DisposeTimestampedIntegerArray(&mut array) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetIntegerArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<bool> {
    const DATA_TYPE: DataType = DataType::Float;
    const DATA_TYPE_NAME: &'static CStr = c"boolean[]";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let transformed = default
            .clone()
            .into_iter()
            .map(|val| if val { 1 } else { 0 })
            .collect::<Vec<_>>();

        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicBooleanArray(
                handle,
                transformed.as_ptr(),
                transformed.len(),
                array.as_mut_ptr(),
            );
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice
            .iter()
            .copied()
            .map(|val| val == 1)
            .collect::<Vec<_>>();
        let time = array.time;

        unsafe { NT_DisposeTimestampedBooleanArray(&mut array) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        let transformed = self
            .into_iter()
            .map(|val| if val { 1 } else { 0 })
            .collect::<Vec<_>>();

        unsafe { NT_SetBooleanArray(handle, time, transformed.as_ptr(), transformed.len()) };
    }
}

impl Payload for Vec<String> {
    const DATA_TYPE: DataType = DataType::StringArray;
    const DATA_TYPE_NAME: &'static CStr = c"string[]";

    fn from_entry_with_time(handle: NT_Handle, mut default: Self) -> (Self, i64) {
        let transformed = default
            .iter_mut()
            .map(|val| NT_String {
                len: val.len(),
                str_: val.as_mut_ptr() as *mut c_char,
            })
            .collect::<Vec<_>>();

        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicStringArray(
                handle,
                transformed.as_ptr(),
                transformed.len(),
                array.as_mut_ptr(),
            );
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice
            .iter()
            .copied()
            .map(|mut val| {
                let safe_str = str::from_utf8(unsafe {
                    slice::from_raw_parts(val.str_ as *const u8, val.len)
                });
                let value = safe_str
                    .map(|val| val.to_string())
                    .expect("Invalid string from network tables");

                unsafe { NT_DisposeString(&mut val) }; // TODO: This might be wrong?

                value
            })
            .collect::<Vec<_>>();
        let time = array.time;

        unsafe { NT_DisposeTimestampedStringArray(&mut array) };

        (value, time)
    }

    fn to_entry(mut self, handle: NT_Handle, time: i64) {
        let transformed = self
            .iter_mut()
            .map(|val| NT_String {
                len: val.len(),
                str_: val.as_mut_ptr() as *mut c_char,
            })
            .collect::<Vec<_>>();

        unsafe { NT_SetStringArray(handle, time, transformed.as_ptr(), transformed.len()) };
    }
}

impl Payload for Vec<u8> {
    const DATA_TYPE: DataType = DataType::Raw;
    const DATA_TYPE_NAME: &'static CStr = c"raw";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let mut array = unsafe {
            let mut array = MaybeUninit::uninit();
            NT_GetAtomicRaw(handle, default.as_ptr(), default.len(), array.as_mut_ptr());
            array.assume_init()
        };

        if array.value.is_null() {
            return (default, 0);
        }

        let safe_slice = unsafe { slice::from_raw_parts(array.value, array.len) };
        let value = safe_slice.to_vec();
        let time = array.time;

        unsafe { NT_DisposeTimestampedRaw(&mut array) };

        (value, time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetRaw(handle, time, self.as_ptr(), self.len()) };
    }
}

pub struct Rpc(pub Vec<u8>);

impl Payload for Rpc {
    const DATA_TYPE: DataType = DataType::Rpc;
    const DATA_TYPE_NAME: &'static CStr = c"rpc";

    fn from_entry_with_time(handle: NT_Handle, default: Self) -> (Self, i64) {
        let (value, time) = <Vec<u8>>::from_entry_with_time(handle, default.0);

        (Self(value), time)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        self.0.to_entry(handle, time)
    }
}
