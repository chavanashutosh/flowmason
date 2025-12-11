# Getting Started

This guide will help you get FlowMason up and running on your local machine.

## Prerequisites

Before you begin, make sure you have the following installed:

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Dioxus CLI**: Install with `cargo install dioxus-cli --locked`
- **SQLite**: Usually comes pre-installed on most systems

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/chavanashutosh/flowmason.git
cd flowmason
```

### 2. Build the Backend

```bash
cargo build --release
```

### 3. Build the Web UI

```bash
cd services/web-ui
dx build --release
cd ../..
```

## Running FlowMason

### Start the API Server

In the project root directory:

```bash
cd services/api
cargo run
```

The API server will start on **http://localhost:3000**

### Start the Web UI (Development)

In a new terminal:

```bash
cd services/web-ui
dx serve --platform web
```

The Web UI will be available on **http://localhost:8080**

**Note**: For production, the web UI is built and served by the API server at http://localhost:3000

## Access Points

- **API Server**: http://localhost:3000
- **Web UI (Development)**: http://localhost:8080 (when running `dx serve`)
- **Web UI (Production)**: http://localhost:3000 (served by API server)
- **API Documentation**: http://localhost:3000/api/v1/bricks

## First Steps

1. **Access the Web UI**: Open http://localhost:8080 (development) or http://localhost:3000 (production) in your browser
2. **Create an Account**: Register a new user account
3. **Create Your First Flow**: Use the Flows page to create a new workflow
4. **Add Bricks**: Add bricks to your flow from the available integrations
5. **Execute**: Test your flow by executing it manually
6. **Schedule**: Set up automated execution using the Scheduler

## Configuration

### Environment Variables

Before running FlowMason, you should set up your environment variables:

1. **Copy the example environment file**:
   ```bash
   cp .env.example .env
   ```

2. **Generate a secure JWT secret**:
   - On Linux/Mac: `openssl rand -base64 32`
   - On Windows PowerShell: `[Convert]::ToBase64String((1..32 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }))`
   
   Update the `JWT_SECRET` value in your `.env` file with the generated secret.

You can configure FlowMason using environment variables:

- `JWT_SECRET`: **Required** - Secret key for JWT token signing (generate a secure random string, minimum 32 characters)
- `DATABASE_URL`: Database connection string (default: `sqlite://flowmason.db`)
- `DATABASE_MAX_CONNECTIONS`: Maximum database connections (default: 10)
- `DATABASE_MIN_CONNECTIONS`: Minimum database connections (default: 2)

### Database

FlowMason uses SQLite by default. The database file (`flowmason.db`) will be created automatically in the project root when you first run the API server.

## Next Steps

- Read about [Core Concepts](concepts.md) to understand how FlowMason works
- Explore the [Bricks documentation](bricks/) to learn about available integrations
- Check out [Examples](examples.md) for workflow ideas
- Review the [API Reference](api/overview.md) for programmatic access

## Troubleshooting

### Port Already in Use

If port 3000 or 8080 is already in use, you can:

- Stop the conflicting service
- Change the port in the configuration
- Use a different port by setting environment variables (for Dioxus: `dx serve --port 8081`)

### Database Errors

If you encounter database errors:

- Ensure SQLite is installed
- Check file permissions in the project directory
- Delete `flowmason.db` to reset the database (this will delete all data)

### Build Errors

If you encounter build errors:

- Ensure you have the latest Rust toolchain: `rustup update`
- Clean and rebuild: `cargo clean && cargo build`
- Check that all dependencies are installed

