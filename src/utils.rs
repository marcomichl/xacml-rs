use std::{any::Any, fmt::{self, Debug, Display, Formatter}, fs};

use derive_builder::UninitializedFieldError;
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

static log_level: LogLevel = LogLevel::DEBUG;

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
    BuildError,
    GeneralError,
    ProcessingError,
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


impl<E> From<E> for XacmlError 
where
    E: std::error::Error + 'static,
{
    fn from(e: E) -> Self {
        let err_any = &e as &dyn Any; // Cast to `Any` for type checking
        if err_any.is::<UninitializedFieldError>() {
            return XacmlError::new(XacmlErrorType::BuildError, format!("Error! Uninitialized Field: {:?}", e));
        }
        XacmlError::new(XacmlErrorType::GeneralError, format!("Error: {}", e.to_string()))
    }
}


impl Display for XacmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "XacmlError: {:?} - {}", self.error_type, self.message)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    DEBUG,
    WARN,
    ERROR
}

pub fn log(level: LogLevel, msg: &str) {
    if level >= log_level{    // Later: use a config parameter here
        println!("{}", msg)
    }
}
