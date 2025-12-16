# FlowMason API Reference

## Base URL

```
http://localhost:3000/api/v1
```

## Authentication

All endpoints (except `/bricks` and `/health`) require authentication via:

```
Authorization: Bearer <token>
```

or

```
Authorization: Bearer <api_key>
```

## Endpoints

### Health Check

#### GET /health

Check API server health.

**Response:**
```json
{
  "status": "ok"
}
```

### Authentication

#### POST /auth/register

Register a new user.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "user": {
    "id": "user_id",
    "email": "user@example.com"
  }
}
```

#### POST /auth/login

Login and get JWT token.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "user": {
    "id": "user_id",
    "email": "user@example.com"
  }
}
```

### Flows

#### GET /flows

List all flows with pagination.

**Query Parameters:**
- `limit` (optional): Number of results (default: 100)
- `offset` (optional): Pagination offset (default: 0)

**Response:**
```json
{
  "items": [
    {
      "id": "flow_id",
      "name": "Flow Name",
      "description": "Flow description",
      "active": true,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ],
  "limit": 100,
  "offset": 0
}
```

#### POST /flows

Create a new flow.

**Request:**
```json
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

**Response:**
```json
{
  "id": "flow_id",
  "name": "My Flow",
  "description": "Flow description",
  "active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

#### GET /flows/:id

Get a specific flow.

**Response:**
```json
{
  "id": "flow_id",
  "name": "My Flow",
  "description": "Flow description",
  "bricks": [...],
  "active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

#### PUT /flows/:id

Update a flow.

**Request:**
```json
{
  "name": "Updated Name",
  "description": "Updated description",
  "bricks": [...],
  "active": false
}
```

#### DELETE /flows/:id

Delete a flow.

**Response:** 204 No Content

#### POST /flows/:id/duplicate

Duplicate a flow.

**Response:**
```json
{
  "id": "new_flow_id",
  "name": "My Flow (Copy)",
  ...
}
```

#### GET /flows/:id/export

Export a flow as JSON.

**Response:**
```json
{
  "version": "1.0",
  "exported_at": "2025-01-01T00:00:00Z",
  "flow": {
    "name": "My Flow",
    "description": "...",
    "bricks": [...]
  }
}
```

#### POST /flows/import

Import a flow from JSON.

**Request:**
```json
{
  "flow": {
    "name": "Imported Flow",
    "description": "...",
    "bricks": [...]
  }
}
```

### Executions

#### POST /executions

Execute a flow.

**Request:**
```json
{
  "flow_id": "flow_id",
  "input_payload": {
    "data": "value"
  }
}
```

**Response:**
```json
{
  "execution_id": "exec_id",
  "flow_id": "flow_id",
  "status": "completed",
  "started_at": "2025-01-01T00:00:00Z",
  "completed_at": "2025-01-01T00:00:01Z",
  "output_payload": {
    "result": "data"
  }
}
```

#### GET /executions

List all executions.

**Query Parameters:**
- `limit` (optional): Number of results
- `offset` (optional): Pagination offset

#### GET /executions/:id

Get execution details.

#### GET /executions/flow/:flow_id

Get executions for a specific flow.

### Webhooks

#### POST /webhooks/flows/:flow_id/trigger

Trigger a flow via webhook (no authentication required).

**Request Body:** JSON payload (optional)

**Response:**
```json
{
  "success": true,
  "execution_id": "exec_id",
  "status": "completed",
  "output": {...}
}
```

### Scheduler

#### POST /scheduler/flows

Schedule a flow.

**Request:**
```json
{
  "flow_id": "flow_id",
  "cron_expression": "0 9 * * *"
}
```

**Response:**
```json
{
  "job_id": "job_id",
  "flow_id": "flow_id",
  "cron_expression": "0 9 * * *",
  "scheduled_at": "2025-01-01T00:00:00Z"
}
```

#### GET /scheduler/flows

List scheduled flows.

#### DELETE /scheduler/flows/:flow_id

Unschedule a flow.

### Usage & Metering

#### GET /usage

Get usage logs.

#### GET /usage/stats

Get usage statistics.

**Response:**
```json
[
  {
    "brick_type": "openai",
    "daily_usage": 50,
    "daily_limit": 200,
    "monthly_usage": 500,
    "monthly_limit": 5000
  }
]
```

### Bricks

#### GET /bricks

List available bricks (no authentication required).

**Response:**
```json
[
  {
    "name": "openai",
    "brick_type": "OpenAi",
    "config_schema": {...}
  }
]
```

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message",
  "status": 400
}
```

### Status Codes

- `200` - Success
- `201` - Created
- `204` - No Content
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `500` - Internal Server Error

## Rate Limiting

Rate limits are enforced per IP:
- **Limit**: 100 requests per second
- **Burst**: 200 requests

Rate limit headers are included in responses:
- `X-RateLimit-Limit`: Maximum requests
- `X-RateLimit-Remaining`: Remaining requests
