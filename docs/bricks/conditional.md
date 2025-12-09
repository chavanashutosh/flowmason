# Conditional Brick

The Conditional brick applies conditional logic to route or transform data based on conditions.

## Configuration

```json
{
  "brick_type": "conditional",
  "config": {
    "condition": "{{amount}} > 1000",
    "true_value": "high_value",
    "false_value": "low_value",
    "output_field": "category"
  }
}
```

## Configuration Options

- **condition** (required): Condition expression (JavaScript-like syntax)
- **true_value** (optional): Value when condition is true
- **false_value** (optional): Value when condition is false
- **output_field** (optional): Field name for output (default: `"result"`)

## Input Format

```json
{
  "amount": 1500,
  "status": "active"
}
```

## Output Format

```json
{
  "amount": 1500,
  "status": "active",
  "category": "high_value"
}
```

## Use Cases

- Route data based on conditions
- Categorize items
- Apply business rules
- Filter or transform data conditionally

## Examples

### Simple Comparison

```json
{
  "brick_type": "conditional",
  "config": {
    "condition": "{{amount}} > 1000",
    "true_value": "high",
    "false_value": "low",
    "output_field": "tier"
  }
}
```

### String Comparison

```json
{
  "brick_type": "conditional",
  "config": {
    "condition": "{{status}} === 'active'",
    "true_value": "enabled",
    "false_value": "disabled",
    "output_field": "state"
  }
}
```

### Complex Condition

```json
{
  "brick_type": "conditional",
  "config": {
    "condition": "{{amount}} > 1000 && {{status}} === 'active'",
    "true_value": "premium",
    "false_value": "standard",
    "output_field": "plan"
  }
}
```

### Multiple Conditions

You can chain multiple conditional bricks for complex logic:

```json
[
  {
    "brick_type": "conditional",
    "config": {
      "condition": "{{amount}} > 5000",
      "true_value": "enterprise",
      "false_value": "{{amount}}"
    }
  },
  {
    "brick_type": "conditional",
    "config": {
      "condition": "{{result}} > 1000",
      "true_value": "business",
      "false_value": "personal"
    }
  }
]
```

## Supported Operators

- Comparison: `>`, `<`, `>=`, `<=`, `===`, `!==`
- Logical: `&&`, `||`, `!`
- Arithmetic: `+`, `-`, `*`, `/`, `%`

## Expression Syntax

- Use `{{field_name}}` to reference input fields
- Supports nested paths: `{{user.profile.age}}`
- Supports array access: `{{items[0].price}}`

