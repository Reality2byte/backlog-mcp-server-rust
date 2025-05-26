use super::error::Error;
use crate::identifier::Identifier;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::LazyLock};

static DOCUMENT_ID_REGEXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9a-f]{32}$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Identifier for DocumentId {
    type Id = String;
    fn value(&self) -> Self::Id {
        self.0.clone()
    }
}

impl From<String> for DocumentId {
    fn from(value: String) -> Self {
        DocumentId(value)
    }
}

impl FromStr for DocumentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = DOCUMENT_ID_REGEXP.captures(s);
        if cap.is_some() {
            Ok(DocumentId(s.to_string()))
        } else {
            Err(Error::InvalidDocumentId(s.to_string()))
        }
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::hash::Hash for DocumentId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
