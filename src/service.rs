use std::env;

use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref JWT_SECRET: EncodingKey = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    role: i64,
}

pub fn generate_token(user_id: i64, role: i64) -> String {
    jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: user_id,
            role,
        },
        &JWT_SECRET,
    ).unwrap()
}