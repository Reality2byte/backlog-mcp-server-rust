use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
pub enum ParentChildCondition {
    All = 0,
    ExcludeChildIssue = 1,
    ChildIssue = 2,
    NeitherParentIssueNorChildIssue = 3,
    ParentIssue = 4,
}
