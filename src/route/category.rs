use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;
use crate::route::acquire_connection;
use crate::schema::category::{Categories, TokenCategory};
use crate::schema::user::Token;
use crate::schema::validate_token;

pub(crate) async fn create_category(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenCategory>) -> Result<Vec<u8>, (StatusCode, String)>{
    println!("->> {:>12} - Create category", "Handler");
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.Category.create_category(conn).await {
                Ok(val) => {
                    println!("->> {:>12} - Create category - SUCCESS", "Handler");
                    Ok(val)
                }
                Err(e) => {
                    println!("->> {:>12} - Create category - FAILED : {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create a category".to_string()))
                }
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn update_category(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenCategory>) -> Result<Vec<u8>, (StatusCode, String)>{
    println!("->> {:>12} - Update category", "Handler");
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.Category.update_category(conn).await {
                Ok(val) => {
                    println!("->> {:>12} - Update category - SUCCESS", "Handler");
                    Ok(val)
                }
                Err(e) => {
                    println!("->> {:>12} - Update category - FAILED : {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not update a category".to_string()))
                }
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn delete_category(State(pool) : State<Arc<MySqlPool>>, payload: Json<TokenCategory>) -> Result<Vec<u8>, (StatusCode, String)>{
    println!("->> {:>12} - Delete category", "Handler");
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match payload.Category.delete_category(conn).await {
                Ok(val) => {
                    println!("->> {:>12} - Delete category - SUCCESS", "Handler");
                    Ok(val)
                }
                Err(e) => {
                    println!("->> {:>12} - Delete category - FAILED : {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not delete a category".to_string()))
                }
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

pub(crate) async fn select_categories(State(pool) : State<Arc<MySqlPool>>, payload: Json<Token>) -> Result<Vec<u8>, (StatusCode, String)>{
    println!("->> {:>12} - Select categories", "Handler");
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(_) => {
            let conn = acquire_connection(pool.clone()).await?;
            match Categories::select_categories(conn).await {
                Ok(val) => {
                    println!("->> {:>12} - Select categories - SUCCESS", "Handler");
                    Ok(val)
                }
                Err(e) => {
                    println!("->> {:>12} - Select categories - FAILED : {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not select categories".to_string()))
                }
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}