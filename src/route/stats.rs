use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::MySqlPool;
use crate::route::acquire_connection;
use crate::schema::stats::Stats;
use crate::schema::user::Token;
use crate::schema::validate_token;

pub(crate) async fn stats(State(pool): State<Arc<MySqlPool>>, payload: Json<Token>) -> Result<Vec<u8>, (StatusCode, String)> {
    let conn = acquire_connection(pool.clone()).await?;
    match validate_token(conn, &payload.Token).await {
        Ok(user) => match user.Role.as_str(){
           "Dev" | "Admin" => {
                let conn = acquire_connection(pool.clone()).await?;
                Stats::get_stats(conn).await
                },
            _ => Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
        },
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}
