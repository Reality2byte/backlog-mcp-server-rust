use super::Error;
use serde_repr::{Deserialize_repr, Serialize_repr};
use schemars::JsonSchema; // Added for JsonSchema
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use std::vec::Vec;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr, JsonSchema)] // Added JsonSchema
pub enum Role {
    Admin = 1,
    Developer = 2,
    Reporter = 3,
    Viewer = 4,
    GuestReporter = 5,
    GuestViewer = 6,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        *self == Role::Admin
    }
    pub fn is_developer(&self) -> bool {
        matches!(*self, Role::Admin | Role::Developer)
    }
    pub fn is_viewer(&self) -> bool {
        matches!(*self, Role::Viewer | Role::GuestViewer)
    }
    pub fn is_guest(&self) -> bool {
        matches!(*self, Role::GuestReporter | Role::GuestViewer)
    }

    pub fn all() -> Vec<Role> {
        vec![
            Role::Admin,
            Role::Developer,
            Role::Reporter,
            Role::Viewer,
            Role::GuestReporter,
            Role::GuestViewer,
        ]
    }
    pub fn developers() -> Vec<Role> {
        vec![Role::Admin, Role::Developer]
    }
    pub fn reporters() -> Vec<Role> {
        vec![
            Role::Admin,
            Role::Developer,
            Role::Reporter,
            Role::GuestReporter,
        ]
    }
    pub fn hosts() -> Vec<Role> {
        vec![Role::Admin, Role::Developer, Role::Reporter, Role::Viewer]
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "normal-user" => Ok(Role::Developer),
            "reporter" => Ok(Role::Reporter),
            "viewer" => Ok(Role::Viewer),
            "guest-reporter" => Ok(Role::GuestReporter),
            "guest-viewer" => Ok(Role::GuestViewer),
            _ => Err(Error::InvalidRole(s.to_string())),
        }
    }
}

impl TryFrom<i32> for Role {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == Role::Admin as i32 => Ok(Role::Admin),
            x if x == Role::Developer as i32 => Ok(Role::Developer),
            x if x == Role::Reporter as i32 => Ok(Role::Reporter),
            x if x == Role::Viewer as i32 => Ok(Role::Viewer),
            x if x == Role::GuestReporter as i32 => Ok(Role::GuestReporter),
            x if x == Role::GuestViewer as i32 => Ok(Role::GuestViewer),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            Role::Admin => write!(f, "admin"),
            Role::Developer => write!(f, "normal-user"),
            Role::Reporter => write!(f, "reporter"),
            Role::Viewer => write!(f, "viewer"),
            Role::GuestReporter => write!(f, "guest-reporter"),
            Role::GuestViewer => write!(f, "guest-viewer"),
        }
    }
}
