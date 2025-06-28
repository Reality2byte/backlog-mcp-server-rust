use backlog_core::identifier::CustomFieldId;
use backlog_issue::models::CustomFieldInput;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomFieldError {
    #[error("Invalid custom field format: {0}")]
    InvalidFormat(String),
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    #[error("Invalid numeric value: {0}")]
    InvalidNumeric(String),
    #[error("Failed to read JSON file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
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
                let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    .map_err(|_| CustomFieldError::InvalidDate(value.clone()))?;
                Ok(CustomFieldInput::Date(date))
            }
            CustomFieldSpec::SingleList { id, other_value } => Ok(CustomFieldInput::SingleList {
                id: *id,
                other_value: other_value.clone(),
            }),
            CustomFieldSpec::MultipleList { ids, other_value } => {
                Ok(CustomFieldInput::MultipleList {
                    ids: ids.clone(),
                    other_value: other_value.clone(),
                })
            }
            CustomFieldSpec::CheckBox { ids } => Ok(CustomFieldInput::CheckBox(ids.clone())),
            CustomFieldSpec::Radio { id, other_value } => Ok(CustomFieldInput::Radio {
                id: *id,
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
        return Err(CustomFieldError::InvalidFormat(
            "Expected format: id:type:value[:other]".to_string(),
        ));
    }

    let id = parts[0]
        .parse::<u32>()
        .map_err(|_| CustomFieldError::InvalidFormat(format!("Invalid field ID: {}", parts[0])))?;

    let field_type = parts[1];
    let value = parts[2];
    let other_value = parts.get(3).map(|s| s.to_string());

    let input = match field_type {
        "text" => CustomFieldInput::Text(value.to_string()),
        "textarea" => CustomFieldInput::TextArea(value.to_string()),
        "numeric" => {
            let num = value
                .parse::<f64>()
                .map_err(|_| CustomFieldError::InvalidNumeric(value.to_string()))?;
            CustomFieldInput::Numeric(num)
        }
        "date" => {
            let date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .map_err(|_| CustomFieldError::InvalidDate(value.to_string()))?;
            CustomFieldInput::Date(date)
        }
        "single_list" => {
            let id = value.parse::<u32>().map_err(|_| {
                CustomFieldError::InvalidFormat(format!("Invalid list ID: {value}"))
            })?;
            CustomFieldInput::SingleList { id, other_value }
        }
        "multiple_list" => {
            let ids: Result<Vec<u32>, _> = value
                .split(',')
                .map(|id| id.trim().parse::<u32>())
                .collect();
            let ids = ids.map_err(|_| {
                CustomFieldError::InvalidFormat(format!("Invalid list IDs: {value}"))
            })?;
            CustomFieldInput::MultipleList { ids, other_value }
        }
        "checkbox" => {
            let ids: Result<Vec<u32>, _> = value
                .split(',')
                .map(|id| id.trim().parse::<u32>())
                .collect();
            let ids = ids.map_err(|_| {
                CustomFieldError::InvalidFormat(format!("Invalid checkbox IDs: {value}"))
            })?;
            CustomFieldInput::CheckBox(ids)
        }
        "radio" => {
            let id = value.parse::<u32>().map_err(|_| {
                CustomFieldError::InvalidFormat(format!("Invalid radio ID: {value}"))
            })?;
            CustomFieldInput::Radio { id, other_value }
        }
        _ => {
            return Err(CustomFieldError::InvalidFormat(format!(
                "Unknown field type: {field_type}"
            )));
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
    path: &Path,
) -> Result<HashMap<CustomFieldId, CustomFieldInput>, CustomFieldError> {
    let content = fs::read_to_string(path)?;
    let specs: HashMap<String, CustomFieldSpec> = serde_json::from_str(&content)?;

    let mut result = HashMap::new();
    for (id_str, spec) in specs {
        let id = id_str
            .parse::<u32>()
            .map_err(|_| CustomFieldError::InvalidFormat(format!("Invalid field ID: {id_str}")))?;
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
                assert_eq!(id, 100);
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
                assert_eq!(ids, vec![100, 200, 300]);
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
