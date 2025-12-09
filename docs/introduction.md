# Introduction to FlowMason

FlowMason is a Rust-based visual automation platform that allows you to build powerful workflows by connecting different services and APIs together. Think of it as a flexible automation tool that lets you create "flows" by chaining together "bricks" - each brick represents a specific integration or operation.

## What is FlowMason?

FlowMason enables you to:

- **Connect Services**: Integrate with popular services like OpenAI, HubSpot, Notion, Odoo, and more
- **Build Workflows**: Create automated workflows by chaining multiple operations together
- **Schedule Tasks**: Run workflows on a schedule using cron expressions
- **Track Usage**: Monitor usage and costs for each integration
- **Transform Data**: Map and transform data between different formats

## Key Concepts

### Flows

A **flow** is a sequence of operations that execute in order. Each flow consists of:
- A unique ID
- A name and description
- A list of bricks (operations)
- An active/inactive status

### Bricks

A **brick** is a single operation or integration. FlowMason supports several types of bricks:

- **AI Services**: OpenAI, NVIDIA
- **CRM/Business Tools**: HubSpot, Notion, Odoo
- **Automation**: n8n webhooks
- **Data Processing**: Field mapping, text combination, conditional logic

### Executions

An **execution** represents a single run of a flow. Each execution tracks:
- The flow that was executed
- Input and output payloads
- Execution status (pending, running, completed, failed)
- Timestamps and error messages

## Architecture

FlowMason is built with:

- **Backend**: Rust with Axum web framework
- **Database**: SQLite for data persistence
- **Frontend**: Next.js with React
- **Scheduler**: Cron-based task scheduling

## License

FlowMason uses a custom proprietary license. See the LICENSE file for details. Free for personal/localhost use, commercial license required for business use.

