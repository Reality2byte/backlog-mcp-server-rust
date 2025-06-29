use backlog_core::identifier::{CustomFieldId, CustomFieldItemId};
use backlog_issue::models::CustomFieldInput;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomFieldError {
    #[error(
        "Invalid custom field format: '{input}'\nExpected format: id:type:value[:other]\nExamples:\n  - 1:text:Sample text\n  - 2:numeric:123.45\n  - 3:date:2024-06-24\n  - 4:single_list:100:Other description\n  - 5:multiple_list:100,200,300\n  - 6:checkbox:10,20,30\n  - 7:radio:400:Other value"
    )]
    InvalidFormat { input: String },

    #[error(
        "Invalid custom field ID: '{input}' is not a valid number.\nID must be a positive integer."
    )]
    InvalidId { input: String },

    #[error(
        "Unknown custom field type: '{field_type}'.\nValid types are:\n  - text\n  - textarea\n  - numeric\n  - date\n  - single_list\n  - multiple_list\n  - checkbox\n  - radio"
    )]
    UnknownFieldType { field_type: String },

    #[error("Invalid numeric value: '{value}' is not a valid number.\nExamples: 123, 45.67, -89.0")]
    InvalidNumericValue { value: String },

    #[error(
        "Invalid date format: '{value}' is not a valid date.\nExpected format: YYYY-MM-DD\nExample: 2024-06-24"
    )]
    InvalidDateFormat { value: String },

    #[error(
        "Missing ID for list field.\nsingle_list requires an ID.\nFormat: id:single_list:list_id[:other_value]\nExample: 1:single_list:100:Other description"
    )]
    MissingListId,

    #[error(
        "Invalid list IDs: '{value}' contains invalid ID.\nAll IDs must be positive integers separated by commas.\nExample: 1:multiple_list:100,200,300"
    )]
    InvalidListIds { value: String },

    #[error(
        "Custom fields file not found: '{path}'.\nMake sure the file exists and the path is correct."
    )]
    FileNotFound { path: String },

    #[error("Invalid JSON format in '{path}': {error}.\nSee CUSTOM_FIELDS_USAGE.md for examples.")]
    InvalidJson { path: String, error: String },

    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
}

/// Custom field specification in JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CustomFieldSpec {
    Text {
        value: String,
    },
    TextArea {
        value: String,
    },
    Numeric {
        value: f64,
    },
    Date {
        value: String,
    },
    SingleList {
        id: u32,
        other_value: Option<String>,
    },
    MultipleList {
        ids: Vec<u32>,
        other_value: Option<String>,
    },
    CheckBox {
        ids: Vec<u32>,
    },
    Radio {
        id: u32,
        other_value: Option<String>,
    },
}

impl CustomFieldSpec {
    /// Convert to CustomFieldInput
    pub fn to_input(&self) -> Result<CustomFieldInput, CustomFieldError> {
        match self {
            CustomFieldSpec::Text { value } => Ok(CustomFieldInput::Text(value.clone())),
            CustomFieldSpec::TextArea { value } => Ok(CustomFieldInput::TextArea(value.clone())),
            CustomFieldSpec::Numeric { value } => Ok(CustomFieldInput::Numeric(*value)),
            CustomFieldSpec::Date { value } => {
                let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| {
                    CustomFieldError::InvalidDateFormat {
                        value: value.clone(),
                    }
                })?;
                Ok(CustomFieldInput::Date(date))
            }
            CustomFieldSpec::SingleList { id, other_value } => Ok(CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(*id),
                other_value: other_value.clone(),
            }),
            CustomFieldSpec::MultipleList { ids, other_value } => {
                Ok(CustomFieldInput::MultipleList {
                    ids: ids.iter().map(|&id| CustomFieldItemId::new(id)).collect(),
                    other_value: other_value.clone(),
                })
            }
            CustomFieldSpec::CheckBox { ids } => Ok(CustomFieldInput::CheckBox(
                ids.iter().map(|&id| CustomFieldItemId::new(id)).collect(),
            )),
            CustomFieldSpec::Radio { id, other_value } => Ok(CustomFieldInput::Radio {
                id: CustomFieldItemId::new(*id),
                other_value: other_value.clone(),
            }),
        }
    }
}

/// Parse custom fields from command line argument
/// Format: "id:type:value" or "id:type:value:other"
/// Examples:
/// - "1:text:Sample text"
/// - "2:numeric:123.45"
/// - "3:date:2024-06-24"
/// - "4:single_list:100:Other description"
/// - "5:multiple_list:100,200,300"
/// - "6:checkbox:10,20,30"
/// - "7:radio:400:Other value"
pub fn parse_custom_field_arg(
    arg: &str,
) -> Result<(CustomFieldId, CustomFieldInput), CustomFieldError> {
    let parts: Vec<&str> = arg.splitn(4, ':').collect();

    if parts.len() < 3 {
        return Err(CustomFieldError::InvalidFormat {
            input: arg.to_string(),
        });
    }

    let id = parts[0]
        .parse::<u32>()
        .map_err(|_| CustomFieldError::InvalidId {
            input: parts[0].to_string(),
        })?;

    let field_type = parts[1];
    let value = parts[2];
    let other_value = parts.get(3).map(|s| s.to_string());

    let input = match field_type {
        "text" => CustomFieldInput::Text(value.to_string()),
        "textarea" => CustomFieldInput::TextArea(value.to_string()),
        "numeric" => {
            let num = value
                .parse::<f64>()
                .map_err(|_| CustomFieldError::InvalidNumericValue {
                    value: value.to_string(),
                })?;
            CustomFieldInput::Numeric(num)
        }
        "date" => {
            let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| {
                CustomFieldError::InvalidDateFormat {
                    value: value.to_string(),
                }
            })?;
            CustomFieldInput::Date(date)
        }
        "single_list" => {
            if value.is_empty() {
                return Err(CustomFieldError::MissingListId);
            }
            let id = value
                .parse::<u32>()
                .map_err(|_| CustomFieldError::InvalidListIds {
                    value: value.to_string(),
                })?;
            CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(id),
                other_value,
            }
        }
        "multiple_list" => {
            let ids: Result<Vec<u32>, _> = value
                .split(',')
                .map(|id| id.trim().parse::<u32>())
                .collect();
            let ids = ids.map_err(|_| CustomFieldError::InvalidListIds {
                value: value.to_string(),
            })?;
            CustomFieldInput::MultipleList {
                ids: ids.into_iter().map(CustomFieldItemId::new).collect(),
                other_value,
            }
        }
        "checkbox" => {
            let ids: Result<Vec<u32>, _> = value
                .split(',')
                .map(|id| id.trim().parse::<u32>())
                .collect();
            let ids = ids.map_err(|_| CustomFieldError::InvalidListIds {
                value: value.to_string(),
            })?;
            CustomFieldInput::CheckBox(ids.into_iter().map(CustomFieldItemId::new).collect())
        }
        "radio" => {
            if value.is_empty() {
                return Err(CustomFieldError::MissingListId);
            }
            let id = value
                .parse::<u32>()
                .map_err(|_| CustomFieldError::InvalidListIds {
                    value: value.to_string(),
                })?;
            CustomFieldInput::Radio {
                id: CustomFieldItemId::new(id),
                other_value,
            }
        }
        _ => {
            return Err(CustomFieldError::UnknownFieldType {
                field_type: field_type.to_string(),
            });
        }
    };

    Ok((CustomFieldId::new(id), input))
}

/// Parse custom fields from JSON file
/// Expected format:
/// {
///   "1": {"type": "text", "value": "Sample text"},
///   "2": {"type": "numeric", "value": 123.45},
///   "3": {"type": "date", "value": "2024-06-24"},
///   "4": {"type": "single_list", "id": 100, "other_value": "Other"},
///   "5": {"type": "multiple_list", "ids": [100, 200], "other_value": "Other"},
///   "6": {"type": "checkbox", "ids": [10, 20, 30]},
///   "7": {"type": "radio", "id": 400, "other_value": "Other"}
/// }
pub fn parse_custom_fields_json(
    path: &str,
) -> Result<HashMap<CustomFieldId, CustomFieldInput>, CustomFieldError> {
    let content = fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            CustomFieldError::FileNotFound {
                path: path.to_string(),
            }
        } else {
            CustomFieldError::FileReadError(e)
        }
    })?;

    let specs: HashMap<String, CustomFieldSpec> =
        serde_json::from_str(&content).map_err(|e| CustomFieldError::InvalidJson {
            path: path.to_string(),
            error: e.to_string(),
        })?;

    let mut result = HashMap::new();
    for (id_str, spec) in specs {
        let id = id_str
            .parse::<u32>()
            .map_err(|_| CustomFieldError::InvalidId {
                input: id_str.clone(),
            })?;
        let input = spec.to_input()?;
        result.insert(CustomFieldId::new(id), input);
    }

    Ok(result)
}

/// Parse multiple custom field arguments
pub fn parse_custom_field_args(
    args: &[String],
) -> Result<HashMap<CustomFieldId, CustomFieldInput>, CustomFieldError> {
    let mut result = HashMap::new();

    for arg in args {
        let (id, input) = parse_custom_field_arg(arg)?;
        result.insert(id, input);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_parse_text_field() {
        let result = parse_custom_field_arg("1:text:Sample text");
        assert!(result.is_ok());
        let (id, input) = result.unwrap();
        assert_eq!(id.value(), 1);
        match input {
            CustomFieldInput::Text(value) => assert_eq!(value, "Sample text"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_parse_numeric_field() {
        let result = parse_custom_field_arg("2:numeric:123.45");
        assert!(result.is_ok());
        let (id, input) = result.unwrap();
        assert_eq!(id.value(), 2);
        match input {
            CustomFieldInput::Numeric(value) => assert_eq!(value, 123.45),
            _ => panic!("Expected Numeric variant"),
        }
    }

    #[test]
    fn test_parse_date_field() {
        let result = parse_custom_field_arg("3:date:2024-06-24");
        assert!(result.is_ok());
        let (id, input) = result.unwrap();
        assert_eq!(id.value(), 3);
        match input {
            CustomFieldInput::Date(date) => {
                assert_eq!(date, NaiveDate::from_ymd_opt(2024, 6, 24).unwrap());
            }
            _ => panic!("Expected Date variant"),
        }
    }

    #[test]
    fn test_parse_single_list_with_other() {
        let result = parse_custom_field_arg("4:single_list:100:Other description");
        assert!(result.is_ok());
        let (id, input) = result.unwrap();
        assert_eq!(id.value(), 4);
        match input {
            CustomFieldInput::SingleList { id, other_value } => {
                assert_eq!(id, CustomFieldItemId::new(100));
                assert_eq!(other_value, Some("Other description".to_string()));
            }
            _ => panic!("Expected SingleList variant"),
        }
    }

    #[test]
    fn test_parse_multiple_list() {
        let result = parse_custom_field_arg("5:multiple_list:100,200,300");
        assert!(result.is_ok());
        let (id, input) = result.unwrap();
        assert_eq!(id.value(), 5);
        match input {
            CustomFieldInput::MultipleList { ids, other_value } => {
                assert_eq!(
                    ids,
                    vec![
                        CustomFieldItemId::new(100),
                        CustomFieldItemId::new(200),
                        CustomFieldItemId::new(300)
                    ]
                );
                assert_eq!(other_value, None);
            }
            _ => panic!("Expected MultipleList variant"),
        }
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_custom_field_arg("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Expected format"));
    }

    #[test]
    fn test_parse_invalid_numeric() {
        let result = parse_custom_field_arg("1:numeric:not_a_number");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid numeric value")
        );
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_custom_field_arg("1:date:2024/06/24");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid date format")
        );
    }
}
