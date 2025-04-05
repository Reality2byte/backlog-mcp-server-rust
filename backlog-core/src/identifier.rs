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
        )*
    };
}

impl_identifier!(
    ProjectId,
    UserId,
    IssueId,
    SpaceId,
    MilestoneId,
    CommentId,
    AttachmentId,
    PullRequestId
);
