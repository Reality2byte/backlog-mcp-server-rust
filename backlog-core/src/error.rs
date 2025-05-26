use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid space key: {0}")]
    InvalidSpaceKey(String),

    #[error("Invalid project key: {0}")]
    InvalidProjectKey(String),

    #[error("Invalid project id or key: {0}")]
    InvalidProjectIdOrKey(String),

    #[error("Invalid issue id or key: {0}")]
    InvalidIssueIdOrKey(String),

    #[error("Invalid issue key: {0}")]
    InvalidIssueKey(String),

    #[error("Invalid role type: {0}")]
    InvalidRole(String),

    #[error("Invalid document id: {0}")]
    InvalidDocumentId(String),
}

pub type Result<T> = std::result::Result<T, Error>;
