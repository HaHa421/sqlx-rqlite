#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::error::Error as StdError;
pub type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

/// Sqlite types
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
    #[cfg(feature = "chrono")]
    Date,
}

pub trait FromJsonValue {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError>
    where
        Self: Sized;
}

impl FromJsonValue for f64 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for i8 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for i16 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for i32 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for i64 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}

impl FromJsonValue for i128 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for u8 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for u16 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for u32 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}

impl FromJsonValue for u64 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
impl FromJsonValue for u128 {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}
#[cfg(feature = "chrono")]
impl FromJsonValue for chrono::DateTime<chrono::Utc> {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        let s: String = serde_json::from_value(value)?;
        let from_database: DateTime<Utc> = DateTime::parse_from_rfc3339(&s)?.into();
        Ok(from_database)
    }
}

impl FromJsonValue for String {
    fn from_json_value(value: Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value)?)
    }
}

pub trait FromJsonValueRef {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError>
    where
        Self: Sized;
}

impl FromJsonValueRef for f64 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for i8 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for i16 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for i32 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for i64 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

impl FromJsonValueRef for i128 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for u8 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for u16 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for u32 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

impl FromJsonValueRef for u64 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
impl FromJsonValueRef for u128 {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}
#[cfg(feature = "chrono")]
impl FromJsonValueRef for chrono::DateTime<chrono::Utc> {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        let s: String = serde_json::from_value(value.clone())?;
        let from_database: DateTime<Utc> = DateTime::parse_from_rfc3339(&s)?.into();
        Ok(from_database)
    }
}

impl FromJsonValueRef for String {
    fn from_json_value_ref(value: &Value) -> Result<Self, BoxDynError> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

pub trait ToJsonValue {
    fn to_json_value(&self) -> Result<Value, serde_json::Error>;
}
/*
impl<T : !ToJsonValue> ToJsonValue for T {
  fn to_json_value(&self)->Result<Value,serde_json::Error> {
    Ok(Value::none())
  }
}
*/

impl ToJsonValue for f64 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for i8 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for i16 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for i32 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

impl ToJsonValue for i64 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for i128 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

impl ToJsonValue for u8 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for u16 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for u32 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

impl ToJsonValue for u64 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl ToJsonValue for u128 {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
#[cfg(feature = "chrono")]
impl ToJsonValue for chrono::DateTime<chrono::Utc> {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        let s: String = self.to_rfc3339();
        serde_json::to_value(&s)
    }
}

impl ToJsonValue for String {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
impl<'a> ToJsonValue for &'a str {
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}
/// Parse vector of json values
pub fn parse_vec_types<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<Type>>, D::Error> {
    let s: Option<Vec<String>> = Option::deserialize(deserializer)?;
    match s {
        Some(s_vec) => {
            let mut vec: Vec<Type> = Vec::with_capacity(s_vec.len());
            for i in 0..s_vec.len() {
                vec.push(match get_type(s_vec[i].as_str()) {
                    Ok(val) => val,
                    Err(_) => {
                        return Err(de::Error::custom(format!(
                            "Unknewn sqlite type {}",
                            s_vec[i]
                        )))
                    }
                });
            }
            Ok(Some(vec))
        }
        None => Ok(None),
    }
}

/// Intepret type
fn get_type(_type: &str) -> Result<Type, ()> {
    Ok(match _type {
        "" => Type::Null,
        "integer" => Type::Integer,
        "real" => Type::Real,
        "text" => Type::Text,
        "blob" => Type::Blob,
        #[cfg(feature = "chrono")]
        "datetime" => Type::Date,
        _ => return Err(()),
    })
}

/// Specify parameters for parameterized statements.
///
/// Warning: Using raw queries may introduce vulnerabilities.
///
/// Named parameters is not supported by rqlite.
/// ```
/// conn.execute("SELECT * FROM foo where name = ?", par!("fiona"))?;
/// ```
#[macro_export]
macro_rules! par {
    ( $( $x:expr ),* ) => {
        {
            let mut vec: Vec<$crate::Value> = Vec::new();
            $(
                //vec.push($crate::to_value($x)?);
                vec.push($x.to_json_value()?);
            )*
            vec
        }
    };
}
