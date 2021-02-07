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

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub created_by: i32,
}

impl<'a> NewPost<'a> {
    pub fn new(title: &'a str, text: &'a str, created_by: i32) -> Result<Self> {
        Ok(Self { title, text, created_by })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<Post> {
        match sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts
                (text, title, created_by)
            VALUES
                ($1, $2, $3)
            RETURNING
                id,
                title,
                text,
                created_at
            "#,
            &self.title,
            &self.text,
            &self.created_by
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(post) => Ok(post),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert post in database."))
            }
        }
    }
}
