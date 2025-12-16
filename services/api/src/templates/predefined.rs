use anyhow::Result;
use flowmason_core::types::{Template, Flow, BrickConfig, BrickType};
use serde_json::Value;
use chrono::Utc;
use uuid::Uuid;

pub fn get_predefined_templates() -> Vec<Template> {
    vec![
        create_customer_enrichment_template(),
        create_hubspot_deal_creation_template(),
        create_ai_content_to_notion_template(),
        create_webhook_to_odoo_template(),
        create_n8n_webhook_pipeline_template(),
        create_hubspot_to_notion_template(),
    ]
}

fn create_customer_enrichment_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "Customer Data Enrichment".to_string(),
        description: Some("Process customer data, enrich with AI, and create CRM records".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "form.first_name", "target_path": "first_name" },
                        { "source_path": "form.last_name", "target_path": "last_name" },
                        { "source_path": "form.email", "target_path": "email" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::CombineText,
                config: json!({
                    "fields": ["first_name", "last_name"],
                    "separator": " ",
                    "output_field": "full_name"
                }),
            },
            BrickConfig {
                brick_type: BrickType::OpenAi,
                config: json!({
                    "api_key": "your-openai-api-key",
                    "model_name": "gpt-3.5-turbo",
                    "prompt_template": "Generate a professional company description for: {{full_name}} working in {{industry}}",
                    "temperature": 0.7,
                    "max_tokens": 200
                }),
            },
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "full_name", "target_path": "properties.firstname" },
                        { "source_path": "email", "target_path": "properties.email" },
                        { "source_path": "content", "target_path": "properties.company_description" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::HubSpot,
                config: json!({
                    "api_key": "your-hubspot-api-key",
                    "operation": "create_deal"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-customer-enrichment".to_string(),
        name: "Customer Data Processing".to_string(),
        description: Some("Map customer data and process with AI".to_string()),
        category: "Data Processing".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_hubspot_deal_creation_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "HubSpot Deal Creation".to_string(),
        description: Some("Create deals in HubSpot from form submissions".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "form.deal_name", "target_path": "properties.dealname" },
                        { "source_path": "form.amount", "target_path": "properties.amount" },
                        { "source_path": "form.stage", "target_path": "properties.dealstage" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::HubSpot,
                config: json!({
                    "api_key": "your-hubspot-api-key",
                    "operation": "create_deal"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-hubspot-deal-creation".to_string(),
        name: "HubSpot Deal Creation".to_string(),
        description: Some("Create deals in HubSpot from form submissions".to_string()),
        category: "CRM Integration".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_ai_content_to_notion_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "AI Content to Notion".to_string(),
        description: Some("Generate blog post content using AI and create Notion pages".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::OpenAi,
                config: json!({
                    "api_key": "your-openai-api-key",
                    "model_name": "gpt-4",
                    "prompt_template": "Write a blog post about {{topic}} with the following requirements:\n- Length: {{length}} words\n- Tone: {{tone}}\n- Include: {{requirements}}",
                    "temperature": 0.8,
                    "max_tokens": 2000
                }),
            },
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "topic", "target_path": "title" },
                        { "source_path": "content", "target_path": "content" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::Notion,
                config: json!({
                    "api_key": "your-notion-api-key",
                    "database_id": "your-database-id",
                    "operation": "create_page"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-ai-content-to-notion".to_string(),
        name: "AI Content Generation to Notion".to_string(),
        description: Some("Generate content with AI and save to Notion".to_string()),
        category: "Content Creation".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_webhook_to_odoo_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "Webhook to Odoo Invoice".to_string(),
        description: Some("Process webhook data and create invoices in Odoo ERP".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "webhook.customer_id", "target_path": "partner_id" },
                        { "source_path": "webhook.order_date", "target_path": "invoice_date" },
                        { "source_path": "webhook.items", "target_path": "invoice_line_ids" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::Conditional,
                config: json!({
                    "condition_field": "webhook.status",
                    "condition": "== 'paid'",
                    "true_value": "posted",
                    "false_value": "draft",
                    "output_field": "state"
                }),
            },
            BrickConfig {
                brick_type: BrickType::Odoo,
                config: json!({
                    "url": "https://your-odoo-instance.com",
                    "database": "your-database-name",
                    "username": "your-username",
                    "password": "your-password",
                    "operation": "create_invoice"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-webhook-to-odoo".to_string(),
        name: "Webhook to Odoo Invoice Creation".to_string(),
        description: Some("Create invoices in Odoo from webhook data".to_string()),
        category: "ERP Integration".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_n8n_webhook_pipeline_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "n8n Webhook Pipeline".to_string(),
        description: Some("Send data to n8n for processing and then sync results to HubSpot".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "event.type", "target_path": "event_type" },
                        { "source_path": "event.data", "target_path": "payload" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::N8n,
                config: json!({
                    "webhook_url": "https://your-n8n-instance.com/webhook/process-data",
                    "method": "POST"
                }),
            },
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "processed_data.customer_name", "target_path": "properties.dealname" },
                        { "source_path": "processed_data.amount", "target_path": "properties.amount" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::HubSpot,
                config: json!({
                    "api_key": "your-hubspot-api-key",
                    "operation": "create_deal"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-n8n-webhook-pipeline".to_string(),
        name: "n8n Webhook Pipeline".to_string(),
        description: Some("Process data through multiple services using n8n webhooks".to_string()),
        category: "Automation".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_hubspot_to_notion_template() -> Template {
    let flow = Flow {
        id: Uuid::new_v4().to_string(),
        name: "HubSpot to Notion Sync".to_string(),
        description: Some("Automatically sync HubSpot deals to a Notion database".to_string()),
        bricks: vec![
            BrickConfig {
                brick_type: BrickType::HubSpot,
                config: json!({
                    "api_key": "your-hubspot-api-key",
                    "operation": "get_deals"
                }),
            },
            BrickConfig {
                brick_type: BrickType::FieldMapping,
                config: json!({
                    "mappings": [
                        { "source_path": "results[].properties.dealname", "target_path": "title" },
                        { "source_path": "results[].properties.amount", "target_path": "amount" },
                        { "source_path": "results[].properties.dealstage", "target_path": "status" }
                    ]
                }),
            },
            BrickConfig {
                brick_type: BrickType::Notion,
                config: json!({
                    "api_key": "your-notion-api-key",
                    "database_id": "your-database-id",
                    "operation": "create_page"
                }),
            },
        ],
        active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Template {
        id: "template-hubspot-to-notion".to_string(),
        name: "HubSpot to Notion Sync".to_string(),
        description: Some("Sync HubSpot deals to Notion database".to_string()),
        category: "CRM Integration".to_string(),
        flow_config: flow,
        is_system: true,
        created_by: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

pub async fn seed_predefined_templates(repo: &flowmason_db::repositories::TemplateRepository) -> Result<()> {
    let templates = get_predefined_templates();
    
    for template in templates {
        // Check if template already exists
        if repo.get(&template.id).await?.is_none() {
            repo.create(&template).await?;
        }
    }
    
    Ok(())
}
