-- Create failed_executions table for dead letter queue
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
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_failed_executions_flow_id ON failed_executions(flow_id);
CREATE INDEX IF NOT EXISTS idx_failed_executions_retry_count ON failed_executions(retry_count);
CREATE INDEX IF NOT EXISTS idx_failed_executions_created_at ON failed_executions(created_at);
