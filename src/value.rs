use crate::rqlite;
use std::borrow::Cow;
//use std::ptr::NonNull;
//use std::slice::from_raw_parts;
//use std::str::from_utf8;
use std::sync::Arc;
//use rqlite::types::FromJsonValue;
use crate::Rqlite;
use rqlite::types::FromJsonValueRef;
use sqlx_core::type_info::TypeInfo;

/*
use libsqlite3_sys::{
    sqlite3_value, sqlite3_value_blob, sqlite3_value_bytes, sqlite3_value_double,
    sqlite3_value_dup, sqlite3_value_free, sqlite3_value_int, sqlite3_value_int64,
    sqlite3_value_type, SQLITE_NULL,
};
*/
pub(crate) use sqlx_core::value::{Value, ValueRef};

use crate::error::BoxDynError;
//use crate::type_info::DataType;
use crate::{/*Sqlite,*/ RqliteTypeInfo};

enum RqliteValueData<'r> {
    Value(&'r RqliteValue),
}

pub struct RqliteValueRef<'r>(RqliteValueData<'r>);

impl<'r> RqliteValueRef<'r> {
    pub(crate) fn value(value: &'r RqliteValue) -> Self {
        Self(RqliteValueData::Value(value))
    }

    pub(super) fn int(&self) -> Result<i32, BoxDynError> {
        match self.0 {
            RqliteValueData::Value(v) => i32::from_json_value_ref(&v.handle),
        }
    }

    pub(super) fn int64(&self) -> Result<i64, BoxDynError> {
        match self.0 {
            RqliteValueData::Value(v) => i64::from_json_value_ref(&v.handle),
        }
    }

    pub(super) fn double(&self) -> Result<f64, BoxDynError> {
        match self.0 {
            RqliteValueData::Value(v) => f64::from_json_value_ref(&v.handle),
        }
    }
    /*
    pub(super) fn blob(&self) -> Result<&'r [u8],BoxDynError> {
        Err(std::io::Error::new(std::io::ErrorKind::Other , "blob not supported yet").into())
    }
    */
    pub(super) fn text(&self) -> Result<String, BoxDynError> {
        match self.0 {
            RqliteValueData::Value(v) => String::from_json_value_ref(&v.handle),
        }
    }
}

impl<'r> ValueRef<'r> for RqliteValueRef<'r> {
    type Database = Rqlite;

    fn to_owned(&self) -> RqliteValue {
        match self.0 {
            RqliteValueData::Value(v) => v.clone(),
        }
    }

    fn type_info(&self) -> Cow<'_, RqliteTypeInfo> {
        match self.0 {
            RqliteValueData::Value(v) => v.type_info(),
        }
    }

    fn is_null(&self) -> bool {
        match self.0 {
            RqliteValueData::Value(v) => v.is_null(),
        }
    }
}

#[derive(Clone)]
pub struct RqliteValue {
    pub(crate) handle: Arc<rqlite::Value>,
    pub(crate) type_info: RqliteTypeInfo,
}

//pub(crate) struct ValueHandle(NonNull<sqlite3_value>);

// SAFE: only protected value objects are stored in RqliteValue
/*
unsafe impl Send for ValueHandle {}
unsafe impl Sync for ValueHandle {}
*/
impl RqliteValue {
    /*
    pub(crate) unsafe fn new(value: *mut sqlite3_value, type_info: RqliteTypeInfo) -> Self {
        debug_assert!(!value.is_null());

        Self {
            type_info,
            handle: Arc::new(ValueHandle(NonNull::new_unchecked(sqlite3_value_dup(
                value,
            )))),
        }
    }
    */
    pub(crate) fn new(value: rqlite::Value, type_info: RqliteTypeInfo) -> Self {
        Self {
            type_info,
            handle: Arc::new(value),
        }
    }
    fn type_info_opt(&self) -> Option<RqliteTypeInfo> {
        /*
        let dt = DataType::from_code(unsafe { sqlite3_value_type(self.handle.0.as_ptr()) });

        if let DataType::Null = dt {
            None
        } else {
            Some(RqliteTypeInfo(dt))
        }
        */
        Some(self.type_info.clone())
    }
    /*
    fn int(&self) -> i32 {
        //unsafe { sqlite3_value_int(self.handle.0.as_ptr()) }
    }

    fn int64(&self) -> i64 {
        //unsafe { sqlite3_value_int64(self.handle.0.as_ptr()) }
    }

    fn double(&self) -> f64 {
        unsafe { sqlite3_value_double(self.handle.0.as_ptr()) }
    }

    fn blob(&self) -> &[u8] {
        let len = unsafe { sqlite3_value_bytes(self.handle.0.as_ptr()) } as usize;

        if len == 0 {
            // empty blobs are NULL so just return an empty slice
            return &[];
        }

        let ptr = unsafe { sqlite3_value_blob(self.handle.0.as_ptr()) } as *const u8;
        debug_assert!(!ptr.is_null());

        unsafe { from_raw_parts(ptr, len) }
    }

    fn text(&self) -> Result<&str, BoxDynError> {
        Ok(from_utf8(self.blob())?)
    }
    */
}

impl Value for RqliteValue {
    type Database = Rqlite;

    fn as_ref(&self) -> RqliteValueRef<'_> {
        RqliteValueRef::value(self)
    }

    fn type_info(&self) -> Cow<'_, RqliteTypeInfo> {
        self.type_info_opt()
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.type_info))
    }

    fn is_null(&self) -> bool {
        self.type_info.is_null()
    }
}
/*
impl Drop for ValueHandle {
    fn drop(&mut self) {
        /*
        unsafe {
            sqlite3_value_free(self.0.as_ptr());
        }
        */
    }
}
*/
// #[cfg(feature = "any")]
// impl<'r> From<RqliteValueRef<'r>> for crate::any::AnyValueRef<'r> {
//     #[inline]
//     fn from(value: RqliteValueRef<'r>) -> Self {
//         crate::any::AnyValueRef {
//             type_info: value.type_info().clone().into_owned().into(),
//             kind: crate::any::value::AnyValueRefKind::Rqlite(value),
//         }
//     }
// }
//
// #[cfg(feature = "any")]
// impl From<RqliteValue> for crate::any::AnyValue {
//     #[inline]
//     fn from(value: RqliteValue) -> Self {
//         crate::any::AnyValue {
//             type_info: value.type_info().clone().into_owned().into(),
//             kind: crate::any::value::AnyValueKind::Rqlite(value),
//         }
//     }
// }
