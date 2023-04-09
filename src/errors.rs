use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct GraphError {
    details: String,
}

impl GraphError {
    pub fn new(msg: &str) -> GraphError {
        GraphError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GraphError {
    fn description(&self) -> &str {
        &self.details
    }
}
