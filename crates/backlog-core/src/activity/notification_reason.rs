use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum NotificationReason {
    NoReason = 0,
    AssignedToIssue = 1,
    IssueCommented = 2,
    IssueCreated = 3,
    IssueUpdated = 4,
    FileAdded = 5,
    ProjectUserAdded = 6,
    // 7, 8 are reserved for future use
    Other = 9,
    AssignedToPullRequest = 10,
    CommentAddedOnPullRequest = 11,
    PullRequestAdded = 12,
    PullRequestUpdated = 13,
    DocumentCommented = 14,
    DocumentCommentReplied = 15,
    IssueMultiCreated = 16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_reason_serialization() {
        let reason = NotificationReason::IssueCommented;
        let serialized = serde_json::to_string(&reason).unwrap();
        assert_eq!(serialized, "2");
    }

    #[test]
    fn test_notification_reason_deserialization() {
        let json = "3";
        let reason: NotificationReason = serde_json::from_str(json).unwrap();
        assert_eq!(reason, NotificationReason::IssueCreated);
    }

    #[test]
    fn test_all_notification_reasons() {
        let test_cases = vec![
            (NotificationReason::NoReason, 0),
            (NotificationReason::AssignedToIssue, 1),
            (NotificationReason::IssueCommented, 2),
            (NotificationReason::IssueCreated, 3),
            (NotificationReason::IssueUpdated, 4),
            (NotificationReason::FileAdded, 5),
            (NotificationReason::ProjectUserAdded, 6),
            (NotificationReason::Other, 9),
            (NotificationReason::AssignedToPullRequest, 10),
            (NotificationReason::CommentAddedOnPullRequest, 11),
            (NotificationReason::PullRequestAdded, 12),
            (NotificationReason::PullRequestUpdated, 13),
            (NotificationReason::DocumentCommented, 14),
            (NotificationReason::DocumentCommentReplied, 15),
            (NotificationReason::IssueMultiCreated, 16),
        ];

        for (reason, expected_value) in test_cases {
            let serialized = serde_json::to_string(&reason).unwrap();
            assert_eq!(serialized, expected_value.to_string());

            let deserialized: NotificationReason = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, reason);
        }
    }
}
