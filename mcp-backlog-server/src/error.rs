use backlog_core::Error as CoreError;
use backlog_api_client::ApiError;
use rmcp::Error as McpError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("API error: {0}")]
    CoreError(CoreError),

    #[error("API error: {0}")]
    ApiError(ApiError),

    #[error("Parameter error: {0}")]
    ParameterError(String),

    #[error("Server error: {0}")]
    ServerError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<CoreError> for Error {
    fn from(err: CoreError) -> Self {
        Error::CoreError(err)
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::ApiError(err)
    }
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        match err {
            Error::CoreError(err) => McpError::invalid_request(err.to_string(), None),
            Error::ServerError(msg) => McpError::internal_error(msg, None),
            Error::ParameterError(msg) => McpError::invalid_params(msg, None),
            Error::ApiError(error) => McpError::invalid_request(error.to_string(), None),
        }
    }
}
