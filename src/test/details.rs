use sqlx::MySql;
use sqlx::pool::PoolConnection;
use crate::get_db_url;
use crate::schema::beneficiary::{Beneficiary, BeneficiaryQueries};
use crate::schema::details::{BeneficiaryAllergy, BeneficiaryNotes, BeneficiaryPresence, Details, TokenAllergy, TokenNote, TokenPresence};
use crate::schema::user::UserRole;


#[cfg(test)]
pub(crate) async fn make_user_role() -> UserRole{
    let user: Option<UserRole> = sqlx::query_as("SELECT Username, Role FROM User")
        .fetch_optional(crate::test::beneficiary::get_conn().await.as_mut())
        .await
        .unwrap();

    user.unwrap_or_else(|| UserRole {
        Username: "soap".to_string(),
        Role: "Dev".to_string(),
    })
}
#[cfg(test)]
pub(crate) async fn make_beneficiary() -> Beneficiary{
    let beneficiary: Option<Beneficiary> = sqlx::query_as(&format!("{} ORDER BY Id DESC LIMIT 1",BeneficiaryQueries::SelectAdminDetails))
        .fetch_optional(crate::test::beneficiary::get_conn().await.as_mut())
        .await
        .unwrap();

    match beneficiary {
        Some(beneficiary) => beneficiary,
        None => {
            let _ = Beneficiary::create_beneficiary(crate::test::beneficiary::get_conn().await, crate::test::beneficiary::make_user_role().await).await;
            sqlx::query_as(&format!("{} ORDER BY Id DESC LIMIT 1",BeneficiaryQueries::SelectAdminDetails))
                .fetch_one(crate::test::beneficiary::get_conn().await.as_mut())
                .await
                .unwrap()
        }
    }
}
#[cfg(test)]
pub(crate) async fn insert_allergy(){
    let beneficiary = make_beneficiary().await;
    let token_allergy = TokenAllergy{
        Token: "test".to_string(),
        Allergy: BeneficiaryAllergy {
            BeneficiaryId: beneficiary.Id,
            Allergy: "Arachides".to_string()
        },
    };
    let conn = get_conn().await;
    let res = token_allergy.insert_allergy(conn).await;

    assert!(res.is_ok());
}


#[cfg(test)]
pub(crate) async fn insert_presence(){
    let beneficiary = make_beneficiary().await;
    let token_presence = TokenPresence{
        Token: "test".to_string(),
        Presence: BeneficiaryPresence { BeneficiaryId: beneficiary.Id, Date: "2023-02-10".to_string() },
    };
    let conn = get_conn().await;
    let res = token_presence.insert_presence(conn).await;

    assert!(res.is_ok());
}


#[cfg(test)]
pub(crate) async fn insert_note(){
    let beneficiary = make_beneficiary().await;
    let beneficiary_note = TokenNote {
        Token : "test".to_string(),
        Content: BeneficiaryNotes {
            BeneficiaryId: beneficiary.Id,
            Date: "2023-08-02".to_string(),
            Type: 0,
            Note: "This is for youuuuuuuuuu".to_string(),
        },
    };
    let conn = get_conn().await;
    let res = beneficiary_note.create_note(conn).await;

    assert!(res.is_ok());
}
#[cfg(test)]
pub(crate) async fn update_note(){
    let beneficiary = make_beneficiary().await;
    let beneficiary_note = TokenNote {
        Token : "test".to_string(),
        Content: BeneficiaryNotes {
            BeneficiaryId: beneficiary.Id,
            Date: "2023-08-02 00:00:00".to_string(),
            Type: 1,
            Note: "Not for you".to_string(),
        },
    };
    let conn = get_conn().await;
    let res = beneficiary_note.update_note(conn).await;

    assert!(res.is_ok());
}


#[cfg(test)]
pub(crate) async fn select_details(){
    let beneficiary = make_beneficiary().await;
    let conn = get_conn().await;
    let user = make_user_role().await;
    let res = Details::get_details(conn,user, beneficiary.Id).await;

    match res {
        Ok(val) => {
            println!("{:?}", val.Notes);
            assert!(!val.Notes.is_empty());
            assert!(!val.Allergies.is_empty());
            assert!(!val.Presences.is_empty());
        }
        Err(_) => assert!(false),
    }
}

#[cfg(test)]
pub(crate) async fn get_conn() -> PoolConnection<MySql>{
    let db = get_db_url();
    let pool = sqlx::mysql::MySqlPool::connect(&db).await.unwrap();
    pool.acquire().await.unwrap()
}
