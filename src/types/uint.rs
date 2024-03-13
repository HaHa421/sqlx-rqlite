use crate::rqlite;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Rqlite, /*RqliteArgumentValue,*/ RqliteTypeInfo, RqliteValueRef};

impl Type<Rqlite> for u8 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for u8 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for u8 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int()? as _)
    }
}

impl Type<Rqlite> for u16 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for u16 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for u16 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int()? as _)
    }
}

impl Type<Rqlite> for u32 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Int64)
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, Rqlite> for u32 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::Number((*self).into()));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for u32 {
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int64()? as _)
    }
}
