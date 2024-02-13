mod beneficiary;
mod user;
mod stats;
mod details;
mod category;

use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{delete, get, post, put};
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::pool::PoolConnection;


use crate::route::user::{create_user, delete_user, get_users, login, update_user};
use crate::route::stats::stats;
use crate::route::beneficiary::{beneficiaries, beneficiary, create_beneficiary, search_beneficiaries, update_beneficiary};
use crate::route::category::{create_category, delete_category, select_categories, update_category};
use crate::route::details::{create_note, delete_allergy, delete_note, delete_presence, insert_allergy, insert_presence, update_note};

pub fn get_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/", get(test_connection)).with_state(pool.clone())
        .merge(user_routes(pool.clone()))
        .merge(beneficiary_routes(pool.clone()))
        .merge(details_routes(pool.clone()))
        .merge(category_routes(pool.clone()))
        .merge(stats_routes(pool.clone()))
}

fn user_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/user/login", post(login)).with_state(pool.clone())
        .route("/user/select", post(get_users)).with_state(pool.clone())
        .route("/user", post(create_user)).with_state(pool.clone())
        .route("/user", put(update_user)).with_state(pool.clone())
        .route("/user", delete(delete_user)).with_state(pool.clone())
}

fn beneficiary_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/beneficiary/select/:id", post(beneficiary)).with_state(pool.clone())
        .route("/beneficiary/select", post(beneficiaries)).with_state(pool.clone())
        .route("/beneficiary/search", post(search_beneficiaries)).with_state(pool.clone())
        .route("/beneficiary", post(create_beneficiary)).with_state(pool.clone())
        .route("/beneficiary", put(update_beneficiary)).with_state(pool.clone())
}

fn details_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/allergy", post(insert_allergy)).with_state(pool.clone())
        .route("/allergy", delete(delete_allergy)).with_state(pool.clone())
        .route("/presence", post(insert_presence)).with_state(pool.clone())
        .route("/presence", delete(delete_presence)).with_state(pool.clone())
        .route("/note", post(create_note)).with_state(pool.clone())
        .route("/note", put(update_note)).with_state(pool.clone())
        .route("/note", delete(delete_note)).with_state(pool.clone())
}

fn category_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/category/select", post(select_categories)).with_state(pool.clone())
        .route("/category", post(create_category)).with_state(pool.clone())
        .route("/category", put(update_category)).with_state(pool.clone())
        .route("/category", delete(delete_category)).with_state(pool.clone())
}

fn stats_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/stats/select", post(stats)).with_state(pool.clone())
}



async fn test_connection(State(pool): State<Arc<MySqlPool>>) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = acquire_connection(pool.clone()).await?;

    match sqlx::query("SELECT 1")
        .fetch_one(conn.as_mut())
        .await {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to the Database".to_string())),
    }
}



pub async fn acquire_connection(pool: Arc<MySqlPool>) -> Result<PoolConnection<MySql>, (StatusCode, String)>{
    match pool.acquire().await {
        Ok(conn) => Ok(conn) ,
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to the Database".to_string())),
    }
}
