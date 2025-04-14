use serde::{Deserialize, Serialize};

pub trait Identifier {
    type Id;
    fn value(&self) -> Self::Id;
}

pub trait Entity {
    type Identifier: Identifier;
    fn id(&self) -> &Self::Identifier;
}

macro_rules! impl_identifier {
    ($($type_name:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
            pub struct $type_name(pub u32);

            impl $type_name {
                pub fn new(value: u32) -> Self {
                    Self(value)
                }
            }

            impl Identifier for $type_name {
                type Id = u32;
                fn value(&self) -> Self::Id {
                    self.0
                }
            }

            impl From<u32> for $type_name {
                fn from(value: u32) -> Self {
                    $type_name(value)
                }
            }

            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )*
    };
}

impl_identifier!(
    ProjectId,
    UserId,
    IssueId,
    SpaceId,
    MilestoneId,
    CategoryId,
    IssueTypeId,
    StatusId,
    PriorityId,
    ResolutionId,
    CommentId,
    AttachmentId,
    PullRequestId
);
