# Combine Text Brick

The Combine Text brick merges multiple text fields into a single field.

## Configuration

```json
{
  "brick_type": "combine_text",
  "config": {
    "fields": ["first_name", "last_name"],
    "separator": " ",
    "output_field": "full_name"
  }
}
```

## Configuration Options

- **fields** (required): Array of field names to combine
- **separator** (optional): Separator between fields (default: `" "`)
- **output_field** (optional): Name of output field (default: `"combined_text"`)

## Input Format

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "title": "Mr"
}
```

## Output Format

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "title": "Mr",
  "full_name": "John Doe"
}
```

## Use Cases

- Combine first and last names
- Merge address components
- Create full descriptions from parts
- Concatenate multiple text fields

## Examples

### Basic Combination

```json
{
  "brick_type": "combine_text",
  "config": {
    "fields": ["first_name", "last_name"],
    "separator": " ",
    "output_field": "full_name"
  }
}
```

### Address Combination

```json
{
  "brick_type": "combine_text",
  "config": {
    "fields": ["street", "city", "state", "zip"],
    "separator": ", ",
    "output_field": "full_address"
  }
}
```

### Custom Separator

```json
{
  "brick_type": "combine_text",
  "config": {
    "fields": ["title", "description"],
    "separator": " - ",
    "output_field": "summary"
  }
}
```

## Handling Missing Fields

- Missing fields are skipped
- Empty strings are included
- Null values are treated as empty strings

