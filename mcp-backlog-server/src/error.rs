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

    #[error("Milestone named '{original_name}' not found in project '{project}'.")]
    MilestoneNotFoundByName {
        project: String,
        original_name: String,            // ユーザーが入力した元の名前
        suggestions: Option<Vec<String>>, // 提案するマイルストーン名のリスト
    },
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
            Error::MilestoneNotFoundByName {
                project,
                original_name,
                suggestions,
            } => {
                let mut message = format!(
                    "Milestone named '{}' not found in project '{}'.",
                    original_name, project
                );
                if let Some(suggs) = suggestions {
                    if !suggs.is_empty() {
                        message.push_str(&format!(" Did you mean one of: {:?}?", suggs));
                    }
                }
                message.push_str(&format!(" You can list all available milestones using the 'get_version_milestone_list' tool for project '{}'.", project));
                McpError::invalid_params(message, None)
            }
        }
    }
}
