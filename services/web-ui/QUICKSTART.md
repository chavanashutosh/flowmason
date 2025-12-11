# Quick Start Guide - FlowMason Dioxus UI

## Prerequisites

1. **Rust** (1.70+): [Install Rust](https://rustup.rs/)
2. **Dioxus CLI**: 
   ```bash
   cargo install dioxus-cli
   ```

## Development Mode

Run the Dioxus development server with hot reloading:

```bash
cd services/web-ui
dx serve --platform web
```

The UI will be available at **http://localhost:8080**

**Note**: Make sure you're in the `services/web-ui` directory, not `services/api`!

The dev server automatically:
- Rebuilds on file changes
- Hot reloads the browser
- Provides helpful error messages

## Production Build

Build the optimized production bundle:

```bash
cd services/web-ui
dx build --release --platform web
```

This creates optimized WASM files in `services/web-ui/dist/`

## Running with API Server

### Option 1: Separate Servers (Development)

1. **Terminal 1** - Start API server:
   ```bash
   cd services/api
   cargo run
   ```
   API available at http://localhost:3000

2. **Terminal 2** - Start Dioxus dev server:
   ```bash
   cd services/web-ui
   dx serve --platform web
   ```
   UI available at http://localhost:8080

### Option 2: Integrated (Production)

1. Build the Dioxus app:
   ```bash
   cd services/web-ui
   dx build --release --platform web
   ```

2. Start the API server (it will serve the built UI):
   ```bash
   cd services/api
   cargo run
   ```

3. Access everything at **http://localhost:3000**

## Troubleshooting

### Build Errors

If you see compilation errors:
```bash
cd services/web-ui
cargo clean
cargo check
```

### Port Already in Use

If port 8080 is taken, Dioxus will try the next available port. Check the console output.

### API Connection Issues

Make sure the API server is running and accessible. The UI makes requests to `/api/v1/*` endpoints.

### Static Files Not Found

If the API server can't find the built files:
1. Verify `services/web-ui/dist/` exists after building
2. Check the path in `services/api/src/server.rs`
3. Ensure you're running from the correct directory

## Project Structure

```
services/web-ui/
├── src/              # Source code
│   ├── main.rs       # Entry point
│   ├── app.rs        # Main app component
│   ├── router.rs     # Routes
│   ├── api.rs        # API client
│   ├── components/   # Reusable components
│   └── pages/        # Page components
├── Dioxus.toml       # Dioxus configuration
├── index.html        # HTML template
└── dist/             # Build output (generated)
```

## Next Steps

- See `BUILD.md` for detailed build instructions
- See `MIGRATION_SUMMARY.md` for migration details
- Check the main `README.md` for overall project setup

