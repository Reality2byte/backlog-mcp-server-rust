use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid space key: {0}")]
    InvalidSpaceKey(String),

    #[error("Invalid project key: {0}")]
    InvalidProjectKey(String),

    #[error("Invalid issue key: {0}")]
    InvalidIssueKey(String),
}


pub type Result<T> = std::result::Result<T, Error>;
