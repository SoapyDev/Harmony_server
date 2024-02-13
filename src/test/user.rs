use sqlx::{MySql, Row};
use sqlx::pool::PoolConnection;
use crate::get_db_url;
use crate::schema::user::{User, UserToken};


#[cfg(test)]
pub(crate) async fn make_user_token() -> UserToken {
 let last_id = sqlx::query("Select Id from User Where Username = 'soap'")
        .fetch_optional(get_conn().await.as_mut())
        .await
        .unwrap();

    let id = match last_id {
        Some(id) => id.get(0),
        None => 1,
    };


    UserToken {
        Token: "test".to_string(),
        User: User {
            Id: id,
            Username: "soap".to_string(),
            Password: "abea3571".to_string(),
            Role: "Dev".to_string(),
        }
    }
}
#[cfg(test)]
pub(crate) async fn create_user(){
    let user = make_user_token().await;
    let conn = get_conn().await;
    let res = user.create_user(conn).await;
    assert!(res.is_ok());
}

#[cfg(test)]
pub(crate) async fn update_user(){
    let user = make_user_token().await;
    let conn = get_conn().await;
    let res = user.update_user(conn).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn delete_user(){
    let user = make_user_token().await;
    let conn = get_conn().await;
    let res = user.delete_user(conn).await;
    assert!(res.is_ok());
}

#[cfg(test)]
pub(crate) async fn get_conn() -> PoolConnection<MySql>{
    let db = get_db_url();
    let pool = sqlx::mysql::MySqlPool::connect(&db).await.unwrap();
    pool.acquire().await.unwrap()
}
