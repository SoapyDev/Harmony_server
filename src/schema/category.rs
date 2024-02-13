#![allow(non_snake_case)]
use std::fmt::Display;
use axum::http::StatusCode;
use bincode::Encode;
use sqlx::Error;
use crate::schema::encode;

enum CategoryQueries{
    SelectCategories,
    CreateCategory,
    UpdateCategory,
    DeleteCategory,
}

impl Display for CategoryQueries{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            CategoryQueries::SelectCategories => write!(f, "SELECT * FROM Categories"),
            CategoryQueries::CreateCategory => write!(f, "INSERT INTO Categories (Category, MonthlyFee, WeeklyFee) VALUES (?, ?, ?)"),
            CategoryQueries::UpdateCategory => write!(f, "UPDATE Categories SET `Category` = ?, `MonthlyFee` = ?, `WeeklyFee` = ? WHERE `Id` = ?"),
            CategoryQueries::DeleteCategory => write!(f, "DELETE FROM Categories WHERE `Id` = ?"),
        }
    }
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, Encode)]
pub(crate) struct Categories {
    pub(crate) Id: i32,
    pub(crate) Category: String,
    pub(crate) MonthlyFee: f32,
    pub(crate) WeeklyFee: f32,
    pub(crate) UsedBy : u32,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct TokenCategory{
    pub(crate) Token: String,
    pub(crate) Category: Categories,
}


impl Categories {
    pub(crate) async fn select_categories(mut conn : sqlx::pool::PoolConnection<sqlx::MySql>) -> Result<Vec<u8>, (StatusCode, String)>{
        println!("->> {:>12} - Select categories", "Handler");
        let categories: Result<Vec<Categories>, Error> = sqlx::query_as(&CategoryQueries::SelectCategories.to_string())
            .fetch_all(conn.as_mut())
            .await.map_err(|e| {
                println!("->> {:>12} - Select categories - FAILED : {}", "Handler", e);
                e
            });
        match categories {
            Ok(categories) => {
                println!("->> {:>12} - Select categories - SUCCESS", "Handler");
                if categories.is_empty(){
                    println!("->> {:>12} - Select categories - FAILED : No categories found", "Handler");
                    return Err((StatusCode::NOT_FOUND, "No categories found".to_string()));
                }
                encode(categories)
            },
            Err(e) => {
                println!("->> {:>12} - Select categories - FAILED : {}", "Handler", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not find categories".to_string()))
            },
        }
    }

    pub(crate) async fn create_category(&self, mut conn : sqlx::pool::PoolConnection<sqlx::MySql>) -> Result<Vec<u8>, (StatusCode, String)>{
        println!("->> {:>12} - Create category", "Handler");
        let result = sqlx::query(&CategoryQueries::CreateCategory.to_string())
            .bind(self.Category.clone())
            .bind(self.MonthlyFee)
            .bind(self.WeeklyFee)
            .execute(conn.as_mut())
            .await;

        match result {
            Ok(_) => {
                println!("->> {:>12} - Create category - SUCCESS", "Handler");
                Self::select_categories(conn).await
            },
            Err(e) => {
                println!("->> {:>12} - Create category - FAILED : {}", "Handler", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create category".to_string()))
            }
        }
    }

    pub(crate) async fn update_category(&self, mut conn : sqlx::pool::PoolConnection<sqlx::MySql>) -> Result<Vec<u8>, (StatusCode, String)>{
        let result = sqlx::query(&CategoryQueries::UpdateCategory.to_string())
            .bind(self.Category.clone())
            .bind(self.MonthlyFee)
            .bind(self.WeeklyFee)
            .bind(self.Id)
            .execute(conn.as_mut())
            .await;

        match result {
            Ok(_) => {
                println!("->> {:>12} - Update category - SUCCESS", "Handler");
                Self::select_categories(conn).await
            },
            Err(e) => {
                println!("->> {:>12} - Update category - FAILED : {}", "Handler", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not update category".to_string()))
            }
        }
    }

    pub(crate) async fn delete_category(&self, mut conn : sqlx::pool::PoolConnection<sqlx::MySql>) -> Result<Vec<u8>, (StatusCode, String)>{
        let result = sqlx::query(&CategoryQueries::DeleteCategory.to_string())
            .bind(self.Id)
            .execute(conn.as_mut())
            .await;

        match result {
            Ok(_) => {
                println!("->> {:>12} - Delete category - SUCCESS", "Handler");
                Self::select_categories(conn).await
            },
            Err(e) => {
                println!("->> {:>12} - Delete category - FAILED : {}", "Handler", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not delete category".to_string()))
            }
        }
    }
}