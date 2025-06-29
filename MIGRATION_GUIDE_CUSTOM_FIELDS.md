# Custom Fields Migration Guide

This guide helps you migrate from the previous `serde_json::Value`-based custom field implementation to the new type-safe custom field API.

## Overview

The new custom field API provides:
- Type safety with dedicated enums for each field type
- Better error handling with descriptive messages
- Simplified CLI integration
- Automatic form parameter serialization

## Migration Steps

### 1. Update Import Statements

**Before:**
```rust
use serde_json::{json, Value};
use std::collections::HashMap;
```

**After:**
```rust
use backlog_issue::models::{CustomFieldInput, CustomFieldValue, CustomFieldTypeId};
use backlog_core::identifier::CustomFieldId;
use std::collections::HashMap;
```

### 2. Update Custom Field Creation

**Before:**
```rust
let mut custom_fields: HashMap<CustomFieldId, Value> = HashMap::new();

// Text field
custom_fields.insert(CustomFieldId::new(1), json!("Sample text"));

// Numeric field
custom_fields.insert(CustomFieldId::new(2), json!(42.5));

// Date field
custom_fields.insert(CustomFieldId::new(3), json!("2024-06-24"));

// Single list
custom_fields.insert(CustomFieldId::new(4), json!({
    "id": 100,
    "otherValue": "Additional info"
}));

// Multiple list
custom_fields.insert(CustomFieldId::new(5), json!([
    {"id": 200},
    {"id": 201}
]));
```

**After:**
```rust
let mut custom_fields: HashMap<CustomFieldId, CustomFieldInput> = HashMap::new();

// Text field
custom_fields.insert(
    CustomFieldId::new(1),
    CustomFieldInput::Text("Sample text".to_string())
);

// Numeric field
custom_fields.insert(
    CustomFieldId::new(2),
    CustomFieldInput::Numeric(42.5)
);

// Date field
custom_fields.insert(
    CustomFieldId::new(3),
    CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap())
);

// Single list
custom_fields.insert(
    CustomFieldId::new(4),
    CustomFieldInput::SingleList {
        id: 100,
        other_value: Some("Additional info".to_string()),
    }
);

// Multiple list
custom_fields.insert(
    CustomFieldId::new(5),
    CustomFieldInput::MultipleList {
        ids: vec![200, 201],
        other_value: None,
    }
);
```

### 3. Update Custom Field Reading

**Before:**
```rust
for field in &issue.custom_fields {
    match field.value {
        Value::String(ref s) => println!("{}: {}", field.name, s),
        Value::Number(ref n) => println!("{}: {}", field.name, n),
        Value::Array(ref arr) => {
            // Complex logic to extract list items
        },
        Value::Object(ref obj) => {
            // Complex logic to extract single selection
        },
        _ => println!("{}: {:?}", field.name, field.value),
    }
}
```

**After:**
```rust
for field in &issue.custom_fields {
    print!("{} (Type: {:?}): ", field.name, field.field_type_id);
    
    match field.field_type_id {
        CustomFieldTypeId::Text | CustomFieldTypeId::TextArea => {
            if let Some(text) = field.value.as_str() {
                println!("{}", text);
            }
        }
        CustomFieldTypeId::Numeric => {
            if let Some(num) = field.value.as_f64() {
                println!("{}", num);
            }
        }
        CustomFieldTypeId::Date => {
            if let Some(date) = field.value.as_str() {
                println!("{}", date);
            }
        }
        CustomFieldTypeId::SingleList | CustomFieldTypeId::Radio => {
            if let Some(obj) = field.value.as_object() {
                if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                    print!("{}", name);
                    if let Some(other) = &field.other_value {
                        if let Some(other_str) = other.as_str() {
                            print!(" (Other: {})", other_str);
                        }
                    }
                    println!();
                }
            }
        }
        CustomFieldTypeId::MultipleList | CustomFieldTypeId::CheckBox => {
            if let Some(arr) = field.value.as_array() {
                let names: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_object()
                        .and_then(|obj| obj.get("name"))
                        .and_then(|name| name.as_str())
                        .map(|s| s.to_string()))
                    .collect();
                println!("[{}]", names.join(", "));
            }
        }
    }
}
```

### 4. CLI Usage Migration

**Before (using raw JSON):**
```bash
# Complex JSON string required
blg issue add -p PROJECT \
  --custom-fields '{"1":"Text value","2":42.5,"3":"2024-06-24","4":{"id":100}}'
```

**After (using type-safe format):**
```bash
# Simple, type-safe format
blg issue add -p PROJECT \
  --custom-field "1:text:Text value" \
  --custom-field "2:numeric:42.5" \
  --custom-field "3:date:2024-06-24" \
  --custom-field "4:single_list:100:Other description"

# Or using JSON file
blg issue add -p PROJECT --custom-fields-json custom_fields.json
```

### 5. Error Handling Migration

**Before:**
```rust
match serde_json::from_str::<HashMap<CustomFieldId, Value>>(&json_str) {
    Ok(fields) => {
        // Process fields
    }
    Err(e) => {
        eprintln!("Invalid JSON: {}", e);
    }
}
```

**After:**
```rust
use cli::custom_fields::{parse_custom_field, CustomFieldError};

match parse_custom_field(&field_str) {
    Ok((id, input)) => {
        custom_fields.insert(id, input);
    }
    Err(e) => {
        // Detailed error messages with examples
        eprintln!("{}", e);
    }
}
```

## Common Patterns

### Converting Date Strings
```rust
// Before
json!("2024-06-24")

// After
use chrono::NaiveDate;
CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap())

// From string
let date_str = "2024-06-24";
let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
CustomFieldInput::Date(date)
```

### Handling Optional Other Values
```rust
// Single list with other value
CustomFieldInput::SingleList {
    id: 100,
    other_value: Some("Custom description".to_string()),
}

// Multiple list without other value
CustomFieldInput::MultipleList {
    ids: vec![200, 201, 202],
    other_value: None,
}
```

### JSON File Format
```json
{
  "1": {
    "type": "text",
    "value": "Sample text"
  },
  "2": {
    "type": "numeric",
    "value": 42.5
  },
  "3": {
    "type": "date",
    "value": "2024-06-24"
  },
  "4": {
    "type": "single_list",
    "id": 100,
    "other_value": "Additional info"
  },
  "5": {
    "type": "multiple_list",
    "ids": [200, 201],
    "other_value": "Other notes"
  }
}
```

## Troubleshooting

### Type Mismatch Errors
If you encounter type mismatch errors, ensure you're using the correct `CustomFieldInput` variant for each field type.

### Date Parsing Errors
Use the `chrono::NaiveDate::parse_from_str` method with the format `"%Y-%m-%d"` for consistent date parsing.

### List ID Validation
The API validates that list item IDs exist. Ensure the IDs you provide match the configured options in your Backlog project.

## Benefits of Migration

1. **Type Safety**: Compile-time checking prevents runtime errors
2. **Better Errors**: Descriptive error messages with examples
3. **Simpler CLI**: Intuitive command-line format
4. **IDE Support**: Auto-completion and type hints
5. **Maintainability**: Clearer code intent and structure

## Further Resources

- [Custom Fields Example](examples/custom_fields_example.rs)
- [Simple Custom Fields Example](examples/custom_fields_simple.rs)
- [API Documentation](API.md#custom-field-support)
- [CLI Usage](README.md#custom-fields)