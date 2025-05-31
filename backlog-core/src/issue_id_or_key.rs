use crate::Identifier;
use crate::error::Error;
use crate::identifier::IssueId;
use crate::issue_key::IssueKey;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)] // To allow deserializing from either a number (ID) or string (Key)
pub enum IssueIdOrKey {
    Id(IssueId),
    Key(IssueKey),
}

impl FromStr for IssueIdOrKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse as IssueId (u32) first
        if let Ok(id_val) = u32::from_str(s) {
            if id_val > 0 {
                return Ok(IssueIdOrKey::Id(IssueId::new(id_val)));
            }
        }
        // If not a u32 or not > 0, try to parse as IssueKey
        match IssueKey::from_str(s) {
            Ok(key) => Ok(IssueIdOrKey::Key(key)),
            Err(_) => Err(Error::InvalidIssueIdOrKey(s.to_string())),
        }
    }
}

impl fmt::Display for IssueIdOrKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueIdOrKey::Id(id) => write!(f, "{}", id.value()),
            IssueIdOrKey::Key(key) => write!(f, "{}", key),
        }
    }
}

impl From<IssueId> for IssueIdOrKey {
    fn from(id: IssueId) -> Self {
        IssueIdOrKey::Id(id)
    }
}

impl From<IssueKey> for IssueIdOrKey {
    fn from(key: IssueKey) -> Self {
        IssueIdOrKey::Key(key)
    }
}

// This allows IssueIdOrKey to be easily converted to a String,
// which is useful for constructing URL paths.
impl From<IssueIdOrKey> for String {
    fn from(val: IssueIdOrKey) -> Self {
        val.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProjectKey;

    #[test]
    fn test_issue_id_or_key_from_str_id() {
        assert_eq!(
            IssueIdOrKey::from_str("123"),
            Ok(IssueIdOrKey::Id(IssueId::new(123)))
        );
    }

    #[test]
    fn test_issue_id_or_key_from_str_key() {
        let project_key = ProjectKey::from_str("BLG").unwrap();
        assert_eq!(
            IssueIdOrKey::from_str("BLG-123"),
            Ok(IssueIdOrKey::Key(IssueKey::new(project_key, 123)))
        );
    }

    #[test]
    fn test_issue_id_or_key_from_str_invalid() {
        assert_eq!(
            IssueIdOrKey::from_str("INVALID-KEY-FORMAT"),
            Err(Error::InvalidIssueIdOrKey("INVALID-KEY-FORMAT".to_string()))
        );
        assert_eq!(
            IssueIdOrKey::from_str("0"),
            Err(Error::InvalidIssueIdOrKey("0".to_string()))
        );
        assert_eq!(
            IssueIdOrKey::from_str("-123"),
            Err(Error::InvalidIssueIdOrKey("-123".to_string()))
        );
    }

    #[test]
    fn test_issue_id_or_key_display_id() {
        let id_or_key = IssueIdOrKey::Id(IssueId::new(456));
        assert_eq!(id_or_key.to_string(), "456");
    }

    #[test]
    fn test_issue_id_or_key_display_key() {
        let project_key = ProjectKey::from_str("TEST").unwrap();
        let id_or_key = IssueIdOrKey::Key(IssueKey::new(project_key, 789));
        assert_eq!(id_or_key.to_string(), "TEST-789");
    }

    #[test]
    fn test_issue_id_or_key_from_types() {
        let issue_id = IssueId::new(1);
        let id_or_key_from_id: IssueIdOrKey = issue_id.into();
        assert_eq!(id_or_key_from_id, IssueIdOrKey::Id(IssueId::new(1)));

        let project_key = ProjectKey::from_str("PROJ").unwrap();
        let issue_key = IssueKey::new(project_key, 2);
        let id_or_key_from_key: IssueIdOrKey = issue_key.clone().into(); // Clone because IssueKey might not be Copy
        assert_eq!(id_or_key_from_key, IssueIdOrKey::Key(issue_key));
    }

    #[test]
    fn test_issue_id_or_key_into_string() {
        let id_val: String = IssueIdOrKey::Id(IssueId::new(123)).into();
        assert_eq!(id_val, "123");

        let project_key = ProjectKey::from_str("KEY").unwrap();
        let key_val: String = IssueIdOrKey::Key(IssueKey::new(project_key, 456)).into();
        assert_eq!(key_val, "KEY-456");
    }

    #[test]
    fn test_serde_id() {
        let id_or_key = IssueIdOrKey::Id(IssueId::new(123));
        let serialized = serde_json::to_string(&id_or_key).unwrap();
        // Based on #[serde(untagged)] and IssueId serializing as a number
        assert_eq!(serialized, "123");
        let deserialized: IssueIdOrKey = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, id_or_key);
    }

    #[test]
    fn test_serde_key() {
        let project_key = ProjectKey::from_str_unchecked("BLG");
        let issue_key = IssueKey::new(project_key, 123);
        let id_or_key = IssueIdOrKey::Key(issue_key);
        let serialized = serde_json::to_string(&id_or_key).unwrap();
        // Based on #[serde(untagged)] and IssueKey serializing as a string
        assert_eq!(serialized, "\"BLG-123\"");
        let deserialized: IssueIdOrKey = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, id_or_key);
    }
}
