use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
    use bincode::{Encode};
    use serde::{Deserialize, Serialize};
    use sqlx::{Decode, Error, MySql, MySqlConnection, Row};
    use sqlx::pool::PoolConnection;
    use crate::schema::{encode, encrypt};
use crate::schema::details::Details;
use crate::schema::user::UserRole;

pub(crate) trait BeneficiaryAction{
        async fn get_beneficiaries(conn: PoolConnection<MySql>, role: UserRole) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn search(conn: PoolConnection<MySql>, role: UserRole, search: &str) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn get_beneficiary(conn: PoolConnection<MySql>, role: UserRole, id: i32) -> Result<Vec<u8>, (StatusCode, String)>;
        async fn update_beneficiary(conn: PoolConnection<MySql>, role : UserRole, beneficiary: Beneficiary) -> Result<StatusCode, (StatusCode, String)>;
    }

pub(crate) enum BeneficiaryQueries{
    SelectUserBeneficiaries,
    SelectAdminBeneficiaries,
    SelectTsBeneficiaries,
    SelectUserDetails,
    SelectAdminDetails,
    SelectTsDetails,
    CreateBeneficiary,
    UpdateUserBeneficiary,
    UpdateAdminBeneficiary,
    UpdateTsBeneficiary,
}

impl Display for BeneficiaryQueries{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match self {
           BeneficiaryQueries::SelectUserBeneficiaries => {
               write!(f,
                      "SELECT \
                       Id, FirstName, LastName, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       HasAllergies, HasGeneralNote \
                       FROM Beneficiary"
               )
           }
           BeneficiaryQueries::SelectAdminBeneficiaries => {
                write!(f,
                       "SELECT \
                       Id, FirstName, LastName, Email, Phone, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       IsActive, HasAllergies, HasGeneralNote \
                       FROM Beneficiary"
                )
           }
           BeneficiaryQueries::SelectTsBeneficiaries => {
                write!(f,
                       "SELECT \
                       Id, FirstName, LastName, Email, Phone, Kid, Adult, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       IsActive, HasAllergies, HasGeneralNote  \
                       FROM Beneficiary"
                )
           }
           BeneficiaryQueries::SelectUserDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Kid, Adult, \
                       MonthlyAmount, WeeklyAmount, Category, MonthlyLimit, WeeklyLimit, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Language, IsActive, HasAllergies, HasGeneralNote \
                       FROM Beneficiary")
           }
           BeneficiaryQueries::SelectAdminDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Email, Phone, Address, PostalCode, Kid, Adult, \
                       MonthlyAmount, WeeklyAmount, Category, MonthlyLimit, WeeklyLimit, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, \
                       IsActive, HasAllergies, HasGeneralNote \
                       FROM Beneficiary")
           }
           BeneficiaryQueries::SelectTsDetails => {
               write!(f, "SELECT Id, FirstName, LastName, Email, Phone, Address, PostalCode, Kid, Adult, \
                       MonthlyAmount, WeeklyAmount, Category, MonthlyLimit, WeeklyLimit, \
                       DATE_FORMAT(Birth, '%Y-%m-%d') AS Birth, \
                       DATE_FORMAT(LastPresence, '%Y-%m-%d') AS LastPresence, \
                       Sexe, Language, Origin, City, Study, Income, FamilySituation, \
                       IsActive, IsSdf, IsEmployed, HasAllergies, HasGeneralNote \
                       FROM Beneficiary")
           }
           BeneficiaryQueries::CreateBeneficiary => {
                write!(f,
                       "INSERT INTO Beneficiary (Email, Phone, Address, PostalCode) VALUES (?, ?, ?, ?)"
                )
           }
           BeneficiaryQueries::UpdateUserBeneficiary => {
               write!(f,
                      "UPDATE `Beneficiary` \
                       SET `FirstName` = ?, `LastName` = ?, `MonthlyAmount` = ?, `WeeklyAmount` = ?, WHERE `Id` = ?"
               )
           }
           BeneficiaryQueries::UpdateAdminBeneficiary => {
                write!(f,
                       "UPDATE `Beneficiary` \
                       SET `FirstName` = ?, `LastName` = ?, `Email` = ?, `Phone` = ?, `Address` = ?, `PostalCode` = ?,  `Kid` = ?, `Adult` = ?, \
                       `MonthlyAmount` = ?, `WeeklyAmount` = ?, `Category` = ?, `MonthlyLimit` = ?, `WeeklyLimit` = ?, \
                       `Birth` = ?, `LastPresence` = ?, `Sexe` = ?, `Language` = ?, `Origin` = ?, \
                       `City` = ?, `IsActive` = ?, `HasAllergies` = ?, `HasGeneralNote` = ? \
                       WHERE `Id` = ?"
                )
           }
           BeneficiaryQueries::UpdateTsBeneficiary => {
                write!(f,
                       "UPDATE `Beneficiary` \
                       SET `FirstName` = ?, `LastName` = ?, `Email` = ?,`Phone` = ?, `Address` = ?, `PostalCode` = ?, \
                       `MonthlyAmount` = ?, `WeeklyAmount` = ?, `Category` = ?, `MonthlyLimit` = ?, `WeeklyLimit` = ?, \
                       `Kid` = ?, `Adult` = ?, `Birth` = ?, `LastPresence` = ?, `Sexe` = ?, `Language` = ?, \
                       `Origin` = ?, `City` = ?, `Study` = ?, `Income` = ?, `FamilySituation` = ?, `IsActive` = ?, \
                       `IsSdf` = ?, `IsEmployed` = ?, `HasAllergies` = ?, `HasGeneralNote` = ?,  \
                       WHERE `Id` = ?"
                )
           },
       }
    }
}
    #[derive(sqlx::FromRow, Encode,Decode, Serialize, Deserialize, Clone)]
    pub(crate) struct Beneficiary {
        pub(crate) Id: i32,
        pub(crate) FirstName: String,
        pub(crate) LastName: String,
        #[sqlx(default)]
        pub(crate) Email: String,
        #[sqlx(default)]
        pub(crate) Phone: String,
        #[sqlx(default)]
        pub(crate) Address: String,
        #[sqlx(default)]
        pub(crate) PostalCode: String,
        pub(crate) Kid: u8,
        pub(crate) Adult: u8,
        #[sqlx(default)]
        pub(crate) MonthlyAmount : f64,
        #[sqlx(default)]
        pub(crate) WeeklyAmount : f64,
        #[sqlx(default)]
        pub(crate) Category : i32,
        #[sqlx(default)]
        pub(crate) MonthlyLimit : f64,
        #[sqlx(default)]
        pub(crate) WeeklyLimit : f64,
        pub(crate) Birth: Option<String>,
        pub(crate) LastPresence: String,
        #[sqlx(default)]
        pub(crate) Sexe: String,
        #[sqlx(default)]
        pub(crate) Language: String,
        #[sqlx(default)]
        pub(crate) Origin: String,
        #[sqlx(default)]
        pub(crate) City: String,
        #[sqlx(default)]
        pub(crate) Study: String,
        #[sqlx(default)]
        pub(crate) Income: String,
        #[sqlx(default)]
        pub(crate) FamilySituation: String,
        pub(crate) IsActive: bool,
        #[sqlx(default)]
        pub(crate) IsSdf: bool,
        #[sqlx(default)]
        pub(crate) IsEmployed: bool,
        pub(crate) HasAllergies: bool,
        pub(crate) HasGeneralNote: bool,
    }


    impl Beneficiary{
        pub(crate) async fn create_beneficiary(mut conn : PoolConnection<MySql>, user_role: UserRole) -> Result<Vec<u8>, (StatusCode, String)> {
            println!("->> {:>12} - Create Beneficiary", "Handler");
            let is_created =
                    sqlx::query(&BeneficiaryQueries::CreateBeneficiary.to_string())
                        .bind(encrypt("".as_bytes()))
                        .bind(encrypt("".as_bytes()))
                        .bind(encrypt("".as_bytes()))
                        .bind(encrypt("".as_bytes()))
                        .execute(conn.as_mut())
                        .await;

            if is_created.is_err(){
                println!("->> {:>12} - Error: {:?}", "Handler", is_created);
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create beneficiary".to_string()))
            }

            if let Ok(id) = sqlx::query("SELECT LAST_INSERT_ID()")
                .fetch_one(conn.as_mut())
                .await
            {
                let id = id.get::<u32, usize>(0);

                let query = match user_role.Role.as_str() {
                    "User" =>{format!("{} WHERE Id = {id}", BeneficiaryQueries::SelectUserDetails)}
                    "Admin" | "Dev" =>{format!("{} WHERE Id = {id}", BeneficiaryQueries::SelectAdminDetails)}
                    "TS" =>{format!("{} WHERE Id = {id}", BeneficiaryQueries::SelectTsDetails)}
                    _ => {
                        println!("->> {:>12} - Error: Invalid role", "Handler");
                        return Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
                    }
                };

                let bene: Result<Beneficiary, Error> = sqlx::query_as(&query)
                    .fetch_one(conn.as_mut())
                    .await;


                match bene {
                    Ok(bene) => {
                        println!("->> {:>12} - Create Beneficiary - SUCCESS", "Handler");
                        encode(bene)
                    },
                    Err(e) => {
                        println!("->> {:>12} - Error: {:?}", "Handler", e);
                        Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get beneficiary".to_string()))
                    }
                }
            }else{
                println!("->> {:>12} - Error: Could not get last insert id", "Handler");
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get last insert id".to_string()))
            }
        }

        async fn find_beneficiaries(conn: &mut MySqlConnection, condition: String, user: UserRole) -> Result<Vec<Beneficiary>, Error>{
            match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&format!("{} {}",BeneficiaryQueries::SelectAdminDetails, condition))
                    .fetch_all(conn)
                    .await,
                "TS" => sqlx::query_as(&format!("{} {}", BeneficiaryQueries::SelectTsDetails, condition))
                    .fetch_all(conn)
                    .await,
                "User" => sqlx::query_as(&format!("{} {}", BeneficiaryQueries::SelectUserDetails, condition))
                    .fetch_all(conn)
                    .await,
                _ => {
                    println!("->> {:>12} - Get Beneficiary - FAILED : Invalid Role", "Handler");
                    Err(Error::RowNotFound)
                }
            }
        }

    }
    impl BeneficiaryAction for Beneficiary {
        async fn get_beneficiaries(mut conn: PoolConnection<MySql>, user: UserRole) -> Result<Vec<u8>, (StatusCode,String)>{
            println!("->> {:>12} - Get Beneficiaries - Role : {}", "Handler", user.Role);
            let bene: Result<Vec<Beneficiary>, Error> = match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::SelectAdminBeneficiaries))
                    .fetch_all(conn.as_mut())
                    .await,
                "TS" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::SelectTsBeneficiaries))
                    .fetch_all(conn.as_mut())
                    .await,
                "User" => sqlx::query_as(&format!("{} WHERE IsActive = 1" ,BeneficiaryQueries::SelectUserBeneficiaries))
                    .fetch_all(conn.as_mut())
                    .await,
                _ => {
                    println!("->> {:>12} - Error: Invalid role", "Handler");
                    return Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
                }
            };

            match bene {
                Ok(bene) => {
                    println!("->> {:>12} - Get Beneficiaries - SUCCESS", "Handler");
                    encode(bene)
                },
                Err(e) => {
                    println!("->> {:>12} - Error: {:?}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get beneficiaries".to_string()))
                }
            }
        }

        async fn search(mut conn: PoolConnection<MySql>, user: UserRole, search: &str) -> Result<Vec<u8>, (StatusCode, String)> {
            println!("->> {:>12} - Search Beneficiaries - Role : {}", "Handler", user.Role);
            let condition = format!("WHERE IsActive = 0 AND FirstName LIKE {search} OR LastName LIKE {search}");
            let bene = Self::find_beneficiaries(conn.as_mut(), condition, user)
                .await
                .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "Could not find any beneficiary".to_string()))?;
            println!("->> {:>12} - Search Beneficiaries - SUCCESS", "Handler");
            encode(bene)
        }

        async fn get_beneficiary(mut conn: PoolConnection<MySql>, user: UserRole, id: i32) -> Result<Vec<u8>, (StatusCode, String)> {
            println!("->> {:>12} - Get Beneficiary - Role : {}", "Handler", user.Role);
            let bene : Result<Beneficiary, Error> = match user.Role.as_str() {
                "Admin" | "Dev" => sqlx::query_as(&format!("{} WHERE Id = {id}",BeneficiaryQueries::SelectAdminDetails))
                    .fetch_one(conn.as_mut())
                    .await,
                "TS" => sqlx::query_as(&format!("{} WHERE Id = {id}", BeneficiaryQueries::SelectTsDetails))
                    .fetch_one(conn.as_mut())
                    .await,
                "User" => sqlx::query_as(&format!("{} WHERE Id = {id}", BeneficiaryQueries::SelectUserDetails))
                    .fetch_one(conn.as_mut())
                    .await,
                _ => {
                    println!("->> {:>12} - Get Beneficiary - FAILED : Invalid Role", "Handler");
                    return Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
                }
            };

            let details = Details::get_details(conn, user, id).await.map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "Could not get details".to_string()))?;

            if let Ok(bene) = bene {
                println!("->> {:>12} - Get Beneficiary - SUCCESS", "Handler");
                encode((bene, details))
            } else {
                println!("->> {:>12} - Get Beneficiary - FAILED : {}", "Handler", bene.err().unwrap());
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not get beneficiary".to_string()))
            }
        }

        async fn update_beneficiary(mut conn: PoolConnection<MySql>, user: UserRole, bene: Beneficiary) -> Result<StatusCode, (StatusCode, String)>{
            println!("->> {:>12} - Update Beneficiary - Role : {}", "Handler", user.Role);
            let result = match user.Role.as_str(){
                "User" => {
                    sqlx::query(&format!("{}", BeneficiaryQueries::UpdateUserBeneficiary))
                        .bind(bene.FirstName)
                        .bind(bene.LastName)
                        .bind(bene.MonthlyAmount)
                        .bind(bene.WeeklyAmount)
                        .bind(bene.Id)
                        .execute(conn.as_mut())
                        .await
                }
                "Admin" | "Dev" => {
                    sqlx::query(&format!("{}", BeneficiaryQueries::UpdateAdminBeneficiary))
                        .bind(bene.FirstName)
                        .bind(bene.LastName)
                        .bind(bene.Email)
                        .bind(bene.Phone)
                        .bind(bene.Address)
                        .bind(bene.PostalCode)
                        .bind(bene.Kid)
                        .bind(bene.Adult)
                        .bind(bene.MonthlyAmount)
                        .bind(bene.WeeklyAmount)
                        .bind(bene.Category)
                        .bind(bene.MonthlyLimit)
                        .bind(bene.WeeklyLimit)
                        .bind(bene.Birth)
                        .bind(bene.LastPresence)
                        .bind(bene.Sexe)
                        .bind(bene.Language)
                        .bind(bene.Origin)
                        .bind(bene.City)
                        .bind(bene.IsActive)
                        .bind(bene.HasAllergies)
                        .bind(bene.HasGeneralNote)
                        .bind(bene.Id)
                        .execute(conn.as_mut())
                        .await
                }
                "TS" => {
                    sqlx::query(&format!("{}", BeneficiaryQueries::UpdateTsBeneficiary))
                        .bind(bene.FirstName)
                        .bind(bene.LastName)
                        .bind(bene.Email)
                        .bind(bene.Phone)
                        .bind(bene.Address)
                        .bind(bene.PostalCode)
                        .bind(bene.MonthlyAmount)
                        .bind(bene.WeeklyAmount)
                        .bind(bene.Category)
                        .bind(bene.MonthlyLimit)
                        .bind(bene.WeeklyLimit)
                        .bind(bene.Kid)
                        .bind(bene.Adult)
                        .bind(bene.Birth)
                        .bind(bene.LastPresence)
                        .bind(bene.Sexe)
                        .bind(bene.Language)
                        .bind(bene.Origin)
                        .bind(bene.City)
                        .bind(bene.Study)
                        .bind(bene.Income)
                        .bind(bene.FamilySituation)
                        .bind(bene.IsActive)
                        .bind(bene.IsSdf)
                        .bind(bene.IsEmployed)
                        .bind(bene.HasAllergies)
                        .bind(bene.HasGeneralNote)
                        .bind(bene.Id)
                        .execute(conn.as_mut())
                        .await
                }
                _ => {
                    println!("->> {:>12} - Error: Invalid role", "Handler");
                    return Err((StatusCode::FORBIDDEN, "Invalid role".to_string()))
                }
            };

            match result {
               Ok(_) => {
                   println!("->> {:>12} - Update Beneficiary - SUCCESS", "Handler");
                   Ok(StatusCode::OK)
               }
                Err(e) => {
                    println!("->> {:>12} - Update Beneficiary - FAILED : {}", "Handler", e);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not update beneficiary".to_string()))
                }
            }

        }
    }