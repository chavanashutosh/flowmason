use anyhow::Result;
use sqlx::{SqlitePool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let mut options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);
    
    // Configure pool size from environment or use defaults
    // Reduced default for SQLite as it handles concurrency better with fewer connections
    let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5); // Reduced from 10 to 5 for better SQLite concurrency
    
    let min_connections = std::env::var("DATABASE_MIN_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1); // Reduced from 2 to 1
    
    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect_with(options)
        .await?;
    
    // Enable WAL mode for better concurrency (allows concurrent reads and writes)
    sqlx::query("PRAGMA journal_mode=WAL;")
        .execute(&pool)
        .await?;
    
    // Initialize schema
    init_schema(&pool).await?;
    
    Ok(pool)
}

async fn init_schema(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS flows (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            bricks TEXT NOT NULL,
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS executions (
            execution_id TEXT PRIMARY KEY,
            flow_id TEXT NOT NULL,
            status TEXT NOT NULL,
            started_at TEXT NOT NULL,
            completed_at TEXT,
            input_payload TEXT NOT NULL,
            output_payload TEXT,
            error TEXT
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS usage_logs (
            id TEXT PRIMARY KEY,
            brick_name TEXT NOT NULL,
            flow_id TEXT NOT NULL,
            execution_id TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            cost_unit REAL NOT NULL,
            token_usage INTEGER,
            metadata TEXT
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS api_keys (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            key_hash TEXT NOT NULL UNIQUE,
            name TEXT,
            created_at TEXT NOT NULL,
            last_used_at TEXT,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quotas (
            brick_type TEXT PRIMARY KEY,
            daily_limit INTEGER NOT NULL,
            monthly_limit INTEGER,
            current_daily_usage INTEGER NOT NULL DEFAULT 0,
            current_monthly_usage INTEGER,
            last_reset_date TEXT
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS scheduled_flows (
            id TEXT PRIMARY KEY,
            flow_id TEXT NOT NULL UNIQUE,
            cron_expression TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (flow_id) REFERENCES flows(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS execution_data (
            id TEXT PRIMARY KEY,
            execution_id TEXT NOT NULL,
            brick_index INTEGER NOT NULL,
            brick_type TEXT NOT NULL,
            data_type TEXT NOT NULL,
            data_key TEXT NOT NULL,
            data_value TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (execution_id) REFERENCES executions(execution_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_execution_data_execution_id 
        ON execution_data(execution_id)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_execution_data_brick_index 
        ON execution_data(execution_id, brick_index)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_execution_data_data_type 
        ON execution_data(data_type)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_execution_data_execution_id_data_type 
        ON execution_data(execution_id, data_type)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_executions_started_at 
        ON executions(started_at)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_usage_logs_execution_id 
        ON usage_logs(execution_id)
        "#
    )
    .execute(pool)
    .await?;

    // Add missing indexes for query optimization
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_executions_flow_id 
        ON executions(flow_id)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_usage_logs_flow_id 
        ON usage_logs(flow_id)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_usage_logs_brick_timestamp 
        ON usage_logs(brick_name, timestamp)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS templates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            category TEXT NOT NULL DEFAULT 'General',
            flow_config TEXT NOT NULL,
            is_system INTEGER NOT NULL DEFAULT 0,
            created_by TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (created_by) REFERENCES users(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_templates_category 
        ON templates(category)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_templates_is_system 
        ON templates(is_system)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_templates_created_by 
        ON templates(created_by)
        "#
    )
    .execute(pool)
    .await?;

    // Create audit_logs table for security audit trail
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            action TEXT NOT NULL,
            resource_type TEXT NOT NULL,
            resource_id TEXT NOT NULL,
            details TEXT,
            ip_address TEXT,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id 
        ON audit_logs(user_id)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_audit_logs_action 
        ON audit_logs(action)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp 
        ON audit_logs(timestamp)
        "#
    )
    .execute(pool)
    .await?;

    // Create failed_executions table for dead letter queue
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS failed_executions (
            id TEXT PRIMARY KEY,
            execution_id TEXT NOT NULL,
            flow_id TEXT NOT NULL,
            error_message TEXT NOT NULL,
            retry_count INTEGER NOT NULL DEFAULT 0,
            max_retries INTEGER NOT NULL DEFAULT 3,
            last_attempt_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (execution_id) REFERENCES executions(execution_id),
            FOREIGN KEY (flow_id) REFERENCES flows(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_failed_executions_flow_id 
        ON failed_executions(flow_id)
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_failed_executions_retry_count 
        ON failed_executions(retry_count)
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

