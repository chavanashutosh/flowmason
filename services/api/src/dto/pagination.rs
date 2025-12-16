use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default = "default_offset")]
    pub offset: u32,
}

fn default_limit() -> u32 {
    100
}

fn default_offset() -> u32 {
    0
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub limit: u32,
    pub offset: u32,
    pub total: Option<u64>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, limit: u32, offset: u32) -> Self {
        Self {
            items,
            limit,
            offset,
            total: None,
        }
    }

    pub fn with_total(items: Vec<T>, limit: u32, offset: u32, total: u64) -> Self {
        Self {
            items,
            limit,
            offset,
            total: Some(total),
        }
    }
}
