# Building FlowMason Web UI (Dioxus)

## Prerequisites

- Rust 1.70+ ([install Rust](https://rustup.rs/))
- Dioxus CLI: `cargo install dioxus-cli`

## Development

Run the development server with hot reloading:

```bash
cd services/web-ui
dx serve --platform web
```

The UI will be available at **http://localhost:8080**

**Important**: Always run Dioxus commands from the `services/web-ui` directory!

## Production Build

Build the optimized production bundle:

```bash
cd services/web-ui
dx build --release --platform web
```

The built files will be in `services/web-ui/dist/`

## Integration with API Server

The API server (`services/api`) is configured to serve static files from `services/web-ui/dist/` when they exist.

1. Build the Dioxus app: `cd services/web-ui && dx build --release`
2. Start the API server: `cd services/api && cargo run`
3. Access the UI at **http://localhost:3000**

## Project Structure

- `src/` - Source code
  - `main.rs` - Entry point
  - `app.rs` - Main app component
  - `router.rs` - Route definitions
  - `api.rs` - API client
  - `components/` - Reusable components
  - `pages/` - Page components
- `Dioxus.toml` - Dioxus configuration
- `index.html` - HTML template
- `dist/` - Build output (generated)

## Notes

- The UI uses Tailwind CSS via CDN (configured in `index.html`)
- API calls are made to `/api/v1/*` endpoints
- All routes are client-side routed using Dioxus Router

