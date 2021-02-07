use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub created_at: DateTime<chrono::Utc>,
}

impl<'a> Post {
    pub async fn all(pg_pool: &PgPool) -> Result<Vec<Self>> {
        match sqlx::query_as!(
            Self,
            r#"
            SELECT
                posts.id,
                posts.title,
                posts.text,
                posts.created_at
            FROM
                posts
            "#,
        )
        .fetch_all(pg_pool)
        .await
        {
            Ok(posts) => Ok(posts),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "An error occured while retrieving the posts from the database.",
                ))
            }
        }
    }
}