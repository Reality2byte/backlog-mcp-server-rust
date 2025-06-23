use crate::models::Comment;
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_core::{Error as CoreError, IssueIdOrKey};
use derive_builder::Builder;
use serde::Serialize;
use std::{fmt, str::FromStr};

/// Response type for getting a list of comments
pub type GetCommentListResponse = Vec<Comment>;

/// Specifies the sort order for listing comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentOrder {
    /// Sorts comments in ascending order (oldest first).
    Asc,
    /// Sorts comments in descending order (newest first).
    Desc,
}

impl fmt::Display for CommentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentOrder::Asc => write!(f, "asc"),
            CommentOrder::Desc => write!(f, "desc"),
        }
    }
}

impl FromStr for CommentOrder {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(CommentOrder::Asc),
            "desc" => Ok(CommentOrder::Desc),
            _ => Err(CoreError::InvalidParameter("CommentOrder".to_string())),
        }
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetCommentListParams {
    #[builder(setter(into))]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(default, setter(into, strip_option))]
    pub min_id: Option<u64>,
    #[builder(default, setter(into, strip_option))]
    pub max_id: Option<u64>,
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u8>,
    #[builder(default, setter(into, strip_option))]
    pub order: Option<CommentOrder>,
}

impl IntoRequest for GetCommentListParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/comments", self.issue_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        let mut params = Vec::new();

        if let Some(min_id) = self.min_id {
            params.push(("minId".to_string(), min_id.to_string()));
        }

        if let Some(max_id) = self.max_id {
            params.push(("maxId".to_string(), max_id.to_string()));
        }

        if let Some(count) = self.count {
            params.push(("count".to_string(), count.to_string()));
        }

        if let Some(order) = &self.order {
            params.push(("order".to_string(), order.to_string()));
        }

        params
    }
}
