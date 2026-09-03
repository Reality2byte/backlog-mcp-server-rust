use crate::models::Issue;
use serde::{Deserialize, Serialize};

/// Issue with relation type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelatedIssue {
    #[serde(flatten)]
    pub issue: Issue,
    /// Currently always "RELATES"
    #[serde(rename = "type")]
    pub relation_type: String,
}
