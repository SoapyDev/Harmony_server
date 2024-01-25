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

    let stats = if let Ok(value) = validate_token(conn, &payload.Token).await{
        let conn = acquire_connection(pool.clone()).await?;
        match value.Role.as_str() {
            "Admin" | "Dev" => Stats::get_stats(conn).await,
            _ => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string()));
            }
        }
    }else{
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid token".to_string()));
    };

    match stats {
        Ok(b) => {
            Ok(b)
        },
        Err(e) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
