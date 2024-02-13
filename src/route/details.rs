use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;
use crate::route::acquire_connection;
use crate::schema::details::{TokenAllergy, TokenNote, TokenPresence};
use crate::schema::validate_token;

pub(crate) async fn insert_allergy(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenAllergy>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.insert_allergy(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn delete_allergy(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenAllergy>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) =>{
            let conn = acquire_connection(pool.clone()).await?;
            match payload.delete_allergy(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}
pub(crate) async fn insert_presence(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenPresence>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) =>{
            let conn = acquire_connection(pool.clone()).await?;
            match payload.insert_presence(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}


pub(crate) async fn delete_presence(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenPresence>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
                let conn = acquire_connection(pool.clone()).await?;
                match payload.delete_presence(conn).await {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                }
            },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn create_note(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenNote>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.create_note(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create note".to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn update_note(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenNote>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.update_note(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not update note".to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn delete_note(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenNote>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.delete_note(conn).await {
                Ok(_) => Ok(StatusCode::OK),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not delete note".to_string()))
            }
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}