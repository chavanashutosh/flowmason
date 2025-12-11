# Troubleshooting Guide

## Common Issues

### Error: "No platform was specified and could not be auto-detected"

**Problem**: Dioxus CLI can't detect which platform to build for.

**Solution**: Always specify the platform explicitly:

```bash
cd services/web-ui
dx serve --platform web
# or
dx build --release --platform web
```

**Why**: Dioxus supports multiple platforms (web, desktop, mobile). You must specify which one to use.

### Error: "failed to read `dioxus/settings.toml` config file"

**Problem**: Running Dioxus commands from the wrong directory.

**Solution**: Make sure you're in the `services/web-ui` directory:

```bash
cd services/web-ui  # ← Important!
dx serve --platform web
```

**Check**: Verify you're in the right directory:
```bash
pwd  # Should show: .../services/web-ui
ls   # Should show: Cargo.toml, Dioxus.toml, src/, etc.
```

### Build Errors

If you see compilation errors:

1. **Clean and rebuild**:
   ```bash
   cd services/web-ui
   cargo clean
   cargo check
   ```

2. **Verify dependencies**:
   ```bash
   cargo tree
   ```

3. **Check Rust version**:
   ```bash
   rustc --version  # Should be 1.70+
   ```

### Port Already in Use

If port 8080 is taken:

- Dioxus will automatically try the next available port
- Check the console output for the actual port number
- Or specify a different port: `dx serve --platform web --port 8081`

### API Connection Issues

**Problem**: UI can't connect to API server.

**Check**:
1. Is the API server running? (`cd services/api && cargo run`)
2. Is it on port 3000?
3. Check browser console for CORS errors

**Solution**: Make sure API server is running before starting the UI.

### Static Files Not Found (Production)

**Problem**: API server can't find built Dioxus files.

**Solution**:
1. Build the Dioxus app first:
   ```bash
   cd services/web-ui
   dx build --release --platform web
   ```

2. Verify `dist/` directory exists:
   ```bash
   ls services/web-ui/dist/
   ```

3. Check the path in `services/api/src/server.rs` matches your setup

## Quick Reference

### Correct Command Sequence

**Development**:
```bash
# Terminal 1: API Server
cd services/api
cargo run

# Terminal 2: Dioxus UI
cd services/web-ui
dx serve --platform web
```

**Production**:
```bash
# Build UI
cd services/web-ui
dx build --release --platform web

# Start API (serves UI)
cd services/api
cargo run
```

## Still Having Issues?

1. Check Dioxus version: `dx --version` (should be 0.6.x)
2. Check Rust version: `rustc --version` (should be 1.70+)
3. Verify you're in the correct directory
4. Check `services/web-ui/Cargo.toml` has `[features]` section
5. Check `services/web-ui/Dioxus.toml` has `default_platform = "web"`

