use crate::{Rqlite, RqliteTypeInfo};
use sqlx_core::ext::ustr::UStr;

pub(crate) use sqlx_core::column::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct RqliteColumn {
    pub(crate) name: UStr,
    pub(crate) ordinal: usize,
    pub(crate) type_info: RqliteTypeInfo,
}

impl Column for RqliteColumn {
    type Database = Rqlite;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &*self.name
    }

    fn type_info(&self) -> &RqliteTypeInfo {
        &self.type_info
    }
}
