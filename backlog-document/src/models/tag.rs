use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct DocumentTag {
    pub id: u32, // Assuming ID is a numeric type
    pub name: String,
}
