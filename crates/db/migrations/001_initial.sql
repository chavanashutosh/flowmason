-- Create flows table
CREATE TABLE IF NOT EXISTS flows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    bricks TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create executions table
CREATE TABLE IF NOT EXISTS executions (
    execution_id TEXT PRIMARY KEY,
    flow_id TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    input_payload TEXT NOT NULL,
    output_payload TEXT,
    error TEXT,
    FOREIGN KEY (flow_id) REFERENCES flows(id)
);

-- Create usage_logs table
CREATE TABLE IF NOT EXISTS usage_logs (
    id TEXT PRIMARY KEY,
    brick_name TEXT NOT NULL,
    flow_id TEXT NOT NULL,
    execution_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    cost_unit REAL NOT NULL,
    token_usage INTEGER,
    metadata TEXT,
    FOREIGN KEY (flow_id) REFERENCES flows(id)
);

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create api_keys table
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    key_hash TEXT NOT NULL UNIQUE,
    name TEXT,
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Create quotas table
CREATE TABLE IF NOT EXISTS quotas (
    brick_type TEXT PRIMARY KEY,
    daily_limit INTEGER NOT NULL,
    monthly_limit INTEGER,
    current_daily_usage INTEGER NOT NULL DEFAULT 0,
    current_monthly_usage INTEGER,
    last_reset_date TEXT
);

-- Create scheduled_flows table
CREATE TABLE IF NOT EXISTS scheduled_flows (
    id TEXT PRIMARY KEY,
    flow_id TEXT NOT NULL UNIQUE,
    cron_expression TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (flow_id) REFERENCES flows(id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_executions_flow_id ON executions(flow_id);
CREATE INDEX IF NOT EXISTS idx_usage_logs_flow_id ON usage_logs(flow_id);
CREATE INDEX IF NOT EXISTS idx_usage_logs_timestamp ON usage_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_scheduled_flows_flow_id ON scheduled_flows(flow_id);

