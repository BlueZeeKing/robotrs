use ::std::os::raw::c_char;
use std::{ffi::CStr, slice, str};

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

pub trait Payload {
    const DATA_TYPE: DataType;
    const DATA_TYPE_NAME: &'static CStr;

    fn from_entry(handle: NT_Handle, default: Self) -> Self;
    fn to_entry(self, handle: NT_Handle, time: i64);
}

impl Payload for f32 {
    const DATA_TYPE: DataType = DataType::Float;
    const DATA_TYPE_NAME: &'static CStr = c"float";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        unsafe { NT_GetFloat(handle, default) }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetFloat(handle, time, self) };
    }
}

impl Payload for f64 {
    const DATA_TYPE: DataType = DataType::Double;
    const DATA_TYPE_NAME: &'static CStr = c"double";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        unsafe { NT_GetDouble(handle, default) }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetDouble(handle, time, self) };
    }
}

impl Payload for i64 {
    const DATA_TYPE: DataType = DataType::Integer;
    const DATA_TYPE_NAME: &'static CStr = c"int";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        unsafe { NT_GetInteger(handle, default) }
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetInteger(handle, time, self) };
    }
}

impl Payload for bool {
    const DATA_TYPE: DataType = DataType::Boolean;
    const DATA_TYPE_NAME: &'static CStr = c"boolean";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let value = unsafe { NT_GetBoolean(handle, if default { 1 } else { 0 }) };
        value == 1
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetBoolean(handle, time, if self { 1 } else { 0 }) };
    }
}

impl Payload for String {
    const DATA_TYPE: DataType = DataType::String;
    const DATA_TYPE_NAME: &'static CStr = c"string";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let string = unsafe {
            NT_GetString(
                handle,
                default.as_ptr() as *const c_char,
                default.len(),
                &mut len,
            )
        };
        let safe_str = str::from_utf8(unsafe { slice::from_raw_parts(string as *const u8, len) });
        let value = safe_str.map(|val| val.to_string()).unwrap_or(default);

        unsafe { NT_DisposeString(&mut NT_String { str_: string, len }) };

        value
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetString(handle, time, self.as_ptr() as *const c_char, self.len()) };
    }
}

pub struct Json(pub String);

impl Payload for Json {
    const DATA_TYPE: DataType = DataType::String;
    const DATA_TYPE_NAME: &'static CStr = c"json";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let string = unsafe {
            NT_GetString(
                handle,
                default.0.as_ptr() as *const c_char,
                default.0.len(),
                &mut len,
            )
        };
        let safe_str = str::from_utf8(unsafe { slice::from_raw_parts(string as *const u8, len) });
        let value = safe_str.map(|val| val.to_string()).unwrap_or(default.0);

        unsafe { NT_DisposeString(&mut NT_String { str_: string, len }) };

        Self(value)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetString(handle, time, self.0.as_ptr() as *const c_char, self.0.len()) };
    }
}

impl Payload for Vec<f32> {
    const DATA_TYPE: DataType = DataType::FloatArray;
    const DATA_TYPE_NAME: &'static CStr = c"float[]";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let array = unsafe { NT_GetFloatArray(handle, default.as_ptr(), default.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice.to_vec();

        unsafe { NT_FreeFloatArray(array) };

        value
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetFloatArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<f64> {
    const DATA_TYPE: DataType = DataType::DoubleArray;
    const DATA_TYPE_NAME: &'static CStr = c"double[]";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let array = unsafe { NT_GetDoubleArray(handle, default.as_ptr(), default.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice.to_vec();

        unsafe { NT_FreeDoubleArray(array) };

        value
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetDoubleArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<i64> {
    const DATA_TYPE: DataType = DataType::IntegerArray;
    const DATA_TYPE_NAME: &'static CStr = c"int[]";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let array =
            unsafe { NT_GetIntegerArray(handle, default.as_ptr(), default.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice.to_vec();

        unsafe { NT_FreeIntegerArray(array) };

        value
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetIntegerArray(handle, time, self.as_ptr(), self.len()) };
    }
}

impl Payload for Vec<bool> {
    const DATA_TYPE: DataType = DataType::Float;
    const DATA_TYPE_NAME: &'static CStr = c"boolean[]";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let transformed = default
            .clone()
            .into_iter()
            .map(|val| if val { 1 } else { 0 })
            .collect::<Vec<_>>();

        let mut len = 0;
        let array = unsafe {
            NT_GetBooleanArray(handle, transformed.as_ptr(), transformed.len(), &mut len)
        };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice
            .iter()
            .copied()
            .map(|val| val == 1)
            .collect::<Vec<_>>();

        unsafe { NT_FreeBooleanArray(array) };

        value
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

    fn from_entry(handle: NT_Handle, mut default: Self) -> Self {
        let transformed = default
            .iter_mut()
            .map(|val| NT_String {
                len: val.len(),
                str_: val.as_mut_ptr() as *mut c_char,
            })
            .collect::<Vec<_>>();

        let mut len = 0;
        let array =
            unsafe { NT_GetStringArray(handle, transformed.as_ptr(), transformed.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
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

                unsafe { NT_DisposeString(&mut val) };

                value
            })
            .collect::<Vec<_>>();

        unsafe { NT_FreeStringArray(array, len) };

        value
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

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let array = unsafe { NT_GetRaw(handle, default.as_ptr(), default.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice.to_vec();

        unsafe { NT_FreeCharArray(array as *mut c_char) };

        value
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetRaw(handle, time, self.as_ptr(), self.len()) };
    }
}

pub struct Rpc(pub Vec<u8>);

impl Payload for Rpc {
    const DATA_TYPE: DataType = DataType::Rpc;
    const DATA_TYPE_NAME: &'static CStr = c"rpc";

    fn from_entry(handle: NT_Handle, default: Self) -> Self {
        let mut len = 0;
        let array = unsafe { NT_GetRaw(handle, default.0.as_ptr(), default.0.len(), &mut len) };

        if array.is_null() {
            return default;
        }

        let safe_slice = unsafe { slice::from_raw_parts(array, len) };
        let value = safe_slice.to_vec();

        unsafe { NT_FreeCharArray(array as *mut c_char) };

        Rpc(value)
    }

    fn to_entry(self, handle: NT_Handle, time: i64) {
        unsafe { NT_SetRaw(handle, time, self.0.as_ptr(), self.0.len()) };
    }
}
