use crate::{Rqlite, /*RqliteArgumentValue, */ RqliteTypeInfo, RqliteValueRef};
use sqlx_core::decode::Decode;
use sqlx_core::encode::{Encode, IsNull};
use sqlx_core::error::BoxDynError;
use sqlx_core::types::{Text, Type};
use std::fmt::Display;
use std::str::FromStr;

impl<T> Type<Rqlite> for Text<T> {
    fn type_info() -> RqliteTypeInfo {
        <String as Type<Rqlite>>::type_info()
    }

    fn compatible(ty: &RqliteTypeInfo) -> bool {
        <String as Type<Rqlite>>::compatible(ty)
    }
}

impl<'q, T> Encode<'q, Rqlite> for Text<T>
where
    T: Display,
{
    fn encode_by_ref(&self, buf: &mut Vec<rqlite::Value>) -> IsNull {
        Encode::<Rqlite>::encode(self.0.to_string(), buf)
    }
}

impl<'r, T> Decode<'r, Rqlite> for Text<T>
where
    T: FromStr,
    BoxDynError: From<<T as FromStr>::Err>,
{
    fn decode(value: RqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ha better to decode &str
        let s: String = Decode::<Rqlite>::decode(value)?;
        Ok(Self(s.as_str().parse()?))
    }
}
