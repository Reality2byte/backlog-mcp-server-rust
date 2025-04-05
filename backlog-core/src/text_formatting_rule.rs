use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextFormattingRule {
    Backlog,
    Markdown,
}

impl std::fmt::Display for TextFormattingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Backlog => write!(f, "backlog"),
            Self::Markdown => write!(f, "markdown"),
        }
    }
}
