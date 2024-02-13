
    use std::sync::Arc;
    use anyhow::Context;
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use sqlx::{Decode, Error, MySql, MySqlPool};
    use sqlx::pool::PoolConnection;
    use bcrypt::{DEFAULT_COST, verify};
    use bincode::{Encode};
    use uuid::{Uuid};
    use crate::route::acquire_connection;
    use crate::schema::beneficiary::{Beneficiary};
    use crate::schema::encode;

    #[derive(sqlx::FromRow,Encode,Decode, Serialize, Deserialize)]
    pub(crate) struct User{
        pub(crate) Id: i32,
        pub(crate) Username: String,
        pub(crate) Password: String,
        pub(crate) Role: String,
    }

    impl User{
        pub(crate) async fn validate_password(&self, other: &str) -> bool {
            verify(other, &self.Password).unwrap_or(false) || other == self.Password
        }

        pub(crate) async fn get_users(mut conn: PoolConnection<MySql>, username: String) -> Result<Vec<u8>, (StatusCode, String)>{
            let users: Result<Vec<User>, Error> = sqlx::query_as("SELECT Id, Username, '' as Password, Role FROM User WHERE Role NOT LIKE 'Dev' AND Username != 'admin' AND Username != ?")
                .bind(username)
                .fetch_all(conn.as_mut())
                .await;
            match users {
                Ok(users) => encode(users),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }

    }

    #[derive(sqlx::FromRow,Encode, Decode, Serialize, Deserialize)]
    pub(crate) struct UserRole{
        pub(crate) Username: String,
        pub(crate) Role: String,
    }

    #[derive(sqlx::FromRow,Encode, Decode, Serialize, Deserialize)]
    pub(crate) struct UserToken{
        pub(crate) Token: String,
        pub(crate) User: User
    }

    impl UserToken{
        pub(crate) async fn create_user(&self, mut conn: PoolConnection<MySql>) -> Result<Vec<u8>, anyhow::Error>{
            let result = sqlx::query("INSERT INTO User (Username, Password, Role) VALUES (?, ?, ?)")
                .bind(&self.User.Username)
                .bind(bcrypt::hash(&self.User.Password, DEFAULT_COST).unwrap())
                .bind(&self.User.Role)
                .execute(conn.as_mut())
                .await
                .context("Failed to insert user");

            if result.is_err() {
                return Err(anyhow::Error::msg("Failed to insert user"));
            }

            let user = sqlx::query_as::<_, User>("SELECT * FROM User WHERE Username = ?")
                .bind(&self.User.Username)
                .fetch_one(conn.as_mut())
                .await
                .context("Failed to get user")?;


            let encoded = encode(user);

            match encoded {
                Ok(val) => Ok(val),
                Err(_) => {
                    Err(anyhow::Error::msg("Failed to encode user"))
                },
            }

        }

        pub(crate) async fn update_user(&self, mut conn: PoolConnection<MySql>) -> Result<(), anyhow::Error>{
            if !self.User.Password.is_empty() {
                let _ = sqlx::query("UPDATE User Set Username = ?, Password = ?, Role = ? WHERE Id = ?")
                    .bind(&self.User.Username)
                    .bind(bcrypt::hash(&self.User.Password, DEFAULT_COST).unwrap())
                    .bind(&self.User.Role)
                    .bind(self.User.Id)
                    .execute(conn.as_mut())
                    .await
                    .context("Failed to update user")?;
            } else {
                let _ = sqlx::query("UPDATE User Set Username = ?, Role = ? WHERE Id = ?")
                    .bind(&self.User.Username)
                    .bind(&self.User.Role)
                    .bind(self.User.Id)
                    .execute(conn.as_mut())
                    .await
                    .context("Failed to update user")?;
            }

            Ok(())
        }

        pub(crate) async fn delete_user(&self, mut conn: PoolConnection<MySql>) -> Result<(), anyhow::Error>{
            let _ = sqlx::query("DELETE FROM UserSession WHERE UserId = ?")
                .bind(self.User.Id)
                .execute(conn.as_mut())
                .await
                .context("Failed to delete user session")?;

            let _ = sqlx::query("DELETE FROM User WHERE Id = ?")
                .bind(self.User.Id)
                .execute(conn.as_mut())
                .await
                .context("Failed to delete user")?;

            Ok(())
        }
    }



    #[derive( sqlx::FromRow, Serialize, Deserialize)]
    pub(crate) struct Token {
        pub(crate) Token: String,
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub(crate) struct TokenBeneId{
        pub(crate) Token: String,
        pub(crate) Id: i32,
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub(crate) struct TokenBene{
        pub(crate) Token: String,
        pub(crate) Beneficiary: Beneficiary,
    }

    #[derive(sqlx::FromRow, Serialize, Deserialize)]
    pub(crate) struct TokenSearch{
        pub(crate) Token: String,
        pub(crate) Search: String,
    }
    #[derive(sqlx::FromRow,Serialize, Deserialize)]
    pub(crate) struct UserLogin{
        pub(crate) Username: String,
        pub(crate) Password: String,
    }
    impl UserLogin{
        pub(crate) async fn get_user(&self, mut conn : PoolConnection<MySql>) -> Result<User, (StatusCode, String)>{
            let user = sqlx::query_as::<_, User>("SELECT * FROM User WHERE Username = ?")
                .bind(&self.Username)
                .fetch_one(conn.as_mut())
                .await;

            match user {
                Ok(user) => Ok(user),
                Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
            }
        }
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize)]
    pub(crate) struct Connection{
        pub(crate) Token: String,
        pub(crate) Role: String,
    }

    impl Connection{
        pub(crate) async fn get_or_create_connection(pool: Arc<MySqlPool>, user: User) -> Result<Vec<u8>, (StatusCode, String)>{
            println!("->> {:>12} - Login - User : {}", "Handler", user.Username);

            let mut connection = Connection{
                Token: format!("{}-{}", user.Username, Uuid::new_v4()),
                Role: user.Role.to_string(),
            };
            let conn = acquire_connection(pool.clone()).await?;
            let session = connection.get_token(conn, user.Id).await;

            if session.is_none() {
                let conn = acquire_connection(pool.clone()).await?;
                connection.create_session(conn, user.Id, connection.Token.clone()).await.map_err(|e| {
                    println!("->> {:>12} - Login - Error : {}", "Handler", e.1);
                    e
                })?;
            }
            println!("->> {:>12} - Login - Token : {}", "Handler", connection.Token);
            encode(connection)
        }

        async fn create_session(&mut self, mut conn : PoolConnection<MySql>, id: i32, session: String) -> Result<(), (StatusCode, String)> {
            println!("->> {:>12} - Create Session - User : {}", "Handler", id);
            let res = sqlx::query("INSERT INTO UserSession (UserId, Token, ConnectionDate, Expires) VALUES (?, ?, NOW(), NOW() + INTERVAL 1 DAY)")
                .bind(id)
                .bind(session)
                .execute(conn.as_mut())
                .await;

            match res {
                Ok(_) => {
                    println!("->> {:>12} - Create Session - Success", "Handler");
                    Ok(())
                },
                Err(_) => {
                    println!("->> {:>12} - Create Session - Failed", "Handler");
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session".to_string()))
                },
            }
        }
        async fn get_token(&mut self, mut conn: PoolConnection<MySql>, id: i32) -> Option<Token>{
            println!("->> {:>12} - Get Token - User : {}", "Handler", id);
            let session: Result<Option<Token>, Error>= sqlx::query_as("SELECT Token FROM UserSession WHERE UserId = ? AND Expires > NOW()")
                .bind(id)
                .fetch_optional(conn.as_mut())
                .await;

            match session {
                Ok(Some(session)) => {
                    println!("->> {:>12} - Get Token - Success", "Handler");
                    self.Token = session.Token.clone();
                    Some(session)
                },
                Ok(None) => {
                    println!("->> {:>12} - Get Token - NO TOKEN", "Handler");
                    None
                },
                Err(e) => {
                    println!("->> {:>12} - Get Token - FAILED : {:?}", "Handler",e);
                    None
                },
            }
        }
    }
