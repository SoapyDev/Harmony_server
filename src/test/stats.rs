use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::get_db_url;
use crate::schema::stats::Stats;

#[cfg(test)]
#[tokio::test]
async fn select_stats(){
    let conn = get_conn().await;
    let stats = Stats::get_stats(conn).await;

    match stats {
        Ok(stat) => {
            println!("{:?}", stat)
        }
        Err(_) => {
            assert!(false, "Failed to get stats")
        }
    }
}
#[cfg(test)]
pub(crate) async fn get_conn() -> PoolConnection<MySql>{
    let db = get_db_url();
    let pool = sqlx::mysql::MySqlPool::connect(&db).await.unwrap();
    pool.acquire().await.unwrap()
}
