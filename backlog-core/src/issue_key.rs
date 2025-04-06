use super::error::Error;
use super::ProjectKey;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::LazyLock;

static ISSUE_KEY_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([_A-Z0-9]{1,25})-([1-9][0-9]*)$").unwrap());

#[derive(Clone, Debug, PartialEq)]
pub struct IssueKey {
    project_key: ProjectKey,
    key_id: u32,
}

/// A type that identify the issue, and is unique through the space.
///
/// IssueKey must start with `ProjectKey`, follow hyphen, and follow number.
impl IssueKey {
    /// Creates a new `IssueKey` from `project_key` and `key_id`.
    ///
    /// # Panics
    /// Panics if key_id = 0.
    pub fn new(project_key: ProjectKey, key_id: u32) -> Self {
        if key_id == 0 {
            panic!("key_id should not be zero.")
        }
        IssueKey {
            project_key,
            key_id,
        }
    }
}

impl From<IssueKey> for String {
    fn from(issue_key: IssueKey) -> Self {
        issue_key.to_string()
    }
}

impl std::fmt::Display for IssueKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", &self.project_key.0, &self.key_id)
    }
}

impl FromStr for IssueKey {
    type Err = Error;

    /// Parses this string slice into `IssueKey`.
    ///
    /// # Errors
    ///
    /// Will return [`Err`] if it's not possible to parse this string slice into
    /// the `IssueKey`.
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        let cap = ISSUE_KEY_REGEXP.captures(key);
        if let Some(m) = cap {
            // safety use from_str_unchecked: the constraint of the regex ISSUE_KEY_REGEXP ensures the project_key is valid
            let project_key = ProjectKey::from_str_unchecked(&m[1]);

            // safety unwrap: the constraint of the regex ISSUE_KEY_REGEXP ensures key_id can be converted into u32 and is greater than zero
            let key_id = u32::from_str(&m[2]).unwrap();

            Ok(IssueKey::new(project_key, key_id))
        } else {
            Err(Error::InvalidIssueKey(key.to_string()))
        }
    }
}

impl Serialize for IssueKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for IssueKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[test]
fn test_issue_key_from_str() {
    assert_eq!(
        IssueKey::from_str("BLG-9"),
        Ok(IssueKey::new(ProjectKey::from_str_unchecked("BLG"), 9,))
    );
    assert_eq!(
        IssueKey::from_str("BLG-09"),
        Err(Error::InvalidIssueKey(String::from("BLG-09")))
    );
    assert_eq!(
        IssueKey::from_str("BLG9"),
        Err(Error::InvalidIssueKey(String::from("BLG9")))
    );
    assert_eq!(
        IssueKey::from_str("BLG-a9"),
        Err(Error::InvalidIssueKey(String::from("BLG-a9")))
    );
    assert_eq!(
        IssueKey::from_str("TOO_LONG_PROJECT_KEY_LN25-9999"),
        Ok(IssueKey::new(
            ProjectKey::from_str_unchecked("TOO_LONG_PROJECT_KEY_LN25"),
            9999,
        ))
    );
    assert_eq!(
        IssueKey::from_str("TOO_LONG_PROJECT_KEY_LEN26-123"),
        Err(Error::InvalidIssueKey(String::from(
            "TOO_LONG_PROJECT_KEY_LEN26-123"
        )))
    );
}

#[test]
fn test_issue_key_to_string() {
    assert_eq!(
        IssueKey::new(ProjectKey::from_str_unchecked("BLG"), 123,).to_string(),
        "BLG-123".to_string()
    );
}

#[test]
fn test_issue_key_serialize() {
    let issue_key = IssueKey::new(ProjectKey::from_str_unchecked("BLG"), 123);
    let serialized = serde_json::to_string(&issue_key).unwrap();
    assert_eq!(serialized, "\"BLG-123\"");
}

#[test]
fn test_issue_key_deserialize() {
    let issue_key: IssueKey = serde_json::from_str("\"BLG-123\"").unwrap();
    assert_eq!(
        issue_key,
        IssueKey::new(ProjectKey::from_str_unchecked("BLG"), 123)
    );

    // Test invalid issue key
    let result: Result<IssueKey, _> = serde_json::from_str("\"invalid-key\"");
    assert!(result.is_err());
}
