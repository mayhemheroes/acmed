//! Opaque error shim. parse_duration only constructs an Error from a String/&str
//! (format!(...).into()) and returns it; it never inspects the contents, so this faithfully
//! models the parser Ok/Err contract while exercising the real parser logic.
pub mod error {
    use std::fmt;

    #[derive(Clone, Debug)]
    pub struct Error {
        pub message: String,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl From<&str> for Error {
        fn from(error: &str) -> Self {
            Error { message: error.to_string() }
        }
    }

    impl From<String> for Error {
        fn from(error: String) -> Self {
            Error { message: error }
        }
    }
}
