use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Rqlite, /*RqliteArgumentValue,*/ RqliteTypeInfo, RqliteValueRef};

impl Type<Rqlite> for i8 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for i8 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for i8 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int()?.try_into()?)
    }
}

impl Type<Rqlite> for i16 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for i16 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for i16 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int()?.try_into()?)
    }
}

impl Type<Rqlite> for i32 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for i32 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for i32 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int()?)
    }
}

impl Type<Rqlite> for i64 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int64)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for i64 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for i64 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int64()?)
    }
}
