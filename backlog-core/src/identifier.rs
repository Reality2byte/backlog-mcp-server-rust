use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

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
            #[cfg_attr(feature = "schemars", derive(JsonSchema))]
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

            impl std::hash::Hash for $type_name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.0.hash(state);
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
    RepositoryId,
    PullRequestId
);

// PrNumber is u64, so it's defined manually instead of via the macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PrNumber(pub u64);

impl PrNumber {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Identifier for PrNumber {
    type Id = u64;
    fn value(&self) -> Self::Id {
        self.0
    }
}

impl From<u64> for PrNumber {
    fn from(value: u64) -> Self {
        PrNumber(value)
    }
}

impl std::fmt::Display for PrNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PrNumber {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u64>() {
            Ok(val) => Ok(PrNumber(val)),
            Err(_) => Err(crate::error::Error::InvalidParameter(format!(
                "Failed to parse PrNumber from string '{}': expected a u64 integer.",
                s
            ))),
        }
    }
}
