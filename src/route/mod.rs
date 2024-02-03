mod beneficiary;
mod user;
mod stats;

use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{delete, get, post};
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::pool::PoolConnection;


use crate::route::user::{create_user, delete_user, get_users, login, update_user};
use crate::route::stats::stats;
use crate::route::beneficiary::{beneficiaries, beneficiary, create_beneficiary, delete_allergy, delete_presence, insert_allergy, insert_presence, search_beneficiaries, update_beneficiary};

pub fn get_routes(pool : Arc<Pool<MySql>>) -> Router{
    Router::new()
        .route("/", get(test_connection)).with_state(pool.clone())
        .route("/login", post(login)).with_state(pool.clone())
        .route("/getUsers", post(get_users)).with_state(pool.clone())
        .route("/createUser", post(create_user)).with_state(pool.clone())
        .route("/updateUser", post(update_user)).with_state(pool.clone())
        .route("/deleteUser", delete(delete_user)).with_state(pool.clone())
        .route("/getBeneficiary", post(beneficiary)).with_state(pool.clone())
        .route("/getBeneficiaries", post(beneficiaries)).with_state(pool.clone())
        .route("/searchBeneficiaries", post(search_beneficiaries)).with_state(pool.clone())
        .route("/createBeneficiary", post(create_beneficiary)).with_state(pool.clone())
        .route("/updateBeneficiary", post(update_beneficiary)).with_state(pool.clone())
        .route("/insertAllergy", post(insert_allergy)).with_state(pool.clone())
        .route("/deleteAllergy", post(delete_allergy)).with_state(pool.clone())
        .route("/insertPresence", post(insert_presence)).with_state(pool.clone())
        .route("/deletePresence", post(delete_presence)).with_state(pool.clone())
        .route("/getStats", post(stats)).with_state(pool.clone())
}



async fn test_connection(State(pool): State<Arc<MySqlPool>>) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = acquire_connection(pool.clone()).await?;

    match sqlx::query("SELECT 1")
        .fetch_one(conn.as_mut())
        .await {
            Ok(_) => Ok(StatusCode::OK),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}



pub async fn acquire_connection(pool: Arc<MySqlPool>) -> Result<PoolConnection<MySql>, (StatusCode, String)>{
    match pool.acquire().await {
        Ok(conn) => Ok(conn) ,
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
