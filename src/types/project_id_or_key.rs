use super::{ids::ProjectId, Error, ProjectKey};
use std::str::FromStr;

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
