use crate::error::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

static REPOSITORY_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,99}$").unwrap());

/// Represents Git repository name.
/// Only single-byte alphanumeric characters, underscores, hyphens, and dots can be used.
/// Only one-byte alphanumeric characters can be used as the first character.
/// The length must be 1 to 100 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryName(String);

impl FromStr for RepositoryName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.len() > 100 {
            return Err(Error::InvalidRepositoryName(s.to_string()));
        }
        if !REPOSITORY_NAME_REGEX.is_match(s) {
            return Err(Error::InvalidRepositoryName(s.to_string()));
        }

        Ok(RepositoryName(s.to_string()))
    }
}

impl fmt::Display for RepositoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
