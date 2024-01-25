use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use axum::{ serve};
use dotenv::dotenv;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{ServerConfig};
use rustls_pemfile::{certs,  pkcs8_private_keys};
use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{Error, MySql, Pool};
use tokio::io;
use tokio_rustls::TlsAcceptor;
use crate::route::get_routes;

mod schema;
mod route;

#[tokio::main]
async fn main() {
    run().await;
}


pub async fn run() {
    let config = get_config();
    let acceptor = TlsAcceptor::from(Arc::new(config.unwrap()));
    let database_url = get_db_url();


    let pool = get_pool(database_url).await.unwrap();
    let app = get_routes(Arc::new(pool));
    let addr = SocketAddr::from(([192, 168, 2, 23], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    serve(listener , app.into_make_service()).await.unwrap();

}

fn get_config() -> Result<ServerConfig, io::Error> {
    let cert = load_certs(Path::new("./certificates/cert.pem")).unwrap();
    let key = load_keys(Path::new("./certificates/key.pem")).unwrap();
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    if !path.exists() {
        println!("Could not find certificate file: {:?}", path);
    }
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

fn load_keys(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    if !path.exists() {
        println!("Could not find private key file: {:?}", path);
    }
    pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
        .next()
        .unwrap()
        .map(Into::into)
}
fn get_db_url() -> String{
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

async fn get_pool(url: String) -> Result<Pool<MySql>, Error>{
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
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(3))
        .connect(url)
        .await
}