use backlog_api_core::Error as ApiError;
use derive_builder::Builder;

/// Parameters for updating a pull request comment.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdatePullRequestCommentParams {
    /// The updated content of the comment.
    pub content: String,
}

impl UpdatePullRequestCommentParams {
    /// Creates a new instance with the required content field.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

// Convert UpdatePullRequestCommentParams to vector of pairs for form encoding
impl From<&UpdatePullRequestCommentParams> for Vec<(String, String)> {
    fn from(params: &UpdatePullRequestCommentParams) -> Self {
        vec![("content".to_string(), params.content.clone())]
    }
}
