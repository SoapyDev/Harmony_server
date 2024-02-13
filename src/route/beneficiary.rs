use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;

use crate::route::acquire_connection;
use crate::schema::beneficiary::{Beneficiary, BeneficiaryAction};
use crate::schema::user::{Token, TokenBene, TokenBeneId, TokenSearch};
use crate::schema::validate_token;

pub(crate) async fn beneficiaries(State(pool) : State<Arc<MySqlPool>>, payload : Json<Token>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;

    if let Ok(value) = validate_token(conn, &payload.Token).await {
            Beneficiary::get_beneficiaries(acquire_connection(pool.clone()).await?, value).await
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn search_beneficiaries(State(pool) : State<Arc<MySqlPool>>, payload : Json<TokenSearch>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;

    if let Ok(value) = validate_token(conn, &payload.Token).await {
        Beneficiary::search(acquire_connection(pool.clone()).await?, value, &payload.Search).await
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn beneficiary(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenBeneId>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;

    if let Ok(user) = validate_token(conn, &payload.Token).await{
        let conn = acquire_connection(pool.clone()).await?;
        Beneficiary::get_beneficiary(conn, user, payload.Id).await
    }else{
        Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn create_beneficiary(State(pool) : State<Arc<MySqlPool>>, payload: Json<Token>) -> Result<Vec<u8>, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(user) => Beneficiary::create_beneficiary(acquire_connection(pool.clone()).await?, user).await,
        Err(_) => Err((StatusCode::UNAUTHORIZED,"Invalid token".to_string()))
    }
}

pub(crate) async fn update_beneficiary(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenBene>) -> Result<StatusCode, (StatusCode, String)>{
    let conn = acquire_connection(pool.clone()).await?;

    let res = match validate_token(conn, &payload.Token).await {
        Ok(user) => Beneficiary::update_beneficiary(acquire_connection(pool.clone()).await?, user, payload.Beneficiary.clone()).await,
        Err(_) => return Err((StatusCode::FORBIDDEN, "Invalid token".to_string())),
    };
    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
    }
}
