# Scheduler API

Schedule flows to run automatically using cron expressions.

## Schedule Flow

Schedule a flow to run on a schedule:

```bash
POST /api/v1/scheduler/flows
Authorization: Bearer <token>
Content-Type: application/json

{
  "flow_id": "flow-123",
  "cron_expression": "0 9 * * *"
}
```

Response:

```json
{
  "job_id": "job-456",
  "flow_id": "flow-123",
  "cron_expression": "0 9 * * *",
  "scheduled_at": "2025-01-01T00:00:00Z"
}
```

## List Scheduled Flows

Get all scheduled flows:

```bash
GET /api/v1/scheduler/flows
Authorization: Bearer <token>
```

Response:

```json
{
  "flows": [
    {
      "flow_id": "flow-123",
      "cron_expression": "0 9 * * *"
    }
  ]
}
```

## Unschedule Flow

Remove a scheduled flow:

```bash
DELETE /api/v1/scheduler/flows/:flow_id
Authorization: Bearer <token>
```

## Cron Expression Format

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of week (0 - 6) (Sunday to Saturday)
│ │ │ │ │
* * * * *
```

## Common Cron Expressions

- `0 * * * *` - Every hour
- `0 9 * * *` - Every day at 9 AM
- `0 0 * * 0` - Every Sunday at midnight
- `*/5 * * * *` - Every 5 minutes
- `0 0 1 * *` - First day of every month at midnight
- `0 9 * * 1-5` - Every weekday at 9 AM

## Examples

### Daily Report

```bash
POST /api/v1/scheduler/flows
{
  "flow_id": "daily-report-flow",
  "cron_expression": "0 9 * * *"
}
```

Runs every day at 9 AM.

### Hourly Sync

```bash
POST /api/v1/scheduler/flows
{
  "flow_id": "sync-flow",
  "cron_expression": "0 * * * *"
}
```

Runs every hour.

### Weekly Summary

```bash
POST /api/v1/scheduler/flows
{
  "flow_id": "weekly-summary",
  "cron_expression": "0 0 * * 0"
}
```

Runs every Sunday at midnight.

## Scheduled Execution Input

Scheduled flows execute with an empty payload `{}` by default. To provide input data, modify the flow to use default values or use a webhook trigger.

## Updating Schedules

To update a schedule, delete the existing schedule and create a new one with the updated cron expression.

## Persistence

Scheduled flows are persisted to the database and will be restored when the server restarts.

