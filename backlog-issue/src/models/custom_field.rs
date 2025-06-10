use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a custom field associated with an issue.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    /// The ID of the custom field.
    pub id: u32,
    /// The field type ID.
    pub field_type_id: u8,
    /// The name of the custom field.
    pub name: String,
    /// The value of the custom field.
    pub value: Value,
    /// Other value associated with the custom field, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_value: Option<Value>,
}
