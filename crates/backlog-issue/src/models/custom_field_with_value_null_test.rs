#[cfg(test)]
mod null_value_tests {
    use crate::models::CustomFieldWithValue;

    #[test]
    fn test_deserialize_null_text_field() {
        let json = r#"{
            "id": 1,
            "fieldTypeId": 1,
            "name": "Null Text",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        // This should fail with current implementation
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Expected string for Text field"));
    }

    #[test]
    fn test_deserialize_null_numeric_field() {
        let json = r#"{
            "id": 2,
            "fieldTypeId": 3,
            "name": "Null Number",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Expected number for Numeric field"));
    }

    #[test]
    fn test_deserialize_null_date_field() {
        let json = r#"{
            "id": 3,
            "fieldTypeId": 4,
            "name": "Null Date",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Expected string for Date field"));
    }

    #[test]
    fn test_deserialize_null_single_list_field() {
        let json = r#"{
            "id": 4,
            "fieldTypeId": 5,
            "name": "Null Single List",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to parse SingleList item"));
    }

    #[test]
    fn test_deserialize_null_multiple_list_field() {
        let json = r#"{
            "id": 5,
            "fieldTypeId": 6,
            "name": "Null Multiple List",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to parse MultipleList items"));
    }

    #[test]
    fn test_deserialize_null_checkbox_field() {
        let json = r#"{
            "id": 6,
            "fieldTypeId": 7,
            "name": "Null Checkbox",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to parse CheckBox items"));
    }

    #[test]
    fn test_deserialize_null_radio_field() {
        let json = r#"{
            "id": 7,
            "fieldTypeId": 8,
            "name": "Null Radio",
            "value": null
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Failed to parse Radio item"));
    }

    #[test]
    fn test_deserialize_empty_string_date_field() {
        let json = r#"{
            "id": 8,
            "fieldTypeId": 4,
            "name": "Empty Date",
            "value": ""
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Invalid date format"));
    }

    #[test]
    fn test_deserialize_empty_string_numeric_field() {
        let json = r#"{
            "id": 9,
            "fieldTypeId": 3,
            "name": "Empty Number",
            "value": ""
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Expected number for Numeric field"));
    }
}
