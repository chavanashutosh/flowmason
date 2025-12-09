use sha2::{Sha256, Digest};
use hex;

pub struct ApiKeyService;

impl ApiKeyService {
    pub fn generate() -> String {
        let key = format!("fm_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        key
    }

    pub fn hash(key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify(key: &str, hash: &str) -> bool {
        Self::hash(key) == hash
    }

    pub fn validate_format(key: &str) -> bool {
        key.starts_with("fm_") && key.len() > 10
    }
}

