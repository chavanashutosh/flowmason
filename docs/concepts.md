# Core Concepts

This document explains the fundamental concepts of FlowMason.

## Flows

A **flow** is the central concept in FlowMason. It represents a sequence of operations that execute in order.

### Flow Structure

```json
{
  "id": "flow-123",
  "name": "Customer Enrichment",
  "description": "Enrich customer data with AI",
  "bricks": [
    {
      "brick_type": "field_mapping",
      "config": { ... }
    },
    {
      "brick_type": "openai",
      "config": { ... }
    }
  ],
  "active": true,
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

### Flow Execution

When a flow executes:

1. The input payload is passed to the first brick
2. Each brick processes the data and passes output to the next brick
3. The final brick's output becomes the flow's output
4. Execution status and results are stored

## Bricks

A **brick** is a single operation or integration. Bricks are the building blocks of flows.

### Brick Types

#### Integration Bricks

Connect to external services:
- **OpenAI**: AI text generation and processing
- **NVIDIA**: AI services (ASR, OCR, text generation)
- **HubSpot**: CRM operations (deals, contacts)
- **Notion**: Workspace operations (pages, databases)
- **Odoo**: ERP operations (invoices, products)
- **n8n**: Webhook integration

#### Processing Bricks

Transform and process data:
- **Field Mapping**: Map fields between data structures
- **Combine Text**: Combine multiple text fields
- **Conditional**: Apply conditional logic

### Brick Configuration

Each brick requires configuration specific to its type:

```json
{
  "brick_type": "openai",
  "config": {
    "api_key": "sk-...",
    "model_name": "gpt-3.5-turbo",
    "prompt_template": "Process: {{input}}"
  }
}
```

## Executions

An **execution** represents a single run of a flow.

### Execution States

- **Pending**: Execution is queued but not started
- **Running**: Execution is currently in progress
- **Completed**: Execution finished successfully
- **Failed**: Execution encountered an error

### Execution Data

Each execution stores:
- Flow ID and execution ID
- Input and output payloads
- Status and timestamps
- Error messages (if failed)

## Data Flow

Data flows through bricks sequentially:

```
Input → Brick 1 → Brick 2 → Brick 3 → Output
```

Each brick receives the output of the previous brick as its input.

### Example

```json
// Input
{ "name": "John", "email": "john@example.com" }

// After Field Mapping Brick
{ "customer_name": "John", "customer_email": "john@example.com" }

// After OpenAI Brick
{ "customer_name": "John", "customer_email": "john@example.com", "summary": "..." }

// Final Output
{ "customer_name": "John", "customer_email": "john@example.com", "summary": "..." }
```

## Scheduling

Flows can be scheduled to run automatically using cron expressions.

### Cron Expression Format

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of week (0 - 6) (Sunday to Saturday)
│ │ │ │ │
* * * * *
```

### Examples

- `0 * * * *` - Every hour
- `0 9 * * *` - Every day at 9 AM
- `0 0 * * 0` - Every Sunday at midnight
- `*/5 * * * *` - Every 5 minutes

## Usage & Metering

FlowMason tracks usage for each brick type:

- **Daily Usage**: Number of executions per day
- **Monthly Usage**: Number of executions per month
- **Quotas**: Limits on usage per brick type
- **Cost Tracking**: Cost units and token usage

### Quota Management

Each brick type has configurable quotas:
- Daily limit
- Monthly limit (optional)
- Automatic reset at day/month boundaries

## Authentication

FlowMason supports two authentication methods:

### JWT Tokens

- Register/login to get a JWT token
- Include token in `Authorization: Bearer <token>` header
- Tokens expire after a set period

### API Keys

- Generate API keys from the API
- Use API key in `Authorization: Bearer <api_key>` header
- API keys don't expire but can be revoked

## Error Handling

When a brick fails:

1. Error is captured and stored
2. Execution status is set to "Failed"
3. Error message is included in execution record
4. Subsequent bricks are not executed

### Error Recovery

- Retry failed executions manually
- Fix configuration and re-execute
- Check logs for detailed error information

