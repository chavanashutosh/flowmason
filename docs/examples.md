# Examples

This page contains example workflows and use cases for FlowMason.

## Customer Data Enrichment

Enrich customer data with AI and sync to CRM.

### Flow Configuration

```json
{
  "name": "Customer Enrichment",
  "description": "Enrich customer data with AI and create HubSpot deal",
  "bricks": [
    {
      "brick_type": "field_mapping",
      "config": {
        "mappings": [
          {
            "source_path": "form.name",
            "target_path": "customer.name"
          },
          {
            "source_path": "form.email",
            "target_path": "customer.email"
          },
          {
            "source_path": "form.company",
            "target_path": "customer.company"
          }
        ]
      }
    },
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Analyze this customer and provide a summary: Name: {{customer.name}}, Company: {{customer.company}}, Email: {{customer.email}}"
      }
    },
    {
      "brick_type": "hubspot",
      "config": {
        "api_key": "hubspot-key",
        "operation": "create_deal",
        "properties": {
          "dealname": "{{customer.name}} - {{customer.company}}",
          "amount": "5000"
        }
      }
    }
  ]
}
```

### Input

```json
{
  "form": {
    "name": "John Doe",
    "email": "john@example.com",
    "company": "Acme Corp"
  }
}
```

## Meeting Notes to Notion

Automatically create Notion pages from meeting notes.

### Flow Configuration

```json
{
  "name": "Meeting Notes to Notion",
  "description": "Create Notion pages from meeting notes with AI summary",
  "bricks": [
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Summarize these meeting notes and extract key action items:\n\n{{meeting_notes}}"
      }
    },
    {
      "brick_type": "notion",
      "config": {
        "api_key": "secret-...",
        "database_id": "notion-db-id",
        "operation": "create_page",
        "properties": {
          "title": "{{meeting_title}}",
          "content": "{{response}}"
        }
      }
    }
  ]
}
```

### Input

```json
{
  "meeting_title": "Weekly Team Sync",
  "meeting_notes": "Discussed project progress, assigned tasks..."
}
```

## Invoice Processing

Process invoices from webhooks and create Odoo invoices.

### Flow Configuration

```json
{
  "name": "Invoice Processing",
  "description": "Create Odoo invoices from webhook data",
  "bricks": [
    {
      "brick_type": "field_mapping",
      "config": {
        "mappings": [
          {
            "source_path": "order.customer_id",
            "target_path": "invoice.partner_id"
          },
          {
            "source_path": "order.items",
            "target_path": "invoice.line_items"
          },
          {
            "source_path": "order.total",
            "target_path": "invoice.amount_total"
          }
        ]
      }
    },
    {
      "brick_type": "odoo",
      "config": {
        "api_url": "https://odoo.example.com",
        "database": "mydb",
        "username": "admin",
        "api_key": "password",
        "operation": "create",
        "model": "account.move",
        "fields": {
          "partner_id": "{{invoice.partner_id}}",
          "invoice_line_ids": "{{invoice.line_items}}"
        }
      }
    }
  ]
}
```

## Text Summarization Pipeline

Summarize long text using multiple AI services.

### Flow Configuration

```json
{
  "name": "Text Summarization",
  "description": "Summarize text using OpenAI",
  "bricks": [
    {
      "brick_type": "openai",
      "config": {
        "api_key": "sk-...",
        "model_name": "gpt-3.5-turbo",
        "prompt_template": "Summarize the following text in 3 bullet points:\n\n{{text}}",
        "temperature": 0.3,
        "max_tokens": 200
      }
    }
  ]
}
```

### Input

```json
{
  "text": "Long article or document text here..."
}
```

## Conditional Data Routing

Route data based on conditions.

### Flow Configuration

```json
{
  "name": "Conditional Routing",
  "description": "Route data based on amount",
  "bricks": [
    {
      "brick_type": "conditional",
      "config": {
        "condition": "{{amount}} > 1000",
        "true_value": "high_value",
        "false_value": "low_value",
        "output_field": "category"
      }
    },
    {
      "brick_type": "field_mapping",
      "config": {
        "mappings": [
          {
            "source_path": "category",
            "target_path": "deal.tier"
          }
        ]
      }
    }
  ]
}
```

## Scheduled Daily Reports

Schedule a flow to run daily.

### Schedule Configuration

```bash
POST /api/v1/scheduler/flows
{
  "flow_id": "daily-report-flow",
  "cron_expression": "0 9 * * *"
}
```

Runs every day at 9 AM UTC.

## Combining Multiple Text Fields

Combine customer name fields.

### Flow Configuration

```json
{
  "name": "Combine Customer Name",
  "description": "Combine first and last name",
  "bricks": [
    {
      "brick_type": "combine_text",
      "config": {
        "fields": ["first_name", "last_name"],
        "separator": " ",
        "output_field": "full_name"
      }
    }
  ]
}
```

### Input

```json
{
  "first_name": "John",
  "last_name": "Doe"
}
```

### Output

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "full_name": "John Doe"
}
```

## Best Practices

1. **Start Simple**: Begin with single-brick flows
2. **Test Incrementally**: Add bricks one at a time
3. **Use Field Mapping**: Transform data between bricks
4. **Handle Errors**: Check execution status and errors
5. **Monitor Usage**: Track usage to stay within quotas
6. **Document Flows**: Add descriptions to flows
7. **Use Templates**: Reuse common flow patterns

