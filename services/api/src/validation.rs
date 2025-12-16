use regex::Regex;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(
        r"^https?://([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}(/.*)?$"
    ).unwrap();
}

/// Validates a URL format
pub fn validate_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
}

/// Validates webhook URL with optional whitelist
pub fn validate_webhook_url(url: &str, whitelist: Option<&HashSet<String>>) -> Result<(), String> {
    // Basic URL format validation
    if !validate_url(url) {
        return Err("Invalid URL format".to_string());
    }

    // Check whitelist if provided
    if let Some(whitelist) = whitelist {
        let domain = extract_domain(url)?;
        if !whitelist.contains(&domain) {
            return Err(format!("URL domain '{}' is not in whitelist", domain));
        }
    }

    // Security: Only allow HTTPS in production (can be configured)
    let allow_http = std::env::var("ALLOW_HTTP_WEBHOOKS")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if !allow_http && !url.starts_with("https://") {
        return Err("Only HTTPS URLs are allowed for webhooks".to_string());
    }

    Ok(())
}

/// Extracts domain from URL
fn extract_domain(url: &str) -> Result<String, String> {
    let url_obj = url::Url::parse(url)
        .map_err(|_| "Invalid URL format".to_string())?;
    
    Ok(url_obj.host_str()
        .ok_or("No host in URL")?
        .to_string())
}

/// Validates email format
pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex.is_match(email)
}

/// Validates cron expression format
pub fn validate_cron_expression(cron: &str) -> Result<(), String> {
    let parts: Vec<&str> = cron.split_whitespace().collect();
    
    if parts.len() != 5 {
        return Err("Cron expression must have exactly 5 parts".to_string());
    }

    // Basic validation - each part should be valid
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            return Err(format!("Part {} of cron expression is empty", i + 1));
        }
        
        // Check for valid characters: numbers, *, -, /, ,
        if !part.chars().all(|c| c.is_ascii_digit() || matches!(c, '*' | '-' | '/' | ',')) {
            return Err(format!("Invalid character in part {} of cron expression", i + 1));
        }
    }

    Ok(())
}
