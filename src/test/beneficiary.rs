use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::get_db_url;
use crate::schema::beneficiary::{Beneficiary, BeneficiaryAction, BeneficiaryQueries};
use crate::schema::user::{UserRole};

#[cfg(test)]
pub(crate) async fn make_user_role() -> UserRole{
    let user: Option<UserRole> = sqlx::query_as("SELECT Username, Role FROM User")
        .fetch_optional(get_conn().await.as_mut())
        .await
        .unwrap();

    user.unwrap_or_else(|| UserRole {
        Username: "soap".to_string(),
        Role: "Dev".to_string(),
    })
}

#[cfg(test)]
pub(crate) async fn get_conn() -> PoolConnection<MySql>{
    let db = get_db_url();
    let pool = sqlx::mysql::MySqlPool::connect(&db).await.unwrap();
    pool.acquire().await.unwrap()
}
#[cfg(test)]
pub(crate) async fn make_beneficiary() -> Beneficiary{
    let beneficiary: Option<Beneficiary> = sqlx::query_as(&format!("{} ORDER BY Id DESC LIMIT 1",BeneficiaryQueries::SelectAdminDetails))
        .fetch_optional(get_conn().await.as_mut())
        .await
        .unwrap();

    match beneficiary {
        Some(beneficiary) => beneficiary,
        None => {
            let _ = Beneficiary::create_beneficiary(get_conn().await, make_user_role().await).await;
            sqlx::query_as(&format!("{} ORDER BY Id DESC LIMIT 1",BeneficiaryQueries::SelectAdminDetails))
                .fetch_one(get_conn().await.as_mut())
                .await
                .unwrap()
        }
    }
}

#[cfg(test)]
pub(crate) async fn create_beneficiary(){
    let user = make_user_role().await;
    let conn= get_conn().await;
    let res = Beneficiary::create_beneficiary(conn, user).await;
    assert!(res.is_ok());
}

#[cfg(test)]
pub(crate) async fn update_beneficiary(){
    let user = make_user_role().await;
    let mut beneficiary = make_beneficiary().await;
    beneficiary.FirstName = "Test".to_string();
    beneficiary.LastName = "Test".to_string();
    let conn = get_conn().await;
    let res = Beneficiary::update_beneficiary(conn, user, beneficiary).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn select_beneficiary(){
    let user = make_user_role().await;
    let beneficiary = make_beneficiary().await;
    let conn = get_conn().await;
    let res = Beneficiary::get_beneficiary(conn, user, beneficiary.Id).await;
    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn select_beneficiaries(){
    let user = make_user_role().await;
    let conn = get_conn().await;
    let res = Beneficiary::get_beneficiaries(conn, user).await;
    assert!(res.is_ok());
}