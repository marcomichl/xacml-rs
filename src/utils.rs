use std::{any::Any, fmt::{self, Debug, Display, Formatter}, fs};

use derive_builder::UninitializedFieldError;
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

static LOG_LEVEL: LogLevel = LogLevel::DEBUG;
/// Namespace used for custom URNs (e.g. attributes, categories, Rule / Policy IDs)
/// Using RFC6761 compliant namespace for anonymized review of the code
/// Will later be changed to an owned namepsace
pub static URN_NAMESPACE: &str = "example.com";

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

/// Returns a URN-style string with 
/// urn:{NAMESPACE}:xacml:{s}
pub fn create_urn(s: &str) -> String {
    format!("urn:{}:xacml:{}", URN_NAMESPACE, s)
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
    if level >= LOG_LEVEL{    // Later: use a config parameter here
        println!("{}", msg)
    }
}
