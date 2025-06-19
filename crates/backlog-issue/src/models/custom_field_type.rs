use crate::models::{InitialDate, ListItem};
use backlog_core::{
    Date,
    identifier::{CustomFieldId, IssueTypeId, ProjectId},
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CustomFieldType {
    pub id: CustomFieldId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub use_issue_type: bool,
    pub applicable_issue_types: Vec<IssueTypeId>,
    pub display_order: i64,
    pub settings: CustomFieldSettings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CustomFieldSettings {
    Text,
    TextArea,
    Numeric(NumericSettings),
    Date(DateSettings),
    SingleList(ListSettings),
    MultipleList(ListSettings),
    Checkbox(ListSettings),
    Radio(ListSettings),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NumericSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DateSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<InitialDate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListSettings {
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub allow_add_item: bool,
    #[serde(default)]
    pub allow_input: bool,
}

impl Serialize for CustomFieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("projectId", &self.project_id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("required", &self.required)?;
        map.serialize_entry("useIssueType", &self.use_issue_type)?;
        map.serialize_entry("applicableIssueTypes", &self.applicable_issue_types)?;
        map.serialize_entry("displayOrder", &self.display_order)?;
        let type_id = match self.settings {
            CustomFieldSettings::Text => 1,
            CustomFieldSettings::TextArea => 2,
            CustomFieldSettings::Numeric(_) => 3,
            CustomFieldSettings::Date(_) => 4,
            CustomFieldSettings::SingleList(_) => 5,
            CustomFieldSettings::MultipleList(_) => 6,
            CustomFieldSettings::Checkbox(_) => 7,
            CustomFieldSettings::Radio(_) => 8,
        };
        map.serialize_entry("typeId", &type_id)?;
        match &self.settings {
            CustomFieldSettings::Numeric(settings) => {
                let val = serde_json::to_value(settings).map_err(serde::ser::Error::custom)?;
                if let serde_json::Value::Object(m) = val {
                    for (k, v) in m {
                        if !v.is_null() {
                            map.serialize_entry(&k, &v)?;
                        }
                    }
                }
            }
            CustomFieldSettings::Date(settings) => {
                let val = serde_json::to_value(settings).map_err(serde::ser::Error::custom)?;
                if let serde_json::Value::Object(m) = val {
                    for (k, v) in m {
                        if !v.is_null() {
                            map.serialize_entry(&k, &v)?;
                        }
                    }
                }
            }
            CustomFieldSettings::SingleList(settings)
            | CustomFieldSettings::MultipleList(settings)
            | CustomFieldSettings::Checkbox(settings)
            | CustomFieldSettings::Radio(settings) => {
                let val = serde_json::to_value(settings).map_err(serde::ser::Error::custom)?;
                if let serde_json::Value::Object(m) = val {
                    for (k, v) in m {
                        if !v.is_null() {
                            map.serialize_entry(&k, &v)?;
                        }
                    }
                }
            }
            _ => {}
        }
        map.end()
    }
}

// A custom `Deserialize` implementation is required for `CustomFieldType` due to two main complexities in the Backlog API's JSON response:
// 1.  **Numeric Enum Tag**: The `typeId` field, which discriminates between different custom field types, is a number (e.g., `3`) rather than a string. Serde's `#[serde(tag = "...")]` attribute expects a string tag by default, which causes automatic derivation to fail.
// 2.  **Ambiguous Field Types**: Field names like `min` and `max` are reused across different custom field types but have different data types (e.g., a number for numeric fields, a date-string for date fields).
// To handle this, we use the "Raw Struct" pattern: first, we deserialize into a temporary `RawCustomFieldType` struct where ambiguous fields are parsed into `serde_json::Value`. Then, we manually match on `type_id` to parse the `Value` fields into their correct, final types.
impl<'de> Deserialize<'de> for CustomFieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawCustomFieldType {
            id: CustomFieldId,
            project_id: ProjectId,
            type_id: i64,
            name: String,
            #[serde(default)]
            description: String,
            required: bool,
            use_issue_type: bool,
            applicable_issue_types: Vec<IssueTypeId>,
            display_order: i64,
            // Numeric and Date
            min: Option<serde_json::Value>,
            max: Option<serde_json::Value>,
            // Numeric
            initial_value: Option<f64>,
            unit: Option<String>,
            // Date
            initial_date: Option<InitialDate>,
            // List
            items: Option<Vec<ListItem>>,
            allow_add_item: Option<bool>,
            allow_input: Option<bool>,
        }

        let raw = RawCustomFieldType::deserialize(deserializer)?;

        let settings = match raw.type_id {
            1 => CustomFieldSettings::Text,
            2 => CustomFieldSettings::TextArea,
            3 => {
                let min = raw
                    .min
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(serde::de::Error::custom)?
                    .flatten();
                let max = raw
                    .max
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(serde::de::Error::custom)?
                    .flatten();
                CustomFieldSettings::Numeric(NumericSettings {
                    min,
                    max,
                    initial_value: raw.initial_value,
                    unit: raw.unit,
                })
            }
            4 => {
                let min = raw
                    .min
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(serde::de::Error::custom)?
                    .flatten();
                let max = raw
                    .max
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(serde::de::Error::custom)?
                    .flatten();
                CustomFieldSettings::Date(DateSettings {
                    min,
                    max,
                    initial_date: raw.initial_date,
                })
            }
            5 => CustomFieldSettings::SingleList(ListSettings {
                items: raw.items.unwrap_or_default(),
                allow_add_item: raw.allow_add_item.unwrap_or_default(),
                allow_input: raw.allow_input.unwrap_or_default(),
            }),
            6 => CustomFieldSettings::MultipleList(ListSettings {
                items: raw.items.unwrap_or_default(),
                allow_add_item: raw.allow_add_item.unwrap_or_default(),
                allow_input: raw.allow_input.unwrap_or_default(),
            }),
            7 => CustomFieldSettings::Checkbox(ListSettings {
                items: raw.items.unwrap_or_default(),
                allow_add_item: raw.allow_add_item.unwrap_or_default(),
                allow_input: raw.allow_input.unwrap_or_default(),
            }),
            8 => CustomFieldSettings::Radio(ListSettings {
                items: raw.items.unwrap_or_default(),
                allow_add_item: raw.allow_add_item.unwrap_or_default(),
                allow_input: raw.allow_input.unwrap_or_default(),
            }),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "unknown typeId: {}",
                    raw.type_id
                )));
            }
        };

        Ok(CustomFieldType {
            id: raw.id,
            project_id: raw.project_id,
            name: raw.name,
            description: raw.description,
            required: raw.required,
            use_issue_type: raw.use_issue_type,
            applicable_issue_types: raw.applicable_issue_types,
            display_order: raw.display_order,
            settings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::Date;
    use chrono::NaiveDate;
    use serde_json;

    #[test]
    fn test_deserialize_text_field() {
        let json = r#"{
            "id": 978974,
            "projectId": 615400,
            "typeId": 1,
            "name": "文字列フィールド（全）",
            "description": "これは文字列型のカスタムフィールドです。",
            "required": true,
            "useIssueType": true,
            "applicableIssueTypes": [ 2536436 ],
            "displayOrder": 2147483646
        }"#;
        let field: CustomFieldType = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, 978974.into());
        assert!(matches!(field.settings, CustomFieldSettings::Text));
    }

    #[test]
    fn test_deserialize_text_area_field() {
        let json = r#"{
            "id": 978977,
            "projectId": 615400,
            "typeId": 2,
            "name": "文章フィールド（全）",
            "description": "これは文章型のカスタムフィールドです。",
            "required": true,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 2147483646
        }"#;
        let field: CustomFieldType = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, 978977.into());
        assert!(matches!(field.settings, CustomFieldSettings::TextArea));
    }

    #[test]
    fn test_deserialize_numeric_field() {
        let json = r#"{
            "id": 978978,
            "projectId": 615400,
            "typeId": 3,
            "name": "数値フィールド（全）",
            "description": "これは数値型のカスタムフィールドです。",
            "required": true,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 2147483646,
            "min": 0,
            "max": 100,
            "initialValue": 50,
            "unit": "pt"
        }"#;
        let field: CustomFieldType = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, 978978.into());
        if let CustomFieldSettings::Numeric(settings) = field.settings {
            assert_eq!(settings.min, Some(0.0));
            assert_eq!(settings.max, Some(100.0));
            assert_eq!(settings.initial_value, Some(50.0));
            assert_eq!(settings.unit, Some("pt".to_string()));
        } else {
            panic!("Wrong settings type");
        }
    }

    #[test]
    fn test_deserialize_date_field() {
        let json = r#"{
            "id": 978979,
            "projectId": 615400,
            "typeId": 4,
            "name": "日付フィールド（全）",
            "description": "これは日付型のカスタムフィールドです。",
            "required": true,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 2147483646,
            "min": "2025-01-01T00:00:00Z",
            "max": "2025-12-31T00:00:00Z",
            "initialDate": {
              "id": 1,
              "date": "2025-12-24T00:00:00Z"
            }
        }"#;
        let field: CustomFieldType = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, 978979.into());
        if let CustomFieldSettings::Date(settings) = field.settings {
            assert_eq!(
                settings.min,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()))
            );
            assert_eq!(
                settings.max,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()))
            );
            let initial_date = settings.initial_date.unwrap();
            assert_eq!(initial_date.id, 1);
            assert_eq!(
                initial_date.date,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap()))
            );
        } else {
            panic!("Wrong settings type");
        }
    }

    #[test]
    fn test_deserialize_list_field() {
        let json = r#"{
            "id": 978982,
            "projectId": 615400,
            "typeId": 5,
            "name": "単一リスト（全）",
            "description": "これは単一リストです。",
            "required": true,
            "useIssueType": true,
            "applicableIssueTypes": [ 2536437 ],
            "displayOrder": 2147483646,
            "allowAddItem": true,
            "items": [
              { "id": 1, "name": "A", "displayOrder": 0 },
              { "id": 2, "name": "B", "displayOrder": 1 },
              { "id": 3, "name": "C", "displayOrder": 2 }
            ]
        }"#;
        let field: CustomFieldType = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, 978982.into());
        if let CustomFieldSettings::SingleList(settings) = field.settings {
            assert_eq!(settings.items.len(), 3);
            assert!(settings.allow_add_item);
        } else {
            panic!("Wrong settings type");
        }
    }

    #[test]
    fn test_serialize_date_field() {
        let field = CustomFieldType {
            id: 978979.into(),
            project_id: 615400.into(),
            name: "日付フィールド（全）".to_string(),
            description: "これは日付型のカスタムフィールドです。".to_string(),
            required: true,
            use_issue_type: false,
            applicable_issue_types: vec![],
            display_order: 2147483646,
            settings: CustomFieldSettings::Date(DateSettings {
                min: Some(Date::from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())),
                max: Some(Date::from(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())),
                initial_date: Some(InitialDate {
                    id: 1,
                    shift: None,
                    date: Some(Date::from(NaiveDate::from_ymd_opt(2025, 12, 24).unwrap())),
                }),
            }),
        };
        let json = serde_json::to_string(&field).unwrap();
        let expected = r#"{"id":978979,"projectId":615400,"name":"日付フィールド（全）","description":"これは日付型のカスタムフィールドです。","required":true,"useIssueType":false,"applicableIssueTypes":[],"displayOrder":2147483646,"typeId":4,"min":"2025-01-01T00:00:00Z","max":"2025-12-31T00:00:00Z","initialDate":{"id":1,"shift":null,"date":"2025-12-24T00:00:00Z"}}"#;
        let expected_val: serde_json::Value = serde_json::from_str(expected).unwrap();
        let actual_val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(actual_val, expected_val);
    }
}
