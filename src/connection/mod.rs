use std::fmt::{self, Debug, Formatter};

use futures_core::future::BoxFuture;
//use futures_util::FutureExt;
use futures_util::future;
pub(crate) use sqlx_core::connection::*;
use sqlx_core::transaction::Transaction;

use crate::error::Error;
use crate::options::RqliteConnectOptions;
use crate::Rqlite;

mod establish;
mod executor;

pub struct RqliteConnection {
    inner: rqlite::Connection,
}

impl Debug for RqliteConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RqliteConnection").finish()
    }
}

impl Connection for RqliteConnection {
    type Database = Rqlite;

    type Options = RqliteConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move { Ok(()) })
    }

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        //self.stream.wait_until_ready().boxed()
        Box::pin(future::ok(()))
    }

    fn cached_statements_size(&self) -> usize {
        //self.cache_statement.len()
        0
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            /*
            while let Some((statement_id, _)) = self.cache_statement.remove_lru() {
                self.stream
                    .send_packet(StmtClose {
                        statement: statement_id,
                    })
                    .await?;
            }
            */
            Ok(())
        })
    }

    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        //!self.stream.write_buffer().is_empty()
        false
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self)
    }

    fn shrink_buffers(&mut self) {
        //self.stream.shrink_buffers();
    }
}
