use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;
use crate::route::acquire_connection;
use crate::schema::user::{Connection, Token, User, UserLogin, UserToken};
use crate::schema::validate_token;

pub(crate) async fn delete_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<StatusCode, (StatusCode, String)>{
    println!();
    println!("->> {:>12} - Delete User", "Handler");
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;
    match payload.delete_user(conn).await {
        Ok(_) => {
            println!("->> {:>12} - Delete User - SUCCESS", "Handler");
            Ok(StatusCode::OK)
        },
        Err(e) => {
            println!("->> {:>12} - Delete User - FAILED : {}", "Handler", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not delete user".to_string()))
        },
    }
}
pub(crate) async fn get_users(State(pool): State<Arc<MySqlPool>>, payload: Json<Token>) -> Result<Vec<u8>, (StatusCode, String)> {
    println!();
    println!("->> {:>12} - Get Users", "Handler");
    let conn = acquire_connection(pool.clone()).await?;

    return match validate_token(conn, &payload.Token).await {
        Ok(user) => {
            let conn = acquire_connection(pool.clone()).await?;
            match user.Role.as_str() {
                "Admin" | "Dev" => {
                    match User::get_users(conn, user.Username).await {
                        Ok(val) => {
                            println!("->> {:>12} - Get Users - SUCCESS", "Handler");
                            Ok(val)
                        },
                        Err(e) => {
                            println!("->> {:>12} - Get Users - FAILED - {:?}", "Handler", e);
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get users".to_string()))
                        }
                    }
                },
                _ => {
                    println!("->> {:>12} - Get Users - FAILED - Invalid Role", "Handler");
                    Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
                },
            }
        }
        Err(_) => {
            println!("->> {:>12} - Get Users - FAILED - Invalid token", "Handler");
            Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
        }
    }
}
pub(crate) async fn login(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserLogin>) -> Result<Vec<u8>, (StatusCode, String)> {
    println!();
    println!("->> {:>12} - Login", "Handler");
    let conn = acquire_connection(pool.clone()).await?;
    let user = payload.get_user(conn).await?;

    match user.validate_password(&payload.Password).await {
        true => {
            match Connection::get_or_create_connection(pool.clone(), user).await {
                Ok(val) => {
                    println!("->> {:>12} - Login - SUCCESS", "Handler");
                    Ok(val)
                },
                Err(e) => {
                    println!("->> {:>12} - Login - FAILED : {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create token".to_string()))
                }
            }
        },
        false => {
            println!("->> {:>12} - Login - FAILED : Invalid credentials", "Handler");
            Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
        }
    }
}

pub(crate) async fn create_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<Vec<u8>, (StatusCode, String)>{
    println!();
    println!("->> {:>12} - Create User", "Handler");
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;
    match payload.create_user(conn).await{
        Ok(val) => {
            println!("->> {:>12} - Create User - SUCCESS", "Handler");
            Ok(val)
        },
        Err(e) => {
            println!("->> {:>12} - Create User - FAILED : {:?}", "Handler", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".to_string()))
        }
    }
}

pub(crate) async fn update_user(State(pool) : State<Arc<MySqlPool>>, payload: Json<UserToken>) -> Result<StatusCode, (StatusCode, String)>{
    println!();
    println!("->> {:>12} - Update User", "Handler");
    let conn = acquire_connection(pool.clone()).await?;

    if validate_token(conn, &payload.Token).await.is_err(){
        return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
    }

    let conn = acquire_connection(pool.clone()).await?;

    match payload.update_user(conn).await {
        Ok(_) => {
            println!("->> {:>12} - Update User - SUCCESS", "Handler");
            Ok(StatusCode::OK)
        },
        Err(_e) => {
            println!("->> {:>12} - Update User - FAILED", "Handler");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not update user".to_string()))
        },
    }
}
