use bincode::{Encode};
    use serde::{Deserialize, Serialize};
    use sqlx::MySql;
    use sqlx::pool::PoolConnection;
    use crate::schema::encode;

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Stats {
        pub(crate) Presences: Vec<Presence>,
        pub(crate) Ages: Vec<Age>,
        pub(crate) Cities: Vec<City>,
        pub(crate) Employments: Vec<Employment>,
        pub(crate) FamilySituations: Vec<FamilySituation>,
        pub(crate) Incomes: Vec<Income>,
        pub(crate) Kids: Vec<Kid>,
        pub(crate) Languages: Vec<Language>,
        pub(crate) Origins: Vec<Origin>,
        pub(crate) Sexes: Vec<Sexe>,
        pub(crate) Studies: Vec<Study>,
    }

    impl Stats{
        pub(crate) async fn get_stats(mut conn: PoolConnection<MySql>) -> Result<Vec<u8>, anyhow::Error> {
            let stats = Self {
                Presences: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Total, Active, Visits FROM Presence ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Ages: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Age_0_19, Age_20_29, Age_30_39, Age_40_49, Age_50_59, Age_60_69, Age_70_Plus FROM Age ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Cities: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Carignan, Chambly, Marieville, Richelieu, StMathias, Other FROM City ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Employments: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Unemployed, Employed FROM Employment ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                FamilySituations: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Single, Couple, CoupleKids, Recomposed, SingleParent, Other FROM FamilySituation ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Incomes: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, NoIncome, Income_1_14999, Income_15000_29999, Income_30000_More FROM Income ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Kids: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, NoKids, OneKid, TwoKids, ThreeToFourKids, FivePlusKids FROM Kid ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Languages: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, French, English, Spanish, Arabic, Mandarin, Other FROM Language ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Origins: sqlx::query_as("SELECT DATE_FORMAT(Date, '%Y-%m-%d') AS Date, NorthAmerican, SouthAmerican, CentralAmerican, Asian, African, European, Other FROM Origin ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Sexes: sqlx::query_as("SELECT  DATE_FORMAT(Date, '%Y-%m-%d') AS Date, Male, Female, Other FROM Sexe ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?,
                Studies: sqlx::query_as("SELECT  DATE_FORMAT(Date, '%Y-%m-%d') AS Date, NoStudy, PrimarySchool, HighSchool, College, University, Other FROM Study ORDER BY Date ASC")
                    .fetch_all(conn.as_mut())
                    .await?
            };
            match encode(stats) {
                Ok(b) => Ok(b),
                Err(( _ ,message)) => Err(anyhow::Error::msg(message))
            }
        }
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Presence {
        pub(crate) Date: String,
        pub(crate) Total: u32,
        pub(crate) Active: u32,
        pub(crate) Visits: u32,
    }

    #[derive(sqlx::FromRow, Encode,Serialize, Deserialize, Debug)]
    pub(crate) struct Age {
        pub(crate) Date: String,
        pub(crate) Age_0_19: u32,
        pub(crate) Age_20_29: u32,
        pub(crate) Age_30_39: u32,
        pub(crate) Age_40_49: u32,
        pub(crate) Age_50_59: u32,
        pub(crate) Age_60_69: u32,
        pub(crate) Age_70_Plus: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct City {
        pub(crate) Date: String,
        pub(crate) Carignan: u32,
        pub(crate) Chambly: u32,
        pub(crate) Marieville: u32,
        pub(crate) Richelieu: u32,
        pub(crate) StMathias: u32,
        pub(crate) Other: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Employment {
        pub(crate) Date: String,
        pub(crate) Unemployed: u32,
        pub(crate) Employed: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct FamilySituation {
        pub(crate) Date: String,
        pub(crate) Single: u32,
        pub(crate) Couple: u32,
        pub(crate) CoupleKids: u32,
        pub(crate) Recomposed: u32,
        pub(crate) SingleParent: u32,
        pub(crate) Other: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Income {
        pub(crate) Date: String,
        pub(crate) NoIncome: u32,
        pub(crate) Income_1_14999: u32,
        pub(crate) Income_15000_29999: u32,
        pub(crate) Income_30000_More: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Kid {
        pub(crate) Date: String,
        pub(crate) NoKids: u32,
        pub(crate) OneKid: u32,
        pub(crate) TwoKids: u32,
        pub(crate) ThreeToFourKids: u32,
        pub(crate) FivePlusKids: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Language {
        pub(crate) Date: String,
        pub(crate) French: u32,
        pub(crate) English: u32,
        pub(crate) Spanish: u32,
        pub(crate) Arabic: u32,
        pub(crate) Mandarin: u32,
        pub(crate) Other: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Origin {
        pub(crate) Date: String,
        pub(crate) NorthAmerican: u32,
        pub(crate) SouthAmerican: u32,
        pub(crate) CentralAmerican: u32,
        pub(crate) Asian: u32,
        pub(crate) African: u32,
        pub(crate) European: u32,
        pub(crate) Other: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Study {
        pub(crate) Date: String,
        pub(crate) NoStudy: u32,
        pub(crate) PrimarySchool: u32,
        pub(crate) HighSchool: u32,
        pub(crate) College: u32,
        pub(crate) University: u32,
        pub(crate) Other: u32,
    }

    #[derive(sqlx::FromRow, Encode, Serialize, Deserialize, Debug)]
    pub(crate) struct Sexe {
        pub(crate) Date: String,
        pub(crate) Male: u32,
        pub(crate) Female: u32,
        pub(crate) Other: u32,
    }