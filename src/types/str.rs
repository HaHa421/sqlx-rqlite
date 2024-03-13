use crate::rqlite;
//use std::borrow::Cow;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Rqlite, /*RqliteArgumentValue, */ RqliteTypeInfo, RqliteValueRef};

impl Type<Rqlite> for str {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Text)
    }
}

impl<'q> Encode<'q, Rqlite> for &'q str {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }
}
/*
impl<'r> Decode<'r, Rqlite> for &'r str {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        value.text().map(|x| {
          x.as_str()
        })
    }
}
*/
impl Type<Rqlite> for Box<str> {
    fn type_info() -> RqliteTypeInfo {
        <&str as Type<Rqlite>>::type_info()
    }
}

impl Encode<'_, Rqlite> for Box<str> {
    fn encode(self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }
}

impl Decode<'_, Rqlite> for Box<str> {
    fn decode(value: RqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        value.text().map(Box::from)
    }
}

impl Type<Rqlite> for String {
    fn type_info() -> RqliteTypeInfo {
        <&str as Type<Rqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Rqlite> for String {
    fn encode(self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for String {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        value.text()
    }
}
/*
impl Type<Rqlite> for Cow<'_, str> {
    fn type_info() -> RqliteTypeInfo {
        <&str as Type<Rqlite>>::type_info()
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        <&str as Type<Rqlite>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Rqlite> for Cow<'q, str> {
    fn encode(self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::String(self.to_string()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for Cow<'r, str> {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        value.text().map(Cow::Borrowed)
    }
}
*/
