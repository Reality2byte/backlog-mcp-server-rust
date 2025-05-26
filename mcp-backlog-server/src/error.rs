use backlog_api_client::ApiError;
use backlog_core::Error as CoreError;
use rmcp::Error as McpError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("API error: {0}")]
    Core(CoreError),

    #[error("API error: {0}")]
    Api(ApiError),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error("Server error: {0}")]
    Server(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<CoreError> for Error {
    fn from(err: CoreError) -> Self {
        Error::Core(err)
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::Api(err)
    }
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        match err {
            Error::Core(err) => McpError::invalid_request(err.to_string(), None),
            Error::Server(msg) => McpError::internal_error(msg, None),
            Error::Parameter(msg) => McpError::invalid_params(msg, None),
            Error::Api(error) => McpError::invalid_request(error.to_string(), None),
        }
    }
}
