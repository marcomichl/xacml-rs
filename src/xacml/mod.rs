pub mod structs;
pub mod enums;

pub mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct XacmlError {
        details: String
    }

    impl XacmlError {
        pub fn new(msg: &str) -> XacmlError {
            XacmlError{details: msg.to_string()}
        }
    }

    impl fmt::Display for XacmlError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error: {}", self.details)
        }
    }

    impl Error for XacmlError {
        fn description(&self) -> &str {
            &self.details
        }
    }
}