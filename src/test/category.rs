use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::get_db_url;
use crate::schema::category::Categories;

#[cfg(test)]
async fn make_category()-> Categories {
    let category = sqlx::query_as("Select * from Categories ORDER BY Id DESC LIMIT 1")
        .fetch_optional(get_conn().await.as_mut())
        .await
        .unwrap();

    match category {
        Some(category) => category,
        None => {
            let _ = create_category().await;
            sqlx::query_as("Select * from Categories ORDER BY Id DESC LIMIT 1")
                .fetch_one(get_conn().await.as_mut())
                .await
                .unwrap()
        }
    }
}

#[cfg(test)]
pub(crate) async fn get_conn() -> PoolConnection<MySql>{
    let db = get_db_url();
    let pool = sqlx::mysql::MySqlPool::connect(&db).await.unwrap();
    pool.acquire().await.unwrap()
}
#[cfg(test)]
#[tokio::test]
async fn test_category(){
    create_category().await;
    update_category().await;
    select_category().await;
    delete_category().await;
}
#[cfg(test)]
pub(crate) async fn create_category(){
    let conn= get_conn().await;
    let category = Categories {
        Id: 0,
        Category: "A".to_string(),
        MonthlyFee: 50.0,
        WeeklyFee: 50.0,
        UsedBy: 0,
    };
    let res = category.create_category(conn).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn update_category(){
    let mut category = make_category().await;
    category.Category = "B".to_string();
    let conn = get_conn().await;
    let res = category.update_category(conn).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn select_category(){
    let conn = get_conn().await;
    let res = Categories::select_categories(conn).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn delete_category(){
    let category = make_category().await;
    let conn = get_conn().await;
    let res = category.delete_category(conn).await;
    assert!(res.is_ok());
}