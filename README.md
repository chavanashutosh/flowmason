# FlowMason

A Rust-based visual automation platform.

## How to Run the Project

### Prerequisites

- Rust 1.70+ ([install Rust](https://rustup.rs/))
- Node.js 18+ ([install Node.js](https://nodejs.org/))

### Start the API Server

```bash
cd services/api
cargo run
```

The API server will start on **http://localhost:3000**

### Start the Web UI

In a new terminal:

```bash
cd web-ui
npm install
npm run dev
```

The Web UI will be available on **http://localhost:3001** (or 3000 if available)

### Access Points

- **API Server**: http://localhost:3000
- **Web UI**: http://localhost:3001
