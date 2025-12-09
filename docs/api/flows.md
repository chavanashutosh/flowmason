# Flows API

Manage flows in FlowMason.

## List Flows

Get all flows:

```bash
GET /api/v1/flows
Authorization: Bearer <token>
```

Response:

```json
[
  {
    "id": "flow-123",
    "name": "Customer Enrichment",
    "description": "Enrich customer data",
    "bricks": [
      {
        "brick_type": "field_mapping",
        "config": { ... }
      }
    ],
    "active": true,
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z"
  }
]
```

## Get Flow

Get a specific flow:

```bash
GET /api/v1/flows/:id
Authorization: Bearer <token>
```

## Create Flow

Create a new flow:

```bash
POST /api/v1/flows
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "My Flow",
  "description": "Flow description",
  "bricks": [
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Process: {{input}}"
      }
    }
  ]
}
```

## Update Flow

Update an existing flow:

```bash
PUT /api/v1/flows/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Updated Flow Name",
  "bricks": [ ... ]
}
```

## Delete Flow

Delete a flow:

```bash
DELETE /api/v1/flows/:id
Authorization: Bearer <token>
```

## Flow Schema

```json
{
  "id": "string",
  "name": "string",
  "description": "string (optional)",
  "bricks": [
    {
      "brick_type": "string",
      "config": { ... }
    }
  ],
  "active": "boolean",
  "created_at": "ISO 8601 datetime",
  "updated_at": "ISO 8601 datetime"
}
```

## Brick Configuration

Each brick requires specific configuration. See the [Bricks documentation](../bricks/) for details.

## Examples

### Simple Flow

```json
{
  "name": "Text Summarization",
  "bricks": [
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Summarize: {{text}}"
      }
    }
  ]
}
```

### Multi-Step Flow

```json
{
  "name": "Customer Processing",
  "bricks": [
    {
      "brick_type": "field_mapping",
      "config": {
        "mappings": [
          {
            "source_path": "user.name",
            "target_path": "customer.name"
          }
        ]
      }
    },
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Analyze customer: {{customer.name}}"
      }
    },
    {
      "brick_type": "hubspot",
      "config": {
        "api_key": "hubspot-key",
        "operation": "create_deal",
        "properties": {
          "dealname": "{{customer.name}}"
        }
      }
    }
  ]
}
```

