#![allow(non_snake_case)]


pub(crate) mod beneficiary;
pub(crate) mod user;
pub(crate) mod stats;

use axum::http::StatusCode;
use bincode::{config, Encode};

use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::schema::user::UserRole;

pub(crate) async fn validate_token(mut conn : PoolConnection<MySql>, token: &String) -> Result<UserRole, anyhow::Error> {
    let user_token = sqlx::query_as("
            SELECT User.Role, User.Username
            FROM UserSession
            INNER JOIN User ON
            UserSession.UserId = User.Id where session = ? and Now() < UserSession.Expires"
    ).bind(token)
        .fetch_one(conn.as_mut())
        .await?;

    Ok(user_token)
}
pub(crate) fn encode<T: Encode>(data: T) -> Result<Vec<u8>, (StatusCode, String)>{
    let config = config::standard();
    match bincode::encode_to_vec(data, config){
        Ok(encoded) => Ok(encoded),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

