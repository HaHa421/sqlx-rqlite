use crate::rqlite;

#[derive(Debug, Clone)]
pub struct RqliteConnectOptions {
    pub(crate) inner: rqlite::ConnectOptions,
}

pub mod connect;
mod parse;
