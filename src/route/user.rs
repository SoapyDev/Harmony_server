use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;
use crate::route::acquire_connection;
use crate::schema::user::{Connection, Token, User, UserLogin, UserToken};
use crate::schema::validate_token;

pub(crate) async fn delete_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;
    match payload.delete_user(conn).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
pub(crate) async fn get_users(State(pool): State<Arc<MySqlPool>>, payload: Json<Token>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;

    return if let Ok(value) = validate_token(conn, &payload.Token).await {
        let conn = acquire_connection(pool.clone()).await?;
        match value.Role.as_str() {
            "Admin" | "Dev" => User::get_users(conn, value.Username).await,
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string())),
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid token".to_string()))
    };
}
pub(crate) async fn login(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserLogin>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;
    let user = payload.get_user(conn).await?;

    if user.validate_password(&payload.Password).await{
        return Connection::get_or_create_connection(pool.clone(), user).await
    }
    Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
}

pub(crate) async fn create_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<Vec<u8>, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;
    match payload.create_user(conn).await{
        Ok(val) => Ok(val),
        Err(_) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".to_string()))
        }
    }
}

pub(crate) async fn update_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;

    match payload.update_user(conn).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
