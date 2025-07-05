use super::Error;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::TryFrom;
use std::fmt;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use std::str::FromStr;
use std::vec::Vec;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Role {
    Admin = 1,
    User = 2,
    Reporter = 3,
    Viewer = 4,
    Guest = 5, // FIXME: classic plan only
}

impl Role {
    pub fn all() -> Vec<Role> {
        vec![
            Role::Admin,
            Role::User,
            Role::Reporter,
            Role::Viewer,
            Role::Guest,
        ]
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "reporter" => Ok(Role::Reporter),
            "viewer" => Ok(Role::Viewer),
            "guest" => Ok(Role::Guest),
            _ => Err(Error::InvalidRole(s.to_string())),
        }
    }
}

impl TryFrom<i32> for Role {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Role::Admin as i32 => Ok(Role::Admin),
            x if x == Role::User as i32 => Ok(Role::User),
            x if x == Role::Reporter as i32 => Ok(Role::Reporter),
            x if x == Role::Viewer as i32 => Ok(Role::Viewer),
            x if x == Role::Guest as i32 => Ok(Role::Guest),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
            Role::Reporter => write!(f, "reporter"),
            Role::Viewer => write!(f, "viewer"),
            Role::Guest => write!(f, "guest"),
        }
    }
}
