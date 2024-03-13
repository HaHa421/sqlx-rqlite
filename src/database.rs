pub(crate) use sqlx_core::database::{
    Database, HasArguments, HasStatement, HasStatementCache, HasValueRef,
};

use crate::{
    connection::RqliteConnection, /*RqliteArgumentValue,*/ RqliteArguments, RqliteColumn,
    /*RqliteConnection, */ RqliteQueryResult, RqliteRow, RqliteStatement,
    RqliteTransactionManager, RqliteTypeInfo, RqliteValue, RqliteValueRef,
};

/// Rqlite database driver.
#[derive(Debug)]
pub struct Rqlite;

impl Database for Rqlite {
    type Connection = RqliteConnection;

    type TransactionManager = RqliteTransactionManager;

    type Row = RqliteRow;

    type QueryResult = RqliteQueryResult;

    type Column = RqliteColumn;

    type TypeInfo = RqliteTypeInfo;

    type Value = RqliteValue;

    const NAME: &'static str = "RQLite";

    const URL_SCHEMES: &'static [&'static str] = &["rqlite"];
}

impl<'r> HasValueRef<'r> for Rqlite {
    type Database = Rqlite;

    type ValueRef = RqliteValueRef<'r>;
}

impl<'q> HasArguments<'q> for Rqlite {
    type Database = Rqlite;

    type Arguments = RqliteArguments;

    type ArgumentBuffer = Vec<rqlite::Value>;
}

impl<'q> HasStatement<'q> for Rqlite {
    type Database = Rqlite;

    type Statement = RqliteStatement<'q>;
}

impl HasStatementCache for Rqlite {}
