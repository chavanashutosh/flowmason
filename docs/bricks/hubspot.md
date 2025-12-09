# HubSpot Brick

The HubSpot brick integrates with HubSpot CRM to perform operations on deals, contacts, and companies.

## Configuration

```json
{
  "brick_type": "hubspot",
  "config": {
    "api_key": "your-hubspot-api-key",
    "operation": "create_deal",
    "properties": {
      "dealname": "{{deal_name}}",
      "amount": "{{amount}}",
      "dealstage": "appointmentscheduled"
    }
  }
}
```

## Configuration Options

- **api_key** (required): Your HubSpot API key
- **operation** (required): Operation to perform (see below)
- **properties** (optional): Properties to set (varies by operation)

## Supported Operations

### Create Deal

```json
{
  "brick_type": "hubspot",
  "config": {
    "api_key": "your-key",
    "operation": "create_deal",
    "properties": {
      "dealname": "{{deal_name}}",
      "amount": "{{amount}}",
      "dealstage": "appointmentscheduled"
    }
  }
}
```

### Get Contact

```json
{
  "brick_type": "hubspot",
  "config": {
    "api_key": "your-key",
    "operation": "get_contact",
    "contact_id": "{{contact_id}}"
  }
}
```

### Update Deal

```json
{
  "brick_type": "hubspot",
  "config": {
    "api_key": "your-key",
    "operation": "update_deal",
    "deal_id": "{{deal_id}}",
    "properties": {
      "dealstage": "closedwon"
    }
  }
}
```

## Input Format

Varies by operation. For create operations:

```json
{
  "deal_name": "New Deal",
  "amount": "5000",
  "contact_id": "12345"
}
```

## Output Format

Returns HubSpot API response format:

```json
{
  "id": "deal-id",
  "properties": {
    "dealname": "New Deal",
    "amount": "5000"
  }
}
```

## Use Cases

- Automatically create deals from form submissions
- Sync contact information
- Update deal stages
- Track sales pipeline

## Common Properties

- **dealname**: Deal name
- **amount**: Deal amount
- **dealstage**: Deal stage ID
- **pipeline**: Pipeline ID
- **closedate**: Close date

