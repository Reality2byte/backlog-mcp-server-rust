use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum NotificationReason {
    AssignedToIssue = 1,
    IssueCommented = 2,
    IssueCreated = 3,
    IssueUpdated = 4,
    FileAdded = 5,
    ProjectUserAdded = 6,
    Other = 9,
    AssignedToPullRequest = 10,
    CommentAddedOnPullRequest = 11,
    PullRequestAdded = 12,
    PullRequestUpdated = 13,
}
