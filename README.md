# FlowMason

A visual automation platform that allows you to build powerful workflows by connecting different services and APIs together. FlowMason lets you create automated "flows" by chaining together "bricks" - each brick represents a specific integration or operation.

## What is FlowMason?

FlowMason is a workflow automation tool that enables you to:

- **Connect Services**: Integrate with popular services like OpenAI, HubSpot, Notion, Odoo, n8n, NVIDIA, and more
- **Build Workflows**: Create automated workflows by chaining multiple operations together in a visual interface
- **Schedule Tasks**: Run workflows on a schedule using cron expressions
- **Track Usage**: Monitor usage and costs for each integration with built-in metering
- **Transform Data**: Map and transform data between different formats using field mapping, text combination, and conditional logic
- **Execute Flows**: Run flows on-demand or via webhooks, with full execution history and error tracking

## Key Concepts

### Flows
A **flow** is a sequence of operations that execute in order. Each flow consists of bricks (operations) that process data sequentially, passing output from one brick to the next.

### Bricks
A **brick** is a single operation or integration. FlowMason supports:
- **AI Services**: OpenAI (text generation), NVIDIA (ASR, OCR, text generation)
- **CRM/Business Tools**: HubSpot (deals, contacts), Notion (pages, databases), Odoo (invoices, products)
- **Automation**: n8n webhooks
- **Data Processing**: Field mapping, text combination, conditional logic

### Executions
Each flow execution tracks input/output payloads, execution status (pending, running, completed, failed), timestamps, and error messages.

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: SQLite for data persistence
- **Frontend**: Dioxus (Rust-based web framework)
- **Scheduler**: Cron-based task scheduling
- **Authentication**: JWT-based authentication with API keys

## Tags

`#rust` `#automation` `#workflow` `#api-integration` `#visual-automation` `#workflow-automation` `#no-code` `#low-code` `#integration-platform` `#dioxus` `#axum` `#openai` `#hubspot` `#notion` `#odoo` `#n8n` `#nvidia` `#cron` `#scheduler` `#webhooks` `#api` `#backend` `#fullstack` `#rustlang` `#automation-tool` `#workflow-engine` `#integration-tool` `#saas` `#productivity` `#business-automation`

## How to Run the Project

### Prerequisites

- Rust 1.70+ ([install Rust](https://rustup.rs/))
- Dioxus CLI: `cargo install dioxus-cli`

### Environment Setup

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Generate a secure JWT secret and update `JWT_SECRET` in `.env`:
   - Linux/Mac: `openssl rand -base64 32`
   - Windows PowerShell: `[Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }))`

### Start the API Server

```bash
cd services/api
cargo run
```

The API server will start on **http://localhost:3000**

### Build and Start the Web UI (Dioxus)

The Web UI is now built with Dioxus (Rust). Build it first:

```bash
cd services/web-ui
dx build --release
```

Or for development:

```bash
cd services/web-ui
dx serve --platform web
```

**Note**: Always specify `--platform web` when running Dioxus commands.

The Web UI will be available on **http://localhost:8080** (development) or served by the API server at **http://localhost:3000** (production)

### Access Points

- **API Server**: http://localhost:3000
- **Web UI (Dev)**: http://localhost:8080 (when running `dx serve`)
- **Web UI (Prod)**: http://localhost:3000 (served by API server after building)
