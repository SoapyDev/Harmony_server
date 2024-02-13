use std::fmt::{Display, Formatter};
use anyhow::Context;
use bincode::Encode;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Error, MySql, MySqlConnection};
use sqlx::pool::PoolConnection;
use crate::schema::user::UserRole;

pub(crate) enum DetailsQueries{
    SelectAllergies,
    SelectPresences,
    SelectUserNotes,
    SelectAdminNotes,
    SelectTsNotes,
    InsertAllergy,
    DeleteAllergy,
    InsertPresence,
    DeletePresence,
    CreateNote,
    UpdateNote,
    DeleteNote,
}

impl Display for DetailsQueries{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DetailsQueries::SelectAllergies => {
                write!(f,"SELECT * FROM BeneficiaryAllergies WHERE BeneficiaryId = ?")
            }
            DetailsQueries::SelectPresences => {
                write!(f, "SELECT BeneficiaryId, DATE_FORMAT(PresenceDate, '%Y-%m-%d %h:%m:%s') AS Date FROM BeneficiaryPresences WHERE BeneficiaryId = ? Order By Date ASC")
            }
            DetailsQueries::SelectUserNotes => {
                write!(f, "SELECT BeneficiaryId, DATE_FORMAT(Date, '%Y-%m-%d %h:%m:%s') as Date, Type, Note FROM BeneficiaryNotes WHERE BeneficiaryId = ? AND Type = 0")
            }
            DetailsQueries::SelectAdminNotes => {
                write!(f, "SELECT BeneficiaryId, DATE_FORMAT(Date, '%Y-%m-%d %h:%m:%s') as Date, Type, Note FROM BeneficiaryNotes WHERE BeneficiaryId = ? AND Type = 0 OR Type = 1")
            }
            DetailsQueries::SelectTsNotes => {
                write!(f, "SELECT BeneficiaryId, DATE_FORMAT(Date, '%Y-%m-%d %h:%m:%s') as Date, Type, Note FROM BeneficiaryNotes WHERE BeneficiaryId = ? AND Type != 1")
            }
            DetailsQueries::InsertAllergy => {
                write!(f, "INSERT INTO BeneficiaryAllergies (BeneficiaryId, Allergy) VALUES (?, ?)")
            }
            DetailsQueries::DeleteAllergy => {
                write!(f, "DELETE FROM BeneficiaryAllergies WHERE BeneficiaryId = ? AND Allergy LIKE ? ESCAPE '#'")
            }
            DetailsQueries::InsertPresence => {
                write!(f, "INSERT INTO BeneficiaryPresences (BeneficiaryId, PresenceDate) VALUES (?, ?)")
            }
            DetailsQueries::DeletePresence => {
                write!(f, "DELETE FROM BeneficiaryPresences WHERE BeneficiaryId = ? AND PresenceDate = ?")
            }
            DetailsQueries::CreateNote => {
                write!(f, "INSERT INTO BeneficiaryNotes (BeneficiaryId, Date, Type, Note) VALUES (?, ?, ?, ?)")
            }
            DetailsQueries::UpdateNote => {
                write!(f, "UPDATE BeneficiaryNotes SET Note = ? WHERE BeneficiaryId = ? AND Date = ?")
            }
            DetailsQueries::DeleteNote => {
                write!(f, "DELETE FROM BeneficiaryNotes WHERE BeneficiaryId = ? AND Date = ?")
            }
        }

    }
}


#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize, Debug)]
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
        println!("->> {:>12} - Insert Allergy - {}", "Handler", self.Allergy.Allergy.clone());
        let _ = sqlx::query(&DetailsQueries::InsertAllergy.to_string())
            .bind(self.Allergy.BeneficiaryId)
            .bind(self.Allergy.Allergy.clone())
            .execute(conn.as_mut())
            .await.map_err(|e| {
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Insert Allergy - SUCCESS", "Handler");
        Ok(())
    }

    pub(crate) async fn delete_allergy(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        let _ = sqlx::query(&DetailsQueries::DeleteAllergy.to_string())
            .bind(self.Allergy.BeneficiaryId)
            .bind(self.Allergy.Allergy.clone())
            .execute(conn.as_mut())
            .await.map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Delete Allergy - SUCCESS", "Handler");
        Ok(())
    }
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize, Debug)]
pub(crate) struct BeneficiaryPresence{
    pub(crate) BeneficiaryId: i32,
    pub(crate) Date: String,
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct TokenPresence{
    pub(crate) Token: String,
    pub(crate) Presence: BeneficiaryPresence,
}


impl TokenPresence{
    pub(crate) async fn insert_presence(&self,mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        println!("->> {:>12} - Insert Presence", "Handler");
        let _ = sqlx::query(&DetailsQueries::InsertPresence.to_string())
            .bind(self.Presence.BeneficiaryId)
            .bind(self.Presence.Date.clone())
            .execute(conn.as_mut())
            .await.map_err(|e| {
            println!("->> {:>12} - Insert Presence -of FAILED: {:?}", "Handler", e);

            e
        })?;
        println!("->> {:>12} - Insert Presence - SUCCESS", "Handler");
        Ok(())
    }

    pub(crate) async fn delete_presence(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        println!("->> {:>12} - Delete Presence", "Handler");
        let _ = sqlx::query(&DetailsQueries::DeletePresence.to_string())
            .bind(self.Presence.BeneficiaryId)
            .bind(self.Presence.Date.clone())
            .execute(conn.as_mut())
            .await.map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Delete Presence - SUCCESS", "Handler");
        Ok(())
    }
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize, Debug)]
pub(crate) struct BeneficiaryNotes{
    pub(crate) BeneficiaryId: i32,
    pub(crate) Date: String,
    pub(crate) Type: i8,
    #[sqlx(default)]
    pub(crate) Note: String,
}

#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct TokenNote{
    pub(crate) Token: String,
    pub(crate) Content: BeneficiaryNotes,
}

impl TokenNote{
    pub(crate) async fn create_note(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        println!("->> {:>12} - Insert Note", "Handler");
        let _ = sqlx::query(&DetailsQueries::CreateNote.to_string())
            .bind(self.Content.BeneficiaryId)
            .bind(self.Content.Date.clone())
            .bind(self.Content.Type)
            .bind(self.Content.Note.clone())
            .execute(conn.as_mut())
            .await.map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Insert Note - SUCCESS", "Handler");
        Ok(())
    }

    pub(crate) async fn update_note(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        println!("->> {:>12} - Update Note", "Handler");
        let _ = sqlx::query(&DetailsQueries::UpdateNote.to_string())
            .bind(self.Content.Note.clone())
            .bind(self.Content.BeneficiaryId)
            .bind(self.Content.Date.clone())
            .execute(conn.as_mut())
            .await.map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Update Note - SUCCESS", "Handler");
        Ok(())
    }

    pub(crate) async fn delete_note(&self, mut conn: PoolConnection<MySql>) -> Result<(), Error>{
        println!("->> {:>12} - Delete Note", "Handler");
        let _ = sqlx::query(&DetailsQueries::DeleteNote.to_string())
            .bind(self.Content.BeneficiaryId)
            .bind(self.Content.Date.clone())
            .execute(conn.as_mut())
            .await.map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        println!("->> {:>12} - Delete Note - SUCCESS", "Handler");
        Ok(())
    }
}



#[derive(sqlx::FromRow, Encode, Decode, Serialize, Deserialize)]
pub(crate) struct Details{
    pub(crate) Id: i32,
    pub(crate) Presences: Vec<BeneficiaryPresence>,
    pub(crate) Allergies: Vec<BeneficiaryAllergy>,
    pub(crate) Notes: Vec<BeneficiaryNotes>,
}

impl Details {
    pub(crate) async fn get_details(mut conn: PoolConnection<MySql>, role: UserRole, id: i32) -> Result<Details, anyhow::Error>{
        println!("->> {:>12} - Get Details - Beneficiary : {id}", "Handler");
        let presences = Self::get_presences(conn.as_mut(), id).await?;
        let allergies = Self::get_allergies(conn.as_mut(), id).await?;
        let notes = Self::get_notes(conn.as_mut(), role, id).await?;
        println!("->> {:>12} - Get Details - SUCCESS", "Handler");
        Ok(
            Self{
                Id: id,
                Presences: presences,
                Allergies: allergies,
                Notes: notes,
            }
        )
    }

    async fn get_allergies(conn: &mut MySqlConnection, id: i32) -> Result<Vec<BeneficiaryAllergy>, anyhow::Error>{
        let allergies = sqlx::query_as(&DetailsQueries::SelectAllergies.to_string())
            .bind(id)
            .fetch_all(conn)
            .await
            .context("Could not get allergy list").map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        Ok(allergies)
    }

    async fn get_presences(conn: &mut MySqlConnection, id: i32) -> Result<Vec<BeneficiaryPresence>, anyhow::Error>{
        let presences = sqlx::query_as(&DetailsQueries::SelectPresences.to_string())
            .bind(id)
            .fetch_all(conn)
            .await
            .context("Could not get presence list").map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        Ok(presences)
    }
    async fn get_notes(conn: &mut MySqlConnection, role: UserRole, id: i32) -> Result<Vec<BeneficiaryNotes>, anyhow::Error>{
        let query = match role.Role.as_str() {
            "Admin" | "Dev" => {DetailsQueries::SelectAdminNotes}
            "TS" => {DetailsQueries::SelectTsNotes}
            _ => {DetailsQueries::SelectUserNotes}
        };

        let notes = sqlx::query_as(&query.to_string())
            .bind(id)
            .fetch_all(conn)
            .await
            .context("Could not get notes").map_err(|e|{
            println!("->> {:>12} - Error: {:?}", "Handler", e);
            e
        })?;
        Ok(notes)
    }
}
