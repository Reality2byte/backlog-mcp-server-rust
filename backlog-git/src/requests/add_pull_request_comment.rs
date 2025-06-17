use backlog_api_core::Error as ApiError;
use derive_builder::Builder;

/// Parameters for adding a comment to a pull request.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddPullRequestCommentParams {
    /// The content of the comment.
    pub content: String,
    /// List of user IDs to notify about this comment.
    #[builder(default)]
    pub notified_user_ids: Option<Vec<u32>>,
}

impl AddPullRequestCommentParams {
    /// Creates a new instance with only the required content field.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            notified_user_ids: None,
        }
    }
}

// Convert AddPullRequestCommentParams to vector of pairs for form encoding
// because RequestBuilder.form() requires proper handling of array parameters like notifiedUserId[]
impl From<&AddPullRequestCommentParams> for Vec<(String, String)> {
    fn from(params: &AddPullRequestCommentParams) -> Self {
        let mut seq = Vec::new();

        // Add content parameter
        seq.push(("content".to_string(), params.content.clone()));

        // Add notified user IDs as separate entries with "notifiedUserId[]" key
        if let Some(user_ids) = &params.notified_user_ids {
            user_ids
                .iter()
                .for_each(|id| seq.push(("notifiedUserId[]".to_string(), id.to_string())));
        }

        seq
    }
}