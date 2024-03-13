#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use sqlx_core::column::ColumnIndex;
use sqlx_core::error::Error;
use sqlx_core::ext::ustr::UStr;
use sqlx_core::row::Row;
use sqlx_core::HashMap;

//use crate::statement::StatementHandle;
use crate::{Rqlite, RqliteColumn, RqliteValue, RqliteValueRef};

/// Implementation of [`Row`] for SQLite.
pub struct RqliteRow {
    pub(crate) values: Box<[RqliteValue]>,
    pub(crate) columns: Arc<Vec<RqliteColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

// Accessing values from the statement object is
// safe across threads as long as we don't call [sqlite3_step]

// we block ourselves from doing that by only exposing
// a set interface on [StatementHandle]

//unsafe impl Send for RqliteRow {}
//unsafe impl Sync for RqliteRow {}

impl RqliteRow {
    /*
    pub(crate) fn current(
        statement: &StatementHandle,
        columns: &Arc<Vec<RqliteColumn>>,
        column_names: &Arc<HashMap<UStr, usize>>,
    ) -> Self {
        let size = statement.column_count();
        let mut values = Vec::with_capacity(size);

        for i in 0..size {
            values.push(unsafe {
                let raw = statement.column_value(i);

                RqliteValue::new(raw, columns[i].type_info.clone())
            });
        }

        Self {
            values: values.into_boxed_slice(),
            columns: Arc::clone(columns),
            column_names: Arc::clone(column_names),
        }
    }
    */
}

impl Row for RqliteRow {
    type Database = Rqlite;

    fn columns(&self) -> &[RqliteColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<RqliteValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(RqliteValueRef::value(&self.values[index]))
    }
}

impl ColumnIndex<RqliteRow> for &'_ str {
    fn index(&self, row: &RqliteRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}

// #[cfg(feature = "any")]
// impl From<RqliteRow> for crate::any::AnyRow {
//     #[inline]
//     fn from(row: RqliteRow) -> Self {
//         crate::any::AnyRow {
//             columns: row.columns.iter().map(|col| col.clone().into()).collect(),
//             kind: crate::any::row::AnyRowKind::Rqlite(row),
//         }
//     }
// }
