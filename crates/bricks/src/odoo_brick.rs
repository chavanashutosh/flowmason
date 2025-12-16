use async_trait::async_trait;
use flowmason_core::{Brick, BrickError, BrickType};
use serde_json::{json, Value};
use crate::http_client::{get_client, execute_with_default_retry};

pub struct OdooBrick;

#[async_trait]
impl Brick for OdooBrick {
    fn name(&self) -> &'static str {
        "odoo"
    }

    fn brick_type(&self) -> BrickType {
        BrickType::Odoo
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "Odoo instance URL"
                },
                "database": {
                    "type": "string",
                    "description": "Database name"
                },
                "username": {
                    "type": "string",
                    "description": "Username"
                },
                "password": {
                    "type": "string",
                    "description": "Password"
                },
                "operation": {
                    "type": "string",
                    "enum": ["get_invoices", "create_invoice", "get_products"],
                    "description": "Operation to perform",
                    "default": "get_invoices"
                }
            },
            "required": ["url", "database", "username", "password", "operation"]
        })
    }

    async fn execute(&self, input: Value, config: Value) -> Result<Value, BrickError> {
        let url = config
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("url is required".to_string()))?;
        let database = config
            .get("database")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("database is required".to_string()))?;
        let username = config
            .get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("username is required".to_string()))?;
        let password = config
            .get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BrickError::ConfigError("password is required".to_string()))?;

        let operation = config
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("get_invoices");

        // Authenticate first
        let uid = self.authenticate(url, database, username, password).await?;

        match operation {
            "get_invoices" => self.get_invoices(url, database, uid, password).await,
            "create_invoice" => self.create_invoice(url, database, uid, password, input).await,
            "get_products" => self.get_products(url, database, uid, password).await,
            _ => Err(BrickError::ConfigError(format!("Unknown operation: {}", operation))),
        }
    }
}

impl OdooBrick {
    async fn authenticate(&self, url: &str, database: &str, username: &str, password: &str) -> Result<i64, BrickError> {
        let client = get_client();
        let auth_url = format!("{}/xmlrpc/2/common", url.trim_end_matches('/'));
        
        let xml_body = format!(
            r#"<?xml version="1.0"?>
<methodCall>
    <methodName>authenticate</methodName>
    <params>
        <param><value><string>{}</string></value></param>
        <param><value><string>{}</string></value></param>
        <param><value><string>{}</string></value></param>
        <param><value><struct></struct></value></param>
    </params>
</methodCall>"#,
            database, username, password
        );

        let response = execute_with_default_retry(
            client
                .post(&auth_url)
                .header("Content-Type", "text/xml")
                .body(xml_body)
        )
        .await
        .map_err(|e| BrickError::ExecutionError(format!("Failed to connect to Odoo: {}", e)))?;

        if !response.status().is_success() {
            return Err(BrickError::ExecutionError(format!(
                "Odoo authentication failed: {}",
                response.status()
            )));
        }

        let text = response.text().await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to read Odoo response: {}", e)))?;
        
        // Parse XML-RPC response - extract integer value
        // Simple parsing - in production use proper XML-RPC library
        if text.contains("<value><i4>") || text.contains("<value><int>") {
            // Extract the integer value
            let uid_str = text
                .split("<i4>")
                .nth(1)
                .or_else(|| text.split("<int>").nth(1))
                .and_then(|s| s.split("</i4>").next().or_else(|| s.split("</int>").next()))
                .ok_or_else(|| BrickError::ExecutionError("Failed to parse authentication response".to_string()))?;
            
            uid_str.parse::<i64>()
                .map_err(|_| BrickError::ExecutionError("Invalid authentication response".to_string()))
        } else if text.contains("<fault>") {
            Err(BrickError::ExecutionError("Authentication failed: Invalid credentials".to_string()))
        } else {
            Err(BrickError::ExecutionError("Unexpected authentication response format".to_string()))
        }
    }

    async fn call_method(&self, url: &str, database: &str, uid: i64, password: &str, model: &str, method: &str, args: Vec<Value>) -> Result<Value, BrickError> {
        let client = get_client();
        let object_url = format!("{}/xmlrpc/2/object", url.trim_end_matches('/'));
        
        // Convert args to XML-RPC format (simplified)
        let args_xml = args.iter().map(|arg| {
            match arg {
                Value::String(s) => format!("<value><string>{}</string></value>", s),
                Value::Number(n) => format!("<value><i4>{}</i4></value>", n),
                Value::Bool(b) => format!("<value><boolean>{}</boolean></value>", if *b { 1 } else { 0 }),
                Value::Array(arr) => {
                    let items: Vec<String> = arr.iter().map(|v| {
                        match v {
                            Value::String(s) => format!("<value><string>{}</string></value>", s),
                            Value::Number(n) => format!("<value><i4>{}</i4></value>", n),
                            _ => "<value><string></string></value>".to_string(),
                        }
                    }).collect();
                    format!("<value><array><data>{}</data></array></value>", items.join(""))
                },
                _ => "<value><string></string></value>".to_string(),
            }
        }).collect::<Vec<_>>().join("");

        let xml_body = format!(
            r#"<?xml version="1.0"?>
<methodCall>
    <methodName>execute_kw</methodName>
    <params>
        <param><value><string>{}</string></value></param>
        <param><value><i4>{}</i4></value></param>
        <param><value><string>{}</string></value></param>
        <param><value><string>{}</string></value></param>
        <param><value><string>{}</string></value></param>
        <param><value><array><data>{}</data></array></value></param>
        <param><value><struct></struct></value></param>
    </params>
</methodCall>"#,
            database, uid, password, model, method, &args_xml
        );

        let response = execute_with_default_retry(
            client
                .post(&object_url)
                .header("Content-Type", "text/xml")
                .body(xml_body)
        )
        .await
        .map_err(|e| BrickError::ExecutionError(format!("Failed to call Odoo method: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BrickError::ExecutionError(format!(
                "Odoo API error ({}): {}",
                status, error_text
            )));
        }

        let text = response.text().await
            .map_err(|e| BrickError::ExecutionError(format!("Failed to read Odoo response: {}", e)))?;

        // Parse response - convert to JSON format
        // This is simplified - in production use proper XML-RPC parsing
        if text.contains("<fault>") {
            return Err(BrickError::ExecutionError("Odoo method call failed".to_string()));
        }

        // For now, return a simplified response
        // In production, properly parse XML-RPC response
        Ok(json!({
            "result": "success",
            "raw_response": text
        }))
    }

    async fn get_invoices(&self, url: &str, database: &str, uid: i64, password: &str) -> Result<Value, BrickError> {
        // Search for invoices
        let args = vec![
            json!([["state", "!=", "cancel"]]), // domain
            json!(["name", "partner_id", "amount_total", "date", "state"]), // fields
        ];
        
        self.call_method(url, database, uid, password, "account.move", "search_read", args).await
    }

    async fn create_invoice(&self, url: &str, database: &str, uid: i64, password: &str, input: Value) -> Result<Value, BrickError> {
        // Extract invoice data from input
        let invoice_data = if input.get("invoice_data").is_some() {
            input["invoice_data"].clone()
        } else {
            input.clone()
        };

        let args = vec![
            json!([invoice_data]), // values
        ];
        
        self.call_method(url, database, uid, password, "account.move", "create", args).await
    }

    async fn get_products(&self, url: &str, database: &str, uid: i64, password: &str) -> Result<Value, BrickError> {
        // Search for products
        let args = vec![
            json!([[]]), // domain (all products)
            json!(["name", "list_price"]), // fields
        ];
        
        self.call_method(url, database, uid, password, "product.product", "search_read", args).await
    }
}

