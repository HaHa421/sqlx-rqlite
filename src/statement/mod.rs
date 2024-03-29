use crate::column::ColumnIndex;
use crate::error::Error;
use crate::{Rqlite, RqliteArguments, RqliteColumn, RqliteTypeInfo};
use sqlx_core::ext::ustr::UStr;
use sqlx_core::{Either, HashMap};
use std::borrow::Cow;
use std::sync::Arc;

use sqlx_core::impl_statement_query;
pub(crate) use sqlx_core::statement::*;
/*
mod handle;
pub(super) mod unlock_notify;
mod r#virtual;

pub(crate) use handle::StatementHandle;
pub(crate) use r#virtual::VirtualStatement;
*/
#[derive(Debug, Clone)]
#[allow(clippy::rc_buffer)]
pub struct RqliteStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) parameters: usize,
    pub(crate) columns: Arc<Vec<RqliteColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl<'q> Statement<'q> for RqliteStatement<'q> {
    type Database = Rqlite;

    fn to_owned(&self) -> RqliteStatement<'static> {
        RqliteStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[RqliteTypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    fn columns(&self) -> &[RqliteColumn] {
        &self.columns
    }

    impl_statement_query!(RqliteArguments);
}

impl ColumnIndex<RqliteStatement<'_>> for &'_ str {
    fn index(&self, statement: &RqliteStatement<'_>) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}

// #[cfg(feature = "any")]
// impl<'q> From<RqliteStatement<'q>> for crate::any::AnyStatement<'q> {
//     #[inline]
//     fn from(statement: RqliteStatement<'q>) -> Self {
//         crate::any::AnyStatement::<'q> {
//             columns: statement
//                 .columns
//                 .iter()
//                 .map(|col| col.clone().into())
//                 .collect(),
//             column_names: statement.column_names,
//             parameters: Some(Either::Right(statement.parameters)),
//             sql: statement.sql,
//         }
//     }
// }
