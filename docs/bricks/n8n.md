# n8n Brick

The n8n brick integrates with n8n workflows via webhooks, allowing you to trigger n8n workflows from FlowMason.

## Configuration

```json
{
  "brick_type": "n8n",
  "config": {
    "webhook_url": "https://your-n8n-instance.com/webhook/your-webhook-id",
    "method": "POST",
    "headers": {
      "Authorization": "Bearer your-token"
    }
  }
}
```

## Configuration Options

- **webhook_url** (required): n8n webhook URL
- **method** (required): HTTP method (`GET`, `POST`, `PUT`, `DELETE`)
- **headers** (optional): Custom HTTP headers
- **timeout** (optional): Request timeout in seconds (default: 30)

## Supported Methods

### POST (Default)

Send data to n8n webhook:

```json
{
  "brick_type": "n8n",
  "config": {
    "webhook_url": "https://n8n.example.com/webhook/abc123",
    "method": "POST"
  }
}
```

### GET

Retrieve data from n8n:

```json
{
  "brick_type": "n8n",
  "config": {
    "webhook_url": "https://n8n.example.com/webhook/abc123",
    "method": "GET",
    "query_params": {
      "id": "{{item_id}}"
    }
  }
}
```

## Input Format

The entire input payload is sent to the n8n webhook:

```json
{
  "data": "any data structure",
  "metadata": {
    "source": "flowmason"
  }
}
```

## Output Format

Returns n8n webhook response:

```json
{
  "result": "n8n workflow output",
  "status": "success"
}
```

## Use Cases

- Trigger complex n8n workflows
- Chain FlowMason and n8n workflows
- Integrate with existing n8n automations
- Use n8n as a processing step

## Setting Up n8n Webhook

1. Create a webhook node in your n8n workflow
2. Copy the webhook URL
3. Configure authentication if needed
4. Use the URL in your FlowMason brick configuration

## Authentication

You can authenticate n8n webhooks using:

- **Bearer Token**: Add to headers
- **API Key**: Include in headers or query params
- **Basic Auth**: Configure in headers

