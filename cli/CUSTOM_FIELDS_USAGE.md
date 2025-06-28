# Custom Fields Usage in CLI

The Backlog CLI now supports custom fields when creating and updating issues.

## Usage

### Command Line Arguments

You can specify custom fields using the `--custom-field` option. The format is:
```
--custom-field "id:type:value[:other]"
```

Where:
- `id`: The custom field ID (numeric)
- `type`: The field type (text, textarea, numeric, date, single_list, multiple_list, checkbox, radio)
- `value`: The field value (format depends on type)
- `other`: Optional "other value" for list/radio fields

### Examples

#### Creating an issue with custom fields:

```bash
# Text field
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "1:text:Sample text value"

# Numeric field
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "2:numeric:123.45"

# Date field (YYYY-MM-DD format)
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "3:date:2024-06-24"

# Single selection list with "other" value
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "4:single_list:100:Other description"

# Multiple selection list
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "5:multiple_list:100,200,300"

# Checkbox (multiple IDs)
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "6:checkbox:10,20,30"

# Radio button with "other" value
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "7:radio:400:Other value"
```

#### Multiple custom fields:

```bash
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-field "1:text:Project ABC" \
  --custom-field "2:numeric:42.0" \
  --custom-field "3:date:2024-12-31" \
  --custom-field "4:single_list:100"
```

### JSON File Format

For complex custom field configurations, you can use a JSON file:

```bash
blg issue create -p PROJECT -s "Issue Title" -t 1 --priority-id 2 \
  --custom-fields-json custom_fields.json
```

Example `custom_fields.json`:
```json
{
  "1": {
    "type": "text",
    "value": "Sample text"
  },
  "2": {
    "type": "numeric",
    "value": 123.45
  },
  "3": {
    "type": "date",
    "value": "2024-06-24"
  },
  "4": {
    "type": "single_list",
    "id": 100,
    "other_value": "Other description"
  },
  "5": {
    "type": "multiple_list",
    "ids": [100, 200, 300],
    "other_value": "Additional notes"
  },
  "6": {
    "type": "checkbox",
    "ids": [10, 20, 30]
  },
  "7": {
    "type": "radio",
    "id": 400,
    "other_value": "Other option"
  }
}
```

### Updating Issues

The same options work with the update command:

```bash
# Update custom fields on an existing issue
blg issue update PROJECT-123 \
  --custom-field "1:text:Updated text" \
  --custom-field "2:numeric:99.99"

# Or using JSON file
blg issue update PROJECT-123 --custom-fields-json updated_fields.json
```

## Field Types Reference

| Type | Value Format | Example |
|------|--------------|---------|
| text | Plain text string | `1:text:Sample text` |
| textarea | Multi-line text string | `2:textarea:Line 1\nLine 2` |
| numeric | Decimal number | `3:numeric:123.45` |
| date | YYYY-MM-DD | `4:date:2024-06-24` |
| single_list | ID[:other] | `5:single_list:100:Other` |
| multiple_list | ID1,ID2,ID3[:other] | `6:multiple_list:100,200:Other` |
| checkbox | ID1,ID2,ID3 | `7:checkbox:10,20,30` |
| radio | ID[:other] | `8:radio:400:Other` |

## Error Handling

The CLI will display clear error messages for:
- Invalid field format
- Invalid date format (must be YYYY-MM-DD)
- Invalid numeric values
- Unknown field types
- JSON parsing errors

## Tips

1. Use the JSON file format for complex configurations or when setting many custom fields
2. The `--custom-field` and `--custom-fields-json` options are mutually exclusive
3. To find custom field IDs and available options, check your Backlog project settings
4. List field IDs (for single_list, multiple_list, checkbox, radio) must match the configured options in your project