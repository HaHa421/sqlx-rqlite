//! An asynchronous client library for rqlite.
//!
//! This library uses tokio for sockets and hyper to handle http requests.
//!
//! Currently there is no transaction support.
//! ```
//! use rqlite::ConnectOptions;
//!
//! let mut conn = ConnectOptions::new("my.node.local", 4001)
//!     .scheme(Scheme::HTTPS)
//!     .user("root")
//!     .pass("root")
//!		.connect().await?;
//!	conn.execute("SELECT * FROM foo where id = ?;", par!(1)).await?;
//! ```

mod connect;
mod cursor;
mod error;
mod row;
pub mod types;

pub use connect::{ConnectOptions, Connection, Node, Scheme};
pub use error::RqliteError;
pub use serde_json::{to_value, Value};
