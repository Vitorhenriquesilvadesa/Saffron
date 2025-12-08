use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(msg: impl Into<String>) -> Self {
        ParseError {
            message: msg.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError::new(s)
    }
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        ParseError::new(s)
    }
}
