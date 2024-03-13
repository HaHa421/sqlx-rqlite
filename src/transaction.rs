use futures_core::future::BoxFuture;

use crate::{Rqlite, RqliteConnection};
use sqlx_core::error::Error;
use sqlx_core::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct RqliteTransactionManager;

impl TransactionManager for RqliteTransactionManager {
    type Database = Rqlite;

    fn begin(_conn: &mut RqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "begin not supported",
            )))
        })
    }

    fn commit(_conn: &mut RqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "commit not supported",
            )))
        })
    }

    fn rollback(_conn: &mut RqliteConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "rollback not supported",
            )))
        })
    }

    fn start_rollback(_conn: &mut RqliteConnection) {
        //conn.worker.start_rollback().ok();
    }
}
