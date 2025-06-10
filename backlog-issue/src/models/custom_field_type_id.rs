use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CustomFieldTypeId {
    SingleText = 1,
    MultipleText = 2,
    Numeric = 3,
    Date = 4,
    SingleList = 5,
    MultipleList = 6,
    CheckBox = 7,
    Radio = 8,
}
