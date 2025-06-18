use backlog_core::identifier::{IssueTypeId, ProjectId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents an issue type in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub id: IssueTypeId,
    pub project_id: ProjectId,
    pub name: String,
    pub color: String,
    pub display_order: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}

/// Represents valid colors for issue types in Backlog.
/// These are the only colors supported by the Backlog API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum IssueTypeColor {
    #[serde(rename = "#e30000")]
    Red,
    #[serde(rename = "#990000")]
    DarkRed,
    #[serde(rename = "#934981")]
    Purple,
    #[serde(rename = "#814fbc")]
    Violet,
    #[serde(rename = "#2779ca")]
    Blue,
    #[serde(rename = "#007e9a")]
    Teal,
    #[serde(rename = "#7ea800")]
    Green,
    #[serde(rename = "#ff9200")]
    Orange,
    #[serde(rename = "#ff3265")]
    Pink,
    #[serde(rename = "#666665")]
    Gray,
}

impl IssueTypeColor {
    /// Returns the hex color code as a string slice.
    pub fn as_hex(&self) -> &'static str {
        match self {
            Self::Red => "#e30000",
            Self::DarkRed => "#990000",
            Self::Purple => "#934981",
            Self::Violet => "#814fbc",
            Self::Blue => "#2779ca",
            Self::Teal => "#007e9a",
            Self::Green => "#7ea800",
            Self::Orange => "#ff9200",
            Self::Pink => "#ff3265",
            Self::Gray => "#666665",
        }
    }

    /// Returns the human-readable name of the color.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::DarkRed => "dark-red",
            Self::Purple => "purple",
            Self::Violet => "violet",
            Self::Blue => "blue",
            Self::Teal => "teal",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Pink => "pink",
            Self::Gray => "gray",
        }
    }

    /// Returns all available issue type colors.
    pub fn all_colors() -> &'static [IssueTypeColor] {
        &[
            Self::Red,
            Self::DarkRed,
            Self::Purple,
            Self::Violet,
            Self::Blue,
            Self::Teal,
            Self::Green,
            Self::Orange,
            Self::Pink,
            Self::Gray,
        ]
    }

    /// Returns all available color names.
    pub fn all_names() -> Vec<&'static str> {
        Self::all_colors().iter().map(|c| c.name()).collect()
    }

    /// Returns all available hex codes.
    pub fn all_hex_codes() -> Vec<&'static str> {
        Self::all_colors().iter().map(|c| c.as_hex()).collect()
    }
}

impl fmt::Display for IssueTypeColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl FromStr for IssueTypeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Accept both hex codes and color names
            "#e30000" | "red" => Ok(Self::Red),
            "#990000" | "dark-red" => Ok(Self::DarkRed),
            "#934981" | "purple" => Ok(Self::Purple),
            "#814fbc" | "violet" => Ok(Self::Violet),
            "#2779ca" | "blue" => Ok(Self::Blue),
            "#007e9a" | "teal" => Ok(Self::Teal),
            "#7ea800" | "green" => Ok(Self::Green),
            "#ff9200" | "orange" => Ok(Self::Orange),
            "#ff3265" | "pink" => Ok(Self::Pink),
            "#666665" | "gray" => Ok(Self::Gray),
            _ => Err(format!(
                "Invalid issue type color: '{}'. Valid colors: {}",
                s,
                Self::all_names().join(", ")
            )),
        }
    }
}
