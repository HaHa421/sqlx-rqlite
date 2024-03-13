use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Rqlite, /*SqliteArgumentValue,*/ RqliteTypeInfo, RqliteValueRef};

impl Type<Rqlite> for f32 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Float)
    }
}

impl<'q> Encode<'q, Rqlite> for f32 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::from(*self));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for f32 {
    fn decode(value: RqliteValueRef<'r>) -> Result<f32, BoxDynError> {
        Ok(value.double()? as f32)
    }
}

impl Type<Rqlite> for f64 {
    fn type_info() -> RqliteTypeInfo {
        RqliteTypeInfo(DataType::Float)
    }
}

impl<'q> Encode<'q, Rqlite> for f64 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::from(*self));

        IsNull::No
    }
}

impl<'r> Decode<'r, Rqlite> for f64 {
    fn decode(value: RqliteValueRef<'r>) -> Result<f64, BoxDynError> {
        Ok(value.double()?)
    }
}
