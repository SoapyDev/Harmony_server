
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

        pub(crate) async fn get_users(mut conn: PoolConnection<MySql>) -> Result<Vec<u8>, (StatusCode, String)>{
            let users: Result<Vec<User>, Error> = sqlx::query_as("SELECT Id, Username, '' as Password, Role FROM User")
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
        pub(crate) async fn create_user(&self, mut conn: PoolConnection<MySql>) -> Result<(), anyhow::Error>{
            sqlx::query("INSERT INTO User (Username, Password, Role) VALUES (?, ?, ?)")
                .bind(&self.User.Username)
                .bind(bcrypt::hash(&self.User.Password, DEFAULT_COST).unwrap())
                .bind(&self.User.Role)
                .execute(conn.as_mut())
                .await
                .context("Failed to insert user")?;
            Ok(())
        }

        pub(crate) async fn update_user(&self, mut conn: PoolConnection<MySql>) -> Result<(), anyhow::Error>{
            let _ = sqlx::query("UPDATE User Set Username = ?, Password = ?, Role = ? WHERE Id = ?")
                .bind(&self.User.Username)
                .bind(bcrypt::hash(&self.User.Password, DEFAULT_COST).unwrap())
                .bind(&self.User.Role)
                .bind(self.User.Id)
                .execute(conn.as_mut())
                .await
                .context("Failed to update user")?;

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
        pub(crate) Session: String,
        pub(crate) Role: String,
    }

    impl Connection{
        pub(crate) async fn get_or_create_connection(pool: Arc<MySqlPool>, user: User) -> Result<Vec<u8>, (StatusCode, String)>{
            let mut connection = Connection{
                Session: Uuid::new_v4().to_string(),
                Role: user.Role.to_string(),
            };
            let conn = acquire_connection(pool.clone()).await?;
            let session = connection.get_session(conn, user.Id).await;

            if session.is_none() {
                let conn = acquire_connection(pool.clone()).await?;
                connection.create_session(conn, user.Id, connection.Session.clone()).await?;
            }
            encode(connection)
        }

        async fn create_session(&mut self, mut conn : PoolConnection<MySql>, id: i32, session: String) -> Result<(), (StatusCode, String)> {
            let res = sqlx::query("INSERT INTO UserSession (UserId, Session, ConnectionDate, Expires) VALUES (?, ?, NOW(), (NOW() + INTERVAL 1 DAY))")
                .bind(id)
                .bind(session)
                .execute(conn.as_mut())
                .await;

            match res {
                Ok(_) => Ok(()),
                Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session".to_string())),
            }
        }
        async fn get_session(&mut self, mut conn: PoolConnection<MySql>, id: i32) -> Option<Token>{
            let session: Result<Token, anyhow::Error> = sqlx::query_as("SELECT Session FROM UserSession WHERE UserId = ? AND Expires > NOW()")
                .bind(id)
                .fetch_one(conn.as_mut())
                .await
                .context("Failed to get session");

            match session {
                Ok(session) => {
                    self.Session = session.Token.clone();
                    Some(session)
                },
                Err(_) => None,
            }
        }
    }
