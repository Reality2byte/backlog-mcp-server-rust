#[cfg(test)]
mod additional_tests {
    use crate::models::{CustomFieldInput, CustomFieldListItem, CustomFieldValue};
    use backlog_core::identifier::CustomFieldItemId;
    use chrono::NaiveDate;

    // Edge case tests for CustomFieldValue
    #[test]
    fn test_custom_field_value_empty_text() {
        let value = CustomFieldValue::Text(String::new());
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_large_numeric() {
        let value = CustomFieldValue::Numeric(f64::MAX);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, f64::MAX.to_string());
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_negative_numeric() {
        let value = CustomFieldValue::Numeric(-123.45);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "-123.45");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_zero_numeric() {
        let value = CustomFieldValue::Numeric(0.0);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "0");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_leap_year_date() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
        let value = CustomFieldValue::Date(date);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "2024-02-29");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_empty_multiple_list() {
        let value = CustomFieldValue::MultipleList {
            items: vec![],
            other_value: None,
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_empty_checkbox() {
        let value = CustomFieldValue::CheckBox(vec![]);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_single_item_multiple_list() {
        let item = CustomFieldListItem {
            id: CustomFieldItemId::new(999),
            name: "Single Item".to_string(),
        };
        let value = CustomFieldValue::MultipleList {
            items: vec![item],
            other_value: Some("Other".to_string()),
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "999");
        assert_eq!(other, Some("Other".to_string()));
    }

    #[test]
    fn test_custom_field_value_list_item_with_empty_name() {
        let item = CustomFieldListItem {
            id: CustomFieldItemId::new(123),
            name: String::new(),
        };
        let value = CustomFieldValue::SingleList {
            item,
            other_value: None,
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "123");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_radio_with_empty_other() {
        let item = CustomFieldListItem {
            id: CustomFieldItemId::new(456),
            name: "Radio Option".to_string(),
        };
        let value = CustomFieldValue::Radio {
            item,
            other_value: Some(String::new()),
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "456");
        assert_eq!(other, Some(String::new()));
    }

    // Edge case tests for CustomFieldInput
    #[test]
    fn test_custom_field_input_unicode_text() {
        let input = CustomFieldInput::Text("テスト🎉 Unicode".to_string());
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "テスト🎉 Unicode");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_multiline_textarea() {
        let input = CustomFieldInput::TextArea("Line 1\nLine 2\nLine 3".to_string());
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "Line 1\nLine 2\nLine 3");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_min_date() {
        let date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let input = CustomFieldInput::Date(date);
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "1970-01-01");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_max_date() {
        let date = NaiveDate::from_ymd_opt(9999, 12, 31).unwrap();
        let input = CustomFieldInput::Date(date);
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "9999-12-31");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_large_id_list() {
        let ids = vec![
            CustomFieldItemId::new(u32::MAX),
            CustomFieldItemId::new(u32::MAX - 1),
            CustomFieldItemId::new(u32::MAX - 2),
        ];
        let input = CustomFieldInput::MultipleList {
            ids,
            other_value: None,
        };
        let (form_value, other) = input.to_form_value();
        assert_eq!(
            form_value,
            format!("{},{},{}", u32::MAX, u32::MAX - 1, u32::MAX - 2)
        );
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_single_checkbox_item() {
        let input = CustomFieldInput::CheckBox(vec![CustomFieldItemId::new(42)]);
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "42");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_radio_id_zero() {
        let input = CustomFieldInput::Radio {
            id: CustomFieldItemId::new(0),
            other_value: Some("Zero ID".to_string()),
        };
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "0");
        assert_eq!(other, Some("Zero ID".to_string()));
    }

    #[test]
    fn test_custom_field_value_clone() {
        let original = CustomFieldValue::Text("Test".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_custom_field_input_clone() {
        let original = CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_custom_field_list_item_clone() {
        let original = CustomFieldListItem {
            id: CustomFieldItemId::new(123),
            name: "Item".to_string(),
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
