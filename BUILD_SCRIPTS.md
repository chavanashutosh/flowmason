# Build Scripts

This directory contains bash scripts for building FlowMason components.

## Prerequisites

- Rust 1.70+ ([install Rust](https://rustup.rs/))
- Dioxus CLI (will be auto-installed if missing): `cargo install dioxus-cli`
- Bash shell (Git Bash, WSL, or Linux/macOS)

## Scripts

### `build.sh` - Main Build Script

Builds all components with options:

```bash
# Build everything (release mode, includes web-ui)
./build.sh

# Build in dev mode
./build.sh dev

# Build Rust only (skip web-ui)
./build.sh release false
```

### `build-all.sh` - Build Everything

Builds all components including web-ui:

```bash
./build-all.sh          # Release build
./build-all.sh dev      # Dev build
```

### `build-api.sh` - Build API Server Only

```bash
./build-api.sh          # Release build
./build-api.sh dev      # Dev build
```

### `build-web-ui.sh` - Build Web UI Only

```bash
./build-web-ui.sh       # Release build
./build-web-ui.sh dev   # Dev build
```

### `build-worker.sh` - Build Worker Only

```bash
./build-worker.sh       # Release build
./build-worker.sh dev   # Dev build
```

### `clean.sh` - Clean Build Artifacts

Removes all build artifacts:

```bash
./clean.sh
```

## Usage Examples

### Quick Start

```bash
# Build everything for production
./build.sh

# Build for development
./build.sh dev

# Build only Rust components (faster, skips web-ui)
./build.sh release false
```

### Individual Components

```bash
# Build just the API server
./build-api.sh

# Build just the web UI
./build-web-ui.sh

# Build just the worker
./build-worker.sh
```

### Clean and Rebuild

```bash
./clean.sh
./build.sh
```

## Build Outputs

### Release Builds
- API Server: `target/release/flowmason-api`
- Worker: `target/release/flowmason-worker`
- TUI: `target/release/flowmason-tui`
- UI Builder: `target/release/flowmason-ui-builder`
- Web UI: `services/web-ui/dist/`

### Dev Builds
- API Server: `target/debug/flowmason-api`
- Worker: `target/debug/flowmason-worker`
- TUI: `target/debug/flowmason-tui`
- UI Builder: `target/debug/flowmason-ui-builder`
- Web UI: `services/web-ui/dist/`

## Notes

- The scripts will automatically install Dioxus CLI if it's missing
- All scripts exit on error (`set -e`)
- The main build script includes colored output for better readability
- On Windows, use Git Bash or WSL to run these scripts
