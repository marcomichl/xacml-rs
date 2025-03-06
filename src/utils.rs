use std::{fmt::{self, Display, Formatter, Debug}, fs};

use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

pub fn parse_xml_file<T>(path: &str) -> Result<T, XacmlError>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path)
        .map_err( |_| XacmlError::new(XacmlErrorType::IoError, format!("Error reading file: {}", path)))?;
    from_str::<T>(&content)
        .map_err( |_| XacmlError::new(XacmlErrorType::DeserializeError, format!("Error deserializing string: {}", content)))
}

#[allow(unused)]
pub fn serialize_to_xml_file<T>(data: &T, path: &str) -> Result<(), XacmlError>
where
    T: Serialize + Debug,
{
    let string = to_string(data)
        .map_err( |_| XacmlError::new(XacmlErrorType::SerializeError, format!("Error serializing data: {:?}", data)))?;
    fs::write(path, &string)
        .map_err( |_| XacmlError::new(XacmlErrorType::IoError, format!("Error writing to file: {}", path)))?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XacmlErrorType{
    IoError,
    FormatError,
    DeserializeError,
    SerializeError,
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XacmlError {
    error_type: XacmlErrorType,
    message: String,
}

impl XacmlError {
    pub fn new(error_type: XacmlErrorType, message: String) -> Self {
        XacmlError {
            error_type,
            message,
        }
    }
}

impl Display for XacmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "XacmlError: {:?} - {}", self.error_type, self.message)
    }
}



