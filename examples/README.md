# FlowMason Examples

This directory contains examples of integrations and connections for FlowMason workflows.

## Structure

- **`integrations/`** - Examples of individual integration bricks (HubSpot, Notion, Odoo, n8n, OpenAI, NVIDIA)
- **`connections/`** - Examples of how to connect bricks together (field mapping, combine text, conditional logic)
- **`workflows/`** - Complete workflow examples combining multiple integrations

## Integration Examples

### CRM & Business Tools
- **HubSpot** (`integrations/hubspot-example.json`) - CRM operations (create deals, get contacts, update deals)
- **Notion** (`integrations/notion-example.json`) - Workspace operations (create pages, get pages, update pages)
- **Odoo** (`integrations/odoo-example.json`) - ERP operations (get invoices, create invoices, get products)

### Webhooks & Automation
- **n8n** (`integrations/n8n-example.json`) - Webhook integration (POST, GET, PUT methods)

### AI Services
- **OpenAI** (`integrations/openai-example.json`) - Text generation, summarization, customer support
- **NVIDIA** (`integrations/nvidia-example.json`) - ASR, OCR, text generation services

## Connection Examples

### Data Transformation
- **Field Mapping** (`connections/field-mapping-example.json`) - Map fields between different data structures
- **Combine Text** (`connections/combine-text-example.json`) - Combine multiple text fields into one
- **Conditional Logic** (`connections/conditional-example.json`) - Apply conditional logic based on field values

## Workflow Examples

### Complete Workflows
- **HubSpot to Notion Sync** (`workflows/hubspot-to-notion.json`) - Sync deals from HubSpot to Notion
- **Customer Enrichment** (`workflows/customer-enrichment.json`) - Enrich customer data with AI and sync to CRM
- **Webhook to Odoo** (`workflows/webhook-to-odoo.json`) - Create invoices in Odoo from webhook data
- **AI Content to Notion** (`workflows/ai-content-to-notion.json`) - Generate content with AI and save to Notion
- **n8n Webhook Pipeline** (`workflows/n8n-webhook-pipeline.json`) - Process data through n8n and sync to HubSpot

## Usage

Each example file contains:
- **name**: Name of the example
- **description**: What the example does
- **brick_type**: Type of brick being used
- **config**: Configuration for the brick
- **input_example**: Sample input data
- **output_example** (for connections): Expected output format

## Building Your Own Workflows

1. **Start with a single integration** - Use examples from `integrations/` to understand how each service works
2. **Add connections** - Use examples from `connections/` to transform data between bricks
3. **Combine into workflows** - Reference `workflows/` examples to see how multiple bricks work together

## Notes

- Replace placeholder values (like `your-api-key`) with your actual credentials
- Input/output examples are illustrative - adjust based on your actual data structure
- Bricks execute sequentially - the output of one brick becomes the input of the next

