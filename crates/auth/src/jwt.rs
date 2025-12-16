use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::error::AuthError;
use crate::user::Claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        
        Self {
            encoding_key,
            decoding_key,
        }
    }

    pub fn generate_token(&self, user_id: &str, email: &str) -> Result<String, AuthError> {
        let now = chrono::Utc::now().timestamp();
        let exp = now + (24 * 60 * 60); // 24 hours
        
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp,
            iat: now,
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InvalidToken)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }

    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                eprintln!("⚠️  WARNING: JWT_SECRET not set! Using insecure default. Set JWT_SECRET environment variable for production.");
                "your-secret-key-change-in-production".to_string()
            });
        Self::new(secret)
    }
}

