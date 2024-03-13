use crate::rqlite;
use crate::encode::{Encode, IsNull};
//use crate::types::Type;
//use crate::type_info::DataType;
use crate::{Rqlite, RqliteTypeInfo};
//use crate::error::BoxDynError;

pub(crate) use sqlx_core::arguments::*;
use sqlx_core::types::Type;
//use sqlx_core::encode::IsNull;

//use rqlite::types::ToJsonValue;
/// Implementation of [`Arguments`] for MySQL.
#[derive(Debug, Default, Clone)]
pub struct RqliteArguments {
    pub(crate) values: Vec<rqlite::Value>,
    pub(crate) types: Vec<RqliteTypeInfo>,
}
/*
impl<'q> Encode<'q, Rqlite> for i8 {
    fn encode_by_ref(&self, args: &mut Vec<rqlite::Value>) -> IsNull {
        args.push(rqlite::Value::from(*self));

        IsNull::No
    }
}
*/
impl RqliteArguments {
    pub(crate) fn add<'q, T>(&mut self, value: T)
    where
        T: Encode<'q, Rqlite> + Type<Rqlite>,
    {
        let ty = T::type_info();

        if let IsNull::Yes = value.encode_by_ref(&mut self.values) {
        } else {
        }
        self.types.push(ty);
    }

    #[doc(hidden)]
    pub fn len(&self) -> usize {
        self.types.len()
    }
}

impl<'q> Arguments<'q> for RqliteArguments {
    type Database = Rqlite;

    fn reserve(&mut self, len: usize, size: usize) {
        self.types.reserve(len);
        self.values.reserve(size);
    }

    fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, Self::Database> + Type<Self::Database>,
    {
        self.add(value)
    }
}
