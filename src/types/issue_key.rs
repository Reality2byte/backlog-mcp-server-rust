use super::error::Error;
use super::ProjectKey;
use regex::Regex;
use std::num::NonZero;
use std::str::FromStr;
use std::sync::LazyLock;

static ISSUE_KEY_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([_A-Z0-9]{1,25})-([1-9][0-9]*)$").unwrap());

#[derive(Debug, PartialEq)]
pub struct IssueKey {
    project_key: ProjectKey,
    key_id: NonZero<u32>,
}

/// A type that identify the issue, and is unique through the space.
///
/// IssueKey must start with `ProjectKey`, follow hyphen, and follow number.
impl IssueKey {
    /// Creates a new `IssueKey` from `project_key` and `key_id`.
    ///
    /// # Panics
    /// Panics if key_id <= 0.
    pub fn new(project_key: ProjectKey, key_id: NonZero<u32>) -> Self {
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

            // safety unwrap: the constraint of the regex ISSUE_KEY_REGEXP ensures key_id can be converted into u32
            let key_id = u32::from_str(&m[2]).unwrap();

            // safety unwrap: the constraint of the regex ISSUE_KEY_REGEXP ensures key_id is greater than zero
            let key_id = NonZero::<u32>::new(key_id).unwrap();

            Ok(IssueKey::new(project_key, key_id))
        } else {
            Err(Error::InvalidIssueKey(key.to_string()))
        }
    }
}

#[test]
fn test_issue_key_from_str() {
    assert_eq!(
        IssueKey::from_str("BLG-9"),
        Ok(IssueKey::new(
            ProjectKey::from_str_unchecked("BLG"),
            NonZero::new(9).unwrap()
        ))
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
            NonZero::new(9999).unwrap()
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
        IssueKey::new(
            ProjectKey::from_str_unchecked("BLG"),
            NonZero::new(123).unwrap()
        )
        .to_string(),
        "BLG-123".to_string()
    );
}
