# FlowMason Troubleshooting Guide

## Common Issues

### API Server Won't Start

**Problem**: Server fails to start on port 3000

**Solutions**:
1. Check if port 3000 is already in use:
   ```bash
   # Linux/Mac
   lsof -i :3000
   
   # Windows
   netstat -ano | findstr :3000
   ```

2. Change the port by modifying `services/api/src/server.rs` or use environment variable

3. Check database file permissions:
   ```bash
   chmod 644 flowmason.db
   ```

### Database Locked Errors

**Problem**: "database is locked" errors

**Solutions**:
1. Ensure WAL mode is enabled (should be automatic)
2. Check for long-running transactions
3. Restart the API server
4. Check disk space

### Flow Execution Fails

**Problem**: Flow execution returns error

**Check**:
1. Verify all brick configurations are correct
2. Check API keys are valid
3. Review execution logs:
   ```bash
   # Check logs with RUST_LOG=debug
   RUST_LOG=debug cargo run
   ```

4. Check quota limits:
   ```bash
   curl http://localhost:3000/api/v1/usage/stats \
     -H "Authorization: Bearer <token>"
   ```

### Authentication Issues

**Problem**: "Unauthorized" errors

**Solutions**:
1. Verify token is valid and not expired
2. Check token format: `Bearer <token>`
3. Regenerate API key if needed
4. Check JWT_SECRET is set correctly

### Webhook URL Validation Fails

**Problem**: Webhook URL rejected

**Solutions**:
1. Ensure URL starts with `https://` (or set `ALLOW_HTTP_WEBHOOKS=true`)
2. Check URL format is valid
3. Verify domain is in whitelist (if configured)

### High Memory Usage

**Problem**: Server uses too much memory

**Solutions**:
1. Reduce database connection pool size:
   ```bash
   export DATABASE_MAX_CONNECTIONS=3
   ```

2. Enable data retention cleanup:
   ```bash
   ./scripts/cleanup_old_data.sh
   ```

3. Check for memory leaks in logs

### Slow Performance

**Problem**: API responses are slow

**Solutions**:
1. Check database indexes are created
2. Vacuum database:
   ```bash
   sqlite3 flowmason.db "VACUUM;"
   ```

3. Analyze database:
   ```bash
   sqlite3 flowmason.db "ANALYZE;"
   ```

4. Check for long-running queries in logs

### Docker Build Fails

**Problem**: Docker build errors

**Solutions**:
1. Ensure Docker has enough disk space
2. Clear Docker cache:
   ```bash
   docker system prune -a
   ```

3. Check Rust version matches Dockerfile
4. Increase Docker memory limit

### Frontend Build Issues

**Problem**: Web UI won't build

**Solutions**:
1. Clear node_modules and reinstall:
   ```bash
   cd services/web-ui-vite
   rm -rf node_modules package-lock.json
   npm install
   ```

2. Check Node.js version (requires 18+)
3. Clear Vite cache:
   ```bash
   rm -rf node_modules/.vite
   ```

## Debugging

### Enable Debug Logging

Set environment variable:
```bash
export RUST_LOG=debug
```

Or for specific modules:
```bash
export RUST_LOG=flowmason_api=debug,flowmason_core=info
```

### Check Database Integrity

```bash
sqlite3 flowmason.db "PRAGMA integrity_check;"
```

### View Recent Executions

```bash
sqlite3 flowmason.db "SELECT * FROM executions ORDER BY started_at DESC LIMIT 10;"
```

### Check Scheduled Flows

```bash
sqlite3 flowmason.db "SELECT * FROM scheduled_flows;"
```

### Monitor Logs

```bash
# Follow logs
tail -f logs/flowmason.log

# Or with Docker
docker-compose logs -f api
```

## Performance Tuning

### Database Optimization

1. **Vacuum regularly**:
   ```bash
   sqlite3 flowmason.db "VACUUM;"
   ```

2. **Update statistics**:
   ```bash
   sqlite3 flowmason.db "ANALYZE;"
   ```

3. **Check index usage**:
   ```bash
   sqlite3 flowmason.db "EXPLAIN QUERY PLAN SELECT * FROM executions WHERE flow_id = 'xxx';"
   ```

### Connection Pool Tuning

Adjust in `.env`:
```bash
DATABASE_MAX_CONNECTIONS=5
DATABASE_MIN_CONNECTIONS=1
```

### Rate Limiting

Adjust in `services/api/src/server.rs`:
```rust
.per_second(100)  // Requests per second
.burst_size(200)  // Burst size
```

## Getting Help

1. Check logs with `RUST_LOG=debug`
2. Review execution history in database
3. Check health endpoint: `curl http://localhost:3000/health`
4. Review error messages in API responses
5. Check GitHub issues for similar problems
