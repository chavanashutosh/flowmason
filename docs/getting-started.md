# Getting Started

This guide will help you get FlowMason up and running on your local machine.

## Prerequisites

Before you begin, make sure you have the following installed:

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Node.js 18+**: [Install Node.js](https://nodejs.org/)
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

### 3. Install Frontend Dependencies

```bash
cd web-ui
npm install
cd ..
```

## Running FlowMason

### Start the API Server

In the project root directory:

```bash
cd services/api
cargo run
```

The API server will start on **http://localhost:3000**

### Start the Web UI

In a new terminal:

```bash
cd web-ui
npm run dev
```

The Web UI will be available on **http://localhost:3001** (or 3000 if available)

## Access Points

- **API Server**: http://localhost:3000
- **Web UI**: http://localhost:3001
- **API Documentation**: http://localhost:3000/api/v1/bricks

## First Steps

1. **Access the Web UI**: Open http://localhost:3001 in your browser
2. **Create an Account**: Register a new user account
3. **Create Your First Flow**: Use the Flows page to create a new workflow
4. **Add Bricks**: Add bricks to your flow from the available integrations
5. **Execute**: Test your flow by executing it manually
6. **Schedule**: Set up automated execution using the Scheduler

## Configuration

### Environment Variables

You can configure FlowMason using environment variables:

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

If port 3000 or 3001 is already in use, you can:

- Stop the conflicting service
- Change the port in the configuration
- Use a different port by setting environment variables

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

