use crate::models::user::{Claims, UserCredentials};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AuthService {
    secret: Vec<u8>,
}

impl AuthService {
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }

    pub fn validate_credentials(&self, credentials: &UserCredentials) -> bool {
        credentials.username == "richard" && credentials.password == "secret"
    }

    pub fn create_jwt(&self, username: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: username.to_string(),
            exp: now + 3600,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .unwrap()
    }

    pub fn validate_jwt(&self, token: &str) -> Option<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .ok()
    }
}
