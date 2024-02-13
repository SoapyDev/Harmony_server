#![allow(non_snake_case)]

pub(crate) mod beneficiary;
pub(crate) mod user;
pub(crate) mod stats;
pub(crate) mod details;
pub(crate) mod category;

use std::fmt::Display;
use axum::http::StatusCode;
use bincode::{config, Encode};

use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::schema::user::UserRole;
use base64::{Engine as _, engine::general_purpose};
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use chacha20poly1305::aead::Aead;
use rand::Rng;

enum TokenValidation{
    ValidateToken
}

impl Display for TokenValidation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            TokenValidation::ValidateToken => write!(f, "
                SELECT User.Role, User.Username
                FROM UserSession
                INNER JOIN User ON
                UserSession.UserId = User.Id where Token = ? and Now() < UserSession.Expires
            "),
        }
    }
}

pub(crate) async fn validate_token(mut conn : PoolConnection<MySql>, token: &String) -> Result<UserRole, anyhow::Error> {
    println!();
    println!("->> {:>12} - Token validation : {}", "Handler", token);
    let user_token = sqlx::query_as(&TokenValidation::ValidateToken.to_string())
        .bind(token)
        .fetch_optional(conn.as_mut())
        .await.map_err(|e| {
            println!("->> {:>12} - Token validation - FAILED : {}", "Handler", e);
            e
        })?;
    
    match user_token { 
        Some(user_token) => {
            println!("->> {:>12} - Token validation - SUCCESS", "Handler");
            Ok(user_token)
        },
        None => {
            println!("->> {:>12} - Token validation - FAILED : Token not found", "Handler");
            Err(anyhow::anyhow!("Token not found"))
        }
    }
}
pub(crate) fn encode<T: Encode>(data: T) -> Result<Vec<u8>, (StatusCode, String)>{
    let config = config::standard();
    match bincode::encode_to_vec(data, config){
        Ok(encoded) => Ok(encoded),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode data".to_string())),
    }
}

pub(crate) fn encrypt(plaintext: &[u8]) -> String {
    let env_key = dotenv::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key_bytes = general_purpose::STANDARD.decode(env_key).expect("failed to decode key");
    let key = Key::from_slice(&key_bytes);

    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).expect("failed to encrypt content");

    let nonce_b64 = general_purpose::STANDARD.encode(nonce_bytes);
    let ciphertext_b64 = general_purpose::STANDARD.encode(ciphertext);

    format!("{}:{}", nonce_b64, ciphertext_b64)
}

