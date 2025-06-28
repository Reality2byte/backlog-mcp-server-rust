#[cfg(test)]
mod custom_field_error_messages_tests {
    use blg::custom_fields::{CustomFieldError, parse_custom_field_arg, parse_custom_fields_json};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_custom_field_invalid_format_helpful_message() {
        // Missing type and value
        let result = parse_custom_field_arg("123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, CustomFieldError::InvalidFormat { .. }));
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidFormat { input } => {
                assert_eq!(input, "123");
                assert!(msg.contains("Invalid custom field format: '123'"));
                assert!(msg.contains("Expected format: id:type:value[:other]"));
                assert!(msg.contains("Examples:"));
                assert!(msg.contains("1:text:Sample text"));
                assert!(msg.contains("2:numeric:123.45"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_custom_field_invalid_id_helpful_message() {
        let result = parse_custom_field_arg("abc:text:value");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidId { input } => {
                assert_eq!(input, "abc");
                assert!(msg.contains("Invalid custom field ID"));
                assert!(msg.contains("'abc' is not a valid number"));
                assert!(msg.contains("ID must be a positive integer"));
            }
            _ => panic!("Expected InvalidId error"),
        }
    }

    #[test]
    fn test_parse_custom_field_unknown_type_helpful_message() {
        let result = parse_custom_field_arg("1:unknown:value");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::UnknownFieldType { field_type } => {
                assert_eq!(field_type, "unknown");
                assert!(msg.contains("Unknown custom field type: 'unknown'"));
                assert!(msg.contains("Valid types are:"));
                assert!(msg.contains("text"));
                assert!(msg.contains("textarea"));
                assert!(msg.contains("numeric"));
                assert!(msg.contains("date"));
                assert!(msg.contains("single_list"));
                assert!(msg.contains("multiple_list"));
                assert!(msg.contains("checkbox"));
                assert!(msg.contains("radio"));
            }
            _ => panic!("Expected UnknownFieldType error"),
        }
    }

    #[test]
    fn test_parse_custom_field_invalid_numeric_helpful_message() {
        let result = parse_custom_field_arg("1:numeric:not-a-number");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidNumericValue { value } => {
                assert_eq!(value, "not-a-number");
                assert!(msg.contains("Invalid numeric value"));
                assert!(msg.contains("'not-a-number' is not a valid number"));
                assert!(msg.contains("Examples: 123, 45.67, -89.0"));
            }
            _ => panic!("Expected InvalidNumericValue error"),
        }
    }

    #[test]
    fn test_parse_custom_field_invalid_date_helpful_message() {
        let result = parse_custom_field_arg("1:date:2024-13-45");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidDateFormat { value } => {
                assert_eq!(value, "2024-13-45");
                assert!(msg.contains("Invalid date format"));
                assert!(msg.contains("'2024-13-45' is not a valid date"));
                assert!(msg.contains("Expected format: YYYY-MM-DD"));
                assert!(msg.contains("Example: 2024-06-24"));
            }
            _ => panic!("Expected InvalidDateFormat error"),
        }
    }

    #[test]
    fn test_parse_custom_field_missing_list_id_helpful_message() {
        let result = parse_custom_field_arg("1:single_list:");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::MissingListId => {
                assert!(msg.contains("Missing ID for list field"));
                assert!(msg.contains("single_list requires an ID"));
                assert!(msg.contains("Format: id:single_list:list_id[:other_value]"));
                assert!(msg.contains("Example: 1:single_list:100:Other description"));
            }
            _ => panic!("Expected MissingListId error"),
        }
    }

    #[test]
    fn test_parse_custom_field_invalid_list_ids_helpful_message() {
        let result = parse_custom_field_arg("1:multiple_list:100,abc,200");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidListIds { value } => {
                assert_eq!(value, "100,abc,200");
                assert!(msg.contains("Invalid list IDs"));
                assert!(msg.contains("'100,abc,200' contains invalid ID"));
                assert!(msg.contains("All IDs must be positive integers"));
                assert!(msg.contains("Example: 1:multiple_list:100,200,300"));
            }
            _ => panic!("Expected InvalidListIds error"),
        }
    }

    #[test]
    fn test_parse_json_file_not_found_helpful_message() {
        let result = parse_custom_fields_json("/nonexistent/file.json");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::FileNotFound { path } => {
                assert_eq!(path, "/nonexistent/file.json");
                assert!(msg.contains("Custom fields file not found"));
                assert!(msg.contains("'/nonexistent/file.json'"));
                assert!(msg.contains("Make sure the file exists and the path is correct"));
            }
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_parse_json_invalid_json_helpful_message() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("invalid.json");
        fs::write(&json_path, "{ invalid json }").unwrap();

        let result = parse_custom_fields_json(json_path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        match err {
            CustomFieldError::InvalidJson { .. } => {
                assert!(msg.contains("Invalid JSON format"));
                assert!(msg.contains("key must be a string"));
                assert!(msg.contains("See CUSTOM_FIELDS_USAGE.md for examples"));
            }
            _ => panic!("Expected InvalidJson error"),
        }
    }

    // TODO: Implement proper JSON validation to generate MissingJsonField errors
    // Currently, serde handles this validation and produces InvalidJson errors instead

    #[test]
    fn test_parse_json_invalid_type_in_json_helpful_message() {
        let temp_dir = TempDir::new().unwrap();
        let json_path = temp_dir.path().join("invalid_type.json");
        fs::write(
            &json_path,
            r#"{"1": {"type": "invalid_type", "value": "test"}}"#,
        )
        .unwrap();

        let result = parse_custom_fields_json(json_path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        // serde tag validation produces InvalidJson error, not UnknownFieldType
        match err {
            CustomFieldError::InvalidJson { .. } => {
                assert!(msg.contains("Invalid JSON format"));
                assert!(msg.contains("See CUSTOM_FIELDS_USAGE.md for examples"));
            }
            _ => panic!("Expected InvalidJson error"),
        }
    }

    #[test]
    fn test_error_display_formatting() {
        // Test that all error messages are properly formatted
        let errors = vec![
            CustomFieldError::InvalidFormat {
                input: "bad".to_string(),
            },
            CustomFieldError::InvalidId {
                input: "abc".to_string(),
            },
            CustomFieldError::UnknownFieldType {
                field_type: "foo".to_string(),
            },
            CustomFieldError::InvalidNumericValue {
                value: "nan".to_string(),
            },
            CustomFieldError::InvalidDateFormat {
                value: "bad-date".to_string(),
            },
            CustomFieldError::MissingListId,
            CustomFieldError::InvalidListIds {
                value: "1,a,3".to_string(),
            },
            CustomFieldError::FileNotFound {
                path: "/path/to/file".to_string(),
            },
            CustomFieldError::InvalidJson {
                path: "/path/to/json".to_string(),
                error: "test error".to_string(),
            },
        ];

        for error in errors {
            let msg = error.to_string();
            // Check if first line ends with proper punctuation or the message contains Examples/Expected
            let first_line = msg.lines().next().unwrap_or("");
            assert!(
                first_line.ends_with('.')
                    || first_line.ends_with(':')
                    || first_line.ends_with('!')
                    || msg.contains("Examples:")
                    || msg.contains("Expected format:"),
                "Error message should have proper formatting: {msg}"
            );
            // All messages should be helpful (contain examples or instructions)
            assert!(
                msg.contains("Example")
                    || msg.contains("Expected")
                    || msg.contains("Valid")
                    || msg.contains("Make sure")
                    || msg.contains("must")
                    || msg.contains("See"),
                "Error message should be helpful: {msg}"
            );
        }
    }
}
