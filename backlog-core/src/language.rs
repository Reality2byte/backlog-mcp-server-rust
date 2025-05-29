use serde::{Deserialize, Serialize};
use schemars::JsonSchema; // Added for JsonSchema

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)] // Added JsonSchema
pub enum Language {
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "en")]
    English,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Japanese => write!(f, "ja"),
            Self::English => write!(f, "en"),
        }
    }
}
