# Field Mapping Brick

The Field Mapping brick transforms data by mapping fields from one structure to another.

## Configuration

```json
{
  "brick_type": "field_mapping",
  "config": {
    "mappings": [
      {
        "source_path": "user.name",
        "target_path": "customer.name"
      },
      {
        "source_path": "user.email",
        "target_path": "customer.email"
      }
    ]
  }
}
```

## Configuration Options

- **mappings** (required): Array of mapping rules
  - **source_path**: Path to source field (dot notation)
  - **target_path**: Path to target field (dot notation)

## Input Format

```json
{
  "user": {
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30
  }
}
```

## Output Format

```json
{
  "customer": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

## Use Cases

- Transform data between different API formats
- Rename fields for compatibility
- Extract nested data
- Restructure JSON objects

## Examples

### Simple Mapping

```json
{
  "brick_type": "field_mapping",
  "config": {
    "mappings": [
      {
        "source_path": "name",
        "target_path": "full_name"
      }
    ]
  }
}
```

### Nested Mapping

```json
{
  "brick_type": "field_mapping",
  "config": {
    "mappings": [
      {
        "source_path": "user.profile.name",
        "target_path": "customer.name"
      },
      {
        "source_path": "user.profile.email",
        "target_path": "customer.contact.email"
      }
    ]
  }
}
```

### Array Mapping

```json
{
  "brick_type": "field_mapping",
  "config": {
    "mappings": [
      {
        "source_path": "items[0].name",
        "target_path": "first_item_name"
      }
    ]
  }
}
```

## Path Notation

- Use dot notation for nested objects: `user.profile.name`
- Use brackets for arrays: `items[0].name`
- Use empty string for root: `""` maps to root level

