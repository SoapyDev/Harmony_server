use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{Error, MySql, Pool};
use crate::route::get_routes;

mod schema;
mod route;
mod test;

#[tokio::main]
async fn main() {
    run().await;
}

pub async fn run() {
    let database_url = get_db_url();
    let pool = get_pool(database_url).await.unwrap();
    let app = get_routes(Arc::new(pool));
    let addr = SocketAddr::from(([192, 168, 2, 23], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub(crate) fn get_db_url() -> String{
    dotenv().ok();
    format!(
        "mysql://{}:{}@{}:{}/{}",
        dotenv::var("DB_USER").expect("DB_USER must be set"),
        dotenv::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        dotenv::var("DB_HOST").expect("DB_HOST must be set"),
        dotenv::var("DB_PORT").expect("DB_PORT must be set"),
        dotenv::var("DB_NAME").expect("DB_NAME must be set")
    )
}

pub(crate) async fn get_pool(url: String) -> Result<Pool<MySql>, Error>{
    let mut i = 0;
    while let Err(e) = get_pool_once(&url).await{
        if i > 5{
            return Err(e);
        }
        println!("Failed to connect to database: {}", e);
        println!("Retrying in 2 seconds");
        tokio::time::sleep(Duration::from_secs(3)).await;
        i += 1;
    }
    Ok(get_pool_once(&url).await.unwrap())
}

async fn get_pool_once(url: &str) -> Result<Pool<MySql>, Error> {
    MySqlPoolOptions::new()
        .min_connections(5)
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect(url)
        .await
}