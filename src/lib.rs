#[macro_use]
extern crate sqlx_core;

pub(crate) use sqlx_core::driver_prelude::*;

pub mod error;
pub mod type_info;
use type_info::RqliteTypeInfo;

mod types;

mod options;
pub use options::RqliteConnectOptions;
pub mod connection;
use connection::RqliteConnection;
pub mod arguments;
use arguments::RqliteArguments;
pub mod column;
use column::RqliteColumn;

pub mod statement;
use statement::RqliteStatement;

pub mod row;
use row::RqliteRow;

pub mod query_result;
use query_result::RqliteQueryResult;

pub mod transaction;
use transaction::RqliteTransactionManager;
pub mod database;
use database::Rqlite;

pub mod value;
use value::*;

impl_into_arguments_for_arguments!(RqliteArguments);
impl_acquire!(Rqlite, RqliteConnection);
impl_column_index_for_row!(RqliteRow);
impl_column_index_for_statement!(RqliteStatement);

pub type RqlitePool = crate::pool::Pool<Rqlite>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for SQLite.
pub type RqlitePoolOptions = crate::pool::PoolOptions<Rqlite>;
