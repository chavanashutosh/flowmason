# API Overview

FlowMason provides a RESTful API for managing flows, executions, and integrations.

## Base URL

```
http://localhost:3000/api/v1
```

## Authentication

Most endpoints require authentication. See [Authentication](authentication.md) for details.

## Endpoints

### Flows
- `GET /flows` - List all flows
- `POST /flows` - Create a new flow
- `GET /flows/:id` - Get a flow by ID
- `PUT /flows/:id` - Update a flow
- `DELETE /flows/:id` - Delete a flow

### Executions
- `POST /executions` - Execute a flow
- `GET /executions` - List all executions
- `GET /executions/:id` - Get execution by ID
- `GET /executions/flow/:flow_id` - Get executions for a flow

### Bricks
- `GET /bricks` - List all available bricks
- `GET /bricks/:brick_type/schema` - Get brick configuration schema

### Scheduler
- `POST /scheduler/flows` - Schedule a flow
- `GET /scheduler/flows` - List scheduled flows
- `DELETE /scheduler/flows/:flow_id` - Unschedule a flow

### Usage
- `GET /usage` - List usage logs
- `GET /usage/stats` - Get usage statistics
- `GET /usage/stats/:brick_type` - Get stats for a brick type

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and get JWT token
- `POST /auth/api-keys` - Create API key
- `GET /auth/api-keys` - List API keys
- `DELETE /auth/api-keys/:id` - Delete API key

## Response Format

### Success Response

```json
{
  "id": "flow-123",
  "name": "My Flow",
  ...
}
```

### Error Response

```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

## Status Codes

- `200` - Success
- `201` - Created
- `204` - No Content
- `400` - Bad Request
- `401` - Unauthorized
- `404` - Not Found
- `500` - Internal Server Error

## Rate Limiting

API requests are rate-limited to 100 requests per second per IP address.

## Pagination

List endpoints support pagination (when implemented):
- `?page=1` - Page number
- `?limit=50` - Items per page

