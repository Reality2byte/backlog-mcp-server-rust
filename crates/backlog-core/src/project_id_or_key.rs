use crate::{
    Error, ProjectKey,
    identifier::{Identifier, ProjectId},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectIdOrKey {
    Id(ProjectId),
    Key(ProjectKey),
    EitherIdOrKey(ProjectId, ProjectKey),
}

impl FromStr for ProjectIdOrKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (u32::from_str(s), ProjectKey::from_str(s)) {
            (Ok(id), Ok(key)) if id > 0 => Ok(Self::EitherIdOrKey(ProjectId::new(id), key)),
            (Ok(id), Err(_)) if id > 0 => Ok(Self::Id(ProjectId::new(id))),
            (Err(_), Ok(key)) => Ok(Self::Key(key)),
            _ => Err(Error::InvalidProjectIdOrKey(s.to_string())),
        }
    }
}

impl std::fmt::Display for ProjectIdOrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectIdOrKey::Id(id) => write!(f, "{}", id.value()),
            ProjectIdOrKey::Key(key) => write!(f, "{}", key),
            ProjectIdOrKey::EitherIdOrKey(id, _) => write!(f, "{}", id.value()),
        }
    }
}

impl From<ProjectId> for ProjectIdOrKey {
    fn from(id: ProjectId) -> Self {
        ProjectIdOrKey::Id(id)
    }
}

impl From<ProjectKey> for ProjectIdOrKey {
    fn from(key: ProjectKey) -> Self {
        ProjectIdOrKey::Key(key)
    }
}
