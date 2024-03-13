use std::{error::Error, fmt};

/// Handle all errors for Connection and Cursor explicitly
#[derive(Debug)]
pub enum RqliteError {
    /// Error in authentification
    AuthError,
    /// SQL syntax error or missing parameters
    SqlError(String),
    /// Error serialising/deserializing data.
    ///
    /// This shouldn't happen, if it does.
    ///
    /// Some changes need to be made to this crate
    DataSer(String),
    /// Connection error, either there is some networking error.
    ///
    /// Or one of the ends closed connection
    Connection(String),
}

impl Error for RqliteError {}

impl fmt::Display for RqliteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RqliteError::AuthError => write!(f, "Error authentificating"),
            RqliteError::SqlError(v) => write!(f, "Error executing sql query: {}", v),
            RqliteError::DataSer(v) => write!(f, "Error with json: {}", v),
            RqliteError::Connection(v) => write!(f, "Connection error: {}", v),
        }
    }
}
