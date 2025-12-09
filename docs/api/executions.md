# Executions API

Execute flows and view execution history.

## Execute Flow

Execute a flow with input data:

```bash
POST /api/v1/executions
Authorization: Bearer <token>
Content-Type: application/json

{
  "flow_id": "flow-123",
  "input_payload": {
    "text": "Long text to summarize..."
  }
}
```

Response:

```json
{
  "execution_id": "exec-456",
  "flow_id": "flow-123",
  "status": "completed",
  "started_at": "2025-01-01T00:00:00Z",
  "completed_at": "2025-01-01T00:00:01Z",
  "input_payload": { ... },
  "output_payload": { ... },
  "error": null
}
```

## List Executions

Get all executions:

```bash
GET /api/v1/executions
Authorization: Bearer <token>
```

Response:

```json
[
  {
    "execution_id": "exec-456",
    "flow_id": "flow-123",
    "status": "completed",
    "started_at": "2025-01-01T00:00:00Z",
    "completed_at": "2025-01-01T00:00:01Z",
    "input_payload": { ... },
    "output_payload": { ... },
    "error": null
  }
]
```

## Get Execution

Get a specific execution:

```bash
GET /api/v1/executions/:id
Authorization: Bearer <token>
```

## Get Flow Executions

Get all executions for a specific flow:

```bash
GET /api/v1/executions/flow/:flow_id
Authorization: Bearer <token>
```

## Execution Status

Executions can have the following statuses:

- **pending**: Queued but not started
- **running**: Currently executing
- **completed**: Finished successfully
- **failed**: Encountered an error

## Execution Schema

```json
{
  "execution_id": "string",
  "flow_id": "string",
  "status": "pending | running | completed | failed",
  "started_at": "ISO 8601 datetime",
  "completed_at": "ISO 8601 datetime (optional)",
  "input_payload": { ... },
  "output_payload": { ... } (optional),
  "error": "string (optional)"
}
```

## Error Handling

If an execution fails:

```json
{
  "execution_id": "exec-456",
  "flow_id": "flow-123",
  "status": "failed",
  "started_at": "2025-01-01T00:00:00Z",
  "completed_at": "2025-01-01T00:00:01Z",
  "input_payload": { ... },
  "output_payload": null,
  "error": "API key invalid"
}
```

## Examples

### Execute with Simple Input

```bash
POST /api/v1/executions
{
  "flow_id": "text-summary-flow",
  "input_payload": {
    "text": "Long article text here..."
  }
}
```

### Execute with Complex Input

```bash
POST /api/v1/executions
{
  "flow_id": "customer-enrichment",
  "input_payload": {
    "user": {
      "name": "John Doe",
      "email": "john@example.com",
      "company": "Acme Corp"
    }
  }
}
```

## Execution Timeout

Executions have a timeout of 30 seconds. If a flow takes longer, it will be marked as failed.

## Retry Logic

Failed executions can be retried by executing the flow again with the same input.

