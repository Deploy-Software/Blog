use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<chrono::Utc>,
}

impl<'a> Settings {
    pub async fn all(pg_pool: &PgPool) -> Result<Vec<Self>> {
        match sqlx::query_as!(
            Self,
            r#"
            SELECT
                settings.id,
                settings.key,
                settings.value,
                settings.created_at
            FROM
                settings
            "#,
        )
        .fetch_all(pg_pool)
        .await
        {
            Ok(settings) => Ok(settings),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "An error occured while retrieving the settings from the database.",
                ))
            }
        }
    }
}