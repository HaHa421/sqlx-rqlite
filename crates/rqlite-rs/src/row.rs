//use serde::de::DeserializeOwned;
use serde_json::Value;
//use std::error::Error;
use crate::types::BoxDynError;
use crate::types::FromJsonValue;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Row {
    pub row: Vec<Value>,
}

/// SQL row
impl Row {
    pub fn column_count(&self) -> usize {
        self.row.len()
    }
    pub(crate) fn new(row: Vec<Value>) -> Row {
        Row { row }
    }

    /// Get n element in row
    /// Return error if element cannot be formatted
    pub fn get<T: FromJsonValue /*DeserializeOwned*/>(&self, id: usize) -> Result<T, BoxDynError> {
        if id >= self.row.len() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Row element with id {} doesn't exist", id),
            )));
        }

        let val: T = T::from_json_value(self.row[id].clone())?;
        Ok(val)
    }
}
