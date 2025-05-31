use backlog_api_client::ProjectIdOrKey;
use backlog_api_client::{ApiError, CoreError};
use rmcp::Error as McpError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("API error: {0}")]
    Api(ApiError),

    #[error("Parameter error: {0}")]
    Parameter(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Milestone named '{original_name}' not found in project '{project_id_or_key}'.")]
    MilestoneNotFoundByName {
        project_id_or_key: ProjectIdOrKey,
        original_name: String,
        suggestions: Option<Vec<String>>,
    },

    #[error("Nothing to update. Please provide a summary and/or a description.")]
    NothingToUpdate,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Self {
        Error::Api(err)
    }
}

impl From<CoreError> for Error {
    fn from(err: CoreError) -> Self {
        Error::Api(err.into())
    }
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        match err {
            Error::Server(msg) => McpError::internal_error(msg, None),
            Error::Parameter(msg) => McpError::invalid_params(msg, None),
            Error::Api(error) => McpError::invalid_request(error.to_string(), None),
            Error::MilestoneNotFoundByName {
                project_id_or_key,
                original_name,
                suggestions,
            } => {
                let mut message = format!(
                    "Milestone named '{}' not found in project '{}'.",
                    original_name, project_id_or_key
                );
                if let Some(suggs) = suggestions {
                    if !suggs.is_empty() {
                        message.push_str(&format!(" Did you mean one of: {:?}?", suggs));
                    }
                }
                message.push_str(&format!(" You can list all available milestones using the 'get_version_milestone_list' tool for project '{}'.", project_id_or_key));
                McpError::invalid_params(message, None)
            }
            Error::NothingToUpdate => McpError::invalid_params(err.to_string(), None),
        }
    }
}
