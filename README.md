# FlowMason

A Rust-based visual automation platform.

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
