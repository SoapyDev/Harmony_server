use std::fmt::{Display, Formatter};
use anyhow::Context;
use axum::http::StatusCode;
    use bincode::{Encode};
    use serde::{Deserialize, Serialize};
    use sqlx::{Decode, Error, MySql, Row};
    use sqlx::pool::PoolConnection;
    use crate::schema::{ encode};
use crate::schema::user::UserRole;

pub(crate) trait BeneficiaryAction{
        async fn get_beneficiaries(conn: PoolConnection<MySql>, role: UserRole) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn search(conn: PoolConnection<MySql>, role: UserRole, search: &str) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn get_beneficiary(conn: PoolConnection<MySql>, role: UserRole, id: i32) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn update_beneficiary(conn: PoolConnection<MySql>, role : UserRole, beneficiary: &Beneficiary) -> Result<StatusCode, Error>;
    }

enum BeneficiaryQueries{
    GetAdmin,
    GetTs,
    GetUser,
    GetTsDetails,
    GetAdminDetails,
    GetUserDetails,
    Create,
    UpdateAdmin,
    UpdateTs,
}

impl Display for BeneficiaryQueries{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self {
           BeneficiaryQueries::GetAdmin => {
                write!(f,
                       "SELECT \
                       Id, FirstName, LastName, Email, Phone, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, \
                       IsActive, HasAllergies, HasGeneralNote \
                       FROM Beneficiary"
                )
           }
           BeneficiaryQueries::GetTs => {
                write!(f,
                       "SELECT \
                       Id, FirstName, LastName, Email, Phone, Address, PostalCode, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, Study, Income, FamilySituation, \
                       IsActive, IsSdf, IsEmployed, HasAllergies, HasGeneralNote  \
                       FROM Beneficiary"
                )
           }
           BeneficiaryQueries::GetUser => {
                write!(f,
                       "SELECT \
                       Id, FirstName, LastName, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Language, IsActive, HasAllergies, HasGeneralNote \
                       FROM Beneficiary"
                )
           }
           BeneficiaryQueries::GetUserDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Language, IsActive, HasAllergies, HasGeneralNote,\
                       GeneralNote FROM Beneficiary WHERE Id = ?")
           }
           BeneficiaryQueries::GetTsDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Email, Phone, Address, PostalCode, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, Study, Income, FamilySituation, \
                       IsActive, IsSdf, IsEmployed, HasAllergies, HasGeneralNote, \
                       GeneralNote, TSNote, Situation \
                       FROM Beneficiary WHERE Id = ?")
           }
           BeneficiaryQueries::GetAdminDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Email, Phone, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, \
                       IsActive, HasAllergies, HasGeneralNote, \
                       GeneralNote, AdminNote FROM Beneficiary WHERE Id = ?")
           }
           BeneficiaryQueries::Create => {
                write!(f,
                       "INSERT INTO Beneficiary \
                       (FirstName, LastName, Email, Phone, Address, PostalCode, Kid, Adult,  \
                       IsActive, IsSdf, IsEmployed, HasAllergies, HasGeneralNote, HasAdminNote, \
                       HasTsNote, GeneralNote, AdminNote, PrivateNote, Situation) \
                       VALUES \
                       ('', '', '', '', '', '', 0, 1, 1, 0, 0, 0, 0, 0, 0, '', '', '', '')"
                )
           }
           BeneficiaryQueries::UpdateAdmin => {
                write!(f,
                       "UPDATE `Beneficiary` \
                       SET `FirstName` = ?, `LastName` = ?, `Email` = ?, `Phone` = ?, `Kid` = ?, `Adult` = ?, \
                       `Birth` = ?, `LastPresence` = ?, `Sexe` = ?, `Language` = ?, `Origin` = ?, \
                       `City` = ?, `IsActive` = ?, `HasAllergies` = ?, `HasGeneralNote` = ?, \
                       `GeneralNote` = ?, `AdminNote` = ? \
                       WHERE `Id` = ?"
                )
           }
           BeneficiaryQueries::UpdateTs => {
                write!(f,
                       "UPDATE `Beneficiary` \
                       SET `FirstName` = ?, `LastName` = ?, `Email` = ?,`Phone` = ?, `Address` = ?, `PostalCode` = ?, \
                       `Kid` = ?, `Adult` = ?, `Birth` = ?, `LastPresence` = ?, `Sexe` = ?, `Language` = ?, \
                       `Origin` = ?, `City` = ?, `Study` = ?, `Income` = ?, `FamilySituation` = ?, `IsActive` = ?, \
                       `IsSdf` = ?, `IsEmployed` = ?, `HasAllergies` = ?, `HasGeneralNote` = ?,  \
                       `GeneralNote` = ?, `TsNote` = ?, `Situation` = ? \
                       WHERE `Id` = ?"
                )
           }
       }
    }
}

    #[derive(sqlx::FromRow, Encode,Decode, Serialize, Deserialize)]
    pub(crate) struct Beneficiary {
        pub Id: i32,
        pub FirstName: String,
        pub LastName: String,
        #[sqlx(default)]
        pub Email: String,
        #[sqlx(default)]
        pub Phone: String,
        #[sqlx(default)]
        pub Address: String,
        #[sqlx(default)]
        pub PostalCode: String,
        pub Kid: u8,
        pub Adult: u8,
        pub Birth: Option<String>,
        pub LastPresence: String,
        #[sqlx(default)]
        pub Sexe: String,
        pub Language: String,
        #[sqlx(default)]
        pub Origin: String,
        #[sqlx(default)]
        pub City: String,
        #[sqlx(default)]
        pub Study: String,
        #[sqlx(default)]
        pub Income: String,
        #[sqlx(default)]
        pub FamilySituation: String,
        pub IsActive: bool,
        #[sqlx(default)]
        pub IsSdf: bool,
        #[sqlx(default)]
        pub IsEmployed: bool,
        pub HasAllergies: bool,
        pub HasGeneralNote: bool,
        #[sqlx(default)]
        pub GeneralNote: String,
        #[sqlx(default)]
        pub AdminNote: String,
        #[sqlx(default)]
        pub TsNote: String,
        #[sqlx(default)]
        pub Situation: String,
    }


    impl Beneficiary{
        pub(crate) async fn create_beneficiary(mut conn : PoolConnection<MySql>, user: UserRole) -> Result<Vec<u8>, (StatusCode, String)> {
            let _ = match user.Role.as_str() {
                "TS" =>
                    sqlx::query(&BeneficiaryQueries::Create.to_string())
                    .execute(conn.as_mut())
                    .await,
                _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string()))
            };


            if let Ok(id) = sqlx::query("SELECT LAST_INSERT_ID()")
                .fetch_one(conn.as_mut())
                .await{
                let id = id.get::<u32, usize>(0);
                let bene: Result<Beneficiary, Error> = sqlx::query_as(&format!("{}", BeneficiaryQueries::GetTsDetails))
                    .bind(id)
                    .fetch_one(conn.as_mut())
                    .await;

                match bene {
                    Ok(bene) => encode(bene),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
                }
            }else{
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get last insert id".to_string()))
            }
        }
    }
    impl BeneficiaryAction for Beneficiary {
        async fn get_beneficiaries(mut conn: PoolConnection<MySql>, user: UserRole) -> Result<Vec<u8>, (StatusCode,String)>{
            let bene: Result<Vec<Beneficiary>, Error> = match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::GetAdmin))
                    .fetch_all(conn.as_mut())
                    .await,
                "TS" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::GetTs))
                    .fetch_all(conn.as_mut())
                    .await,
                "User" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::GetUser))
                    .fetch_all(conn.as_mut())
                    .await,
                _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string()))
            };

            match bene {
                Ok(bene) => encode(bene),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }

        async fn search(mut conn: PoolConnection<MySql>, user: UserRole, search: &str) -> Result<Vec<u8>, (StatusCode, String)> {
            let condition = "WHERE IsActive = 0 AND FirstName LIKE ? OR LastName LIKE ?";
            let bene: Result<Vec<Beneficiary>, Error> = match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&format!("{} {}", BeneficiaryQueries::GetAdmin, condition))
                    .bind(format!("%{}%", search))
                    .bind(format!("%{}%", search))
                    .fetch_all(conn.as_mut())
                    .await,
                "TS" => sqlx::query_as(&format!("{} {}", BeneficiaryQueries::GetTs, condition))
                    .bind(format!("%{}%", search))
                    .bind(format!("%{}%", search))
                    .fetch_all(conn.as_mut())
                    .await,
                "User" => sqlx::query_as(&format!("{} {} ", BeneficiaryQueries::GetUser, condition))
                    .bind(format!("%{}%", search))
                    .bind(format!("%{}%", search))
                    .fetch_all(conn.as_mut())
                    .await,
                _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string()))
            };

            match bene {
                Ok(bene) => encode(bene),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }

        async fn get_beneficiary(mut conn: PoolConnection<MySql>, user: UserRole, id: i32) -> Result<Vec<u8>, (StatusCode, String)> {
            let bene : Result<Beneficiary, Error> = match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&BeneficiaryQueries::GetAdminDetails.to_string())
                    .bind(id)
                    .fetch_one(conn.as_mut())
                    .await,
                "TS" => sqlx::query_as(&BeneficiaryQueries::GetTsDetails.to_string())
                    .bind(id)
                    .fetch_one(conn.as_mut())
                    .await,
                "User" => sqlx::query_as(&BeneficiaryQueries::GetUserDetails.to_string())
                    .bind(id)
                    .fetch_one(conn.as_mut())
                    .await,
                _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid role".to_string()))
            };

            let details = Details::get_details(conn, id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Ok(bene) = bene {
                encode((bene, details))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get beneficiary".to_string()))
            }

        }

        async fn update_beneficiary(mut conn: PoolConnection<MySql>, user: UserRole, bene: &Beneficiary) -> Result<StatusCode, Error>{
            match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query(&BeneficiaryQueries::UpdateAdmin.to_string())
                    .bind(bene.FirstName.clone())
                    .bind(bene.LastName.clone())
                    .bind(bene.Email.clone())
                    .bind(bene.Phone.clone())
                    .bind(bene.Kid)
                    .bind(bene.Adult)
                    .bind(bene.Birth.clone())
                    .bind(bene.LastPresence.clone())
                    .bind(bene.Sexe.clone())
                    .bind(bene.Language.clone())
                    .bind(bene.Origin.clone())
                    .bind(bene.City.clone())
                    .bind(bene.IsActive)
                    .bind(bene.HasAllergies)
                    .bind(bene.HasGeneralNote)
                    .bind(bene.GeneralNote.clone())
                    .bind(bene.AdminNote.clone())
                    .bind(bene.Id)
                    .execute(conn.as_mut())
                    .await?,
                "TS" => sqlx::query(&BeneficiaryQueries::UpdateTs.to_string())
                    .bind(bene.FirstName.clone())
                    .bind(bene.LastName.clone())
                    .bind(bene.Email.clone())
                    .bind(bene.Phone.clone())
                    .bind(bene.Address.clone())
                    .bind(bene.PostalCode.clone())
                    .bind(bene.Kid)
                    .bind(bene.Adult)
                    .bind(bene.Birth.clone())
                    .bind(bene.LastPresence.clone())
                    .bind(bene.Sexe.clone())
                    .bind(bene.Language.clone())
                    .bind(bene.Origin.clone())
                    .bind(bene.City.clone())
                    .bind(bene.Study.clone())
                    .bind(bene.Income.clone())
                    .bind(bene.FamilySituation.clone())
                    .bind(bene.IsActive)
                    .bind(bene.IsSdf)
                    .bind(bene.IsEmployed)
                    .bind(bene.HasAllergies)
                    .bind(bene.HasGeneralNote)
                    .bind(bene.GeneralNote.clone())
                    .bind(bene.TsNote.clone())
                    .bind(bene.Situation.clone())
                    .bind(bene.Id)
                    .execute(conn.as_mut())
                    .await?,
                _ => return Err(Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Invalid role")))
            };

            Ok(StatusCode::OK)
        }
    }

    #[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
    pub(crate) struct BeneficiaryAllergy{
        pub(crate) BeneficiaryId: i32,
        pub(crate) Allergy: String,
    }

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct TokenAllergy{
    pub(crate) Token: String,
    pub(crate) Allergy: BeneficiaryAllergy,
}
    impl TokenAllergy{
        pub(crate) async fn insert_allergy(&self,mut conn: PoolConnection<MySql>) -> Result<(), Error>{
            let _ = sqlx::query("INSERT INTO Harmony.BeneficiaryAllergies (BeneficiaryId, Allergy) VALUES (?, ?)")
                .bind(self.Allergy.BeneficiaryId)
                .bind(self.Allergy.Allergy.clone())
                .execute(conn.as_mut())
                .await?;
            Ok(())
        }

        pub(crate) async fn delete_allergy(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
            let _ = sqlx::query("DELETE FROM Harmony.BeneficiaryAllergies WHERE BeneficiaryId = ? AND Allergy LIKE ? ESCAPE '#'")
                .bind(self.Allergy.BeneficiaryId)
                .bind(self.Allergy.Allergy.clone())
                .execute(conn.as_mut())
                .await?;
            Ok(())
        }
    }

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct BeneficiaryPresence{
    pub(crate) BeneficiaryId: i32,
    pub(crate) Date: String,
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct TokenPresence{
    pub(crate) Token: String,
    pub(crate) Presence: BeneficiaryPresence,
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct BeneficiaryNotes{
    #[sqlx(default)]
    pub(crate) GeneralNote: String,
    #[sqlx(default)]
    pub(crate) AdminNote: String,
    #[sqlx(default)]
    pub(crate) PrivateNote: String,
    #[sqlx(default)]
    pub(crate) Situation: String,

}
impl TokenPresence{

    pub(crate) async fn insert_presence(&self,mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        let _ = sqlx::query("INSERT INTO BeneficiaryPresences (BeneficiaryId, PresenceDate) VALUES (?, ?)")
            .bind(self.Presence.BeneficiaryId)
            .bind(self.Presence.Date.clone())
            .execute(conn.as_mut())
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_presence(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        let _ = sqlx::query("DELETE FROM BeneficiaryPresences WHERE BeneficiaryId = ? AND PresenceDate = ?")
            .bind(self.Presence.BeneficiaryId)
            .bind(self.Presence.Date.clone())
            .execute(conn.as_mut())
            .await?;
        Ok(())
    }
}

    #[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
    pub(crate) struct Details{
        pub(crate) Id: i32,
        pub(crate) Presences: Vec<BeneficiaryPresence>,
        pub(crate) Allergies: Vec<BeneficiaryAllergy>,
    }

    impl Details {
        pub(crate) async fn get_details(mut conn: PoolConnection<MySql>,  id: i32) -> Result<Details, anyhow::Error>{


            let presences = sqlx::query_as("SELECT BeneficiaryId, DATE_FORMAT(PresenceDate, '%Y-%m-%d') AS Date FROM BeneficiaryPresences WHERE BeneficiaryId = ? Order By Date ASC")
            .bind(id)
                .fetch_all(conn.as_mut())
                .await
                .context("Could not get presence list")?;

            let allergies =  sqlx::query_as("SELECT * FROM BeneficiaryAllergies WHERE BeneficiaryId = ?")
            .bind(id)
                .fetch_all(conn.as_mut())
                .await
                .context("Could not get allergy list")?;

            Ok(
                Self{
                    Id: id,
                    Presences: presences,
                    Allergies: allergies,
                }
            )
        }
    }