# Usage & Metering API

Monitor usage and quotas for each brick type.

## List Usage Logs

Get all usage logs:

```bash
GET /api/v1/usage
Authorization: Bearer <token>
```

Response:

```json
[
  {
    "id": "log-123",
    "brick_name": "openai",
    "flow_id": "flow-123",
    "execution_id": "exec-456",
    "timestamp": "2025-01-01T00:00:00Z",
    "cost_unit": 0.01,
    "token_usage": 100,
    "metadata": { ... }
  }
]
```

## Get Usage Statistics

Get usage statistics for all brick types:

```bash
GET /api/v1/usage/stats
Authorization: Bearer <token>
```

Response:

```json
[
  {
    "brick_type": "openai",
    "daily_usage": 50,
    "daily_limit": 200,
    "monthly_usage": 500,
    "monthly_limit": 5000
  },
  {
    "brick_type": "nvidia",
    "daily_usage": 100,
    "daily_limit": 1000,
    "monthly_usage": 2000,
    "monthly_limit": 25000
  }
]
```

## Get Brick Statistics

Get statistics for a specific brick type:

```bash
GET /api/v1/usage/stats/:brick_type
Authorization: Bearer <token>
```

Supported brick types:
- `openai`
- `nvidia`
- `hubspot`
- `notion`
- `odoo`
- `n8n`
- `field_mapping`
- `combine_text`
- `conditional`

## Usage Log Schema

```json
{
  "id": "string",
  "brick_name": "string",
  "flow_id": "string",
  "execution_id": "string",
  "timestamp": "ISO 8601 datetime",
  "cost_unit": "number",
  "token_usage": "number (optional)",
  "metadata": { ... } (optional)
}
```

## Statistics Schema

```json
{
  "brick_type": "string",
  "daily_usage": "number",
  "daily_limit": "number",
  "monthly_usage": "number (optional)",
  "monthly_limit": "number (optional)"
}
```

## Quota Management

Quotas are automatically managed:

- **Daily limits**: Reset at midnight UTC
- **Monthly limits**: Reset on the first day of the month
- **Usage tracking**: Automatically incremented on each execution

## Monitoring Usage

Use the statistics endpoint to:

- Monitor daily and monthly usage
- Check if quotas are approaching limits
- Track costs across different brick types
- Identify high-usage flows

## Examples

### Check OpenAI Usage

```bash
GET /api/v1/usage/stats/openai
```

### Monitor All Usage

```bash
GET /api/v1/usage/stats
```

Response shows usage vs limits for all brick types.

## Quota Exceeded

When a quota is exceeded:

- Flow execution will fail with a quota error
- Error message indicates which quota was exceeded
- Usage statistics will show usage >= limit

## Resetting Quotas

Quotas reset automatically:
- Daily quotas reset at midnight UTC
- Monthly quotas reset on the 1st of each month

Manual quota adjustments require database access or admin API (if implemented).

