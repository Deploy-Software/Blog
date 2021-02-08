use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub text: String,
    pub summary: String,
    pub created_at: DateTime<chrono::Utc>,
}

impl<'a> Post {
    pub async fn all(pg_pool: &PgPool) -> Result<Vec<Self>> {
        match sqlx::query_as!(
            Self,
            r#"
            SELECT
                posts.id,
                posts.slug,
                posts.title,
                posts.text,
                posts.summary,
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

    pub async fn get(pg_pool: &PgPool, post_id: i32) -> Result<Option<Self>> {
        match sqlx::query_as!(
            Self,
            r#"
            SELECT
                posts.id,
                posts.slug,
                posts.title,
                posts.text,
                posts.summary,
                posts.created_at
            FROM
                posts
            WHERE
                posts.id = $1
            "#,
            post_id
        )
        .fetch_optional(pg_pool)
        .await
        {
            Ok(post) => match post {
                Some(exists) => Ok(Some(exists)),
                None => Ok(None),
            },
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "An error occured while retrieving the posts from the database.",
                ))
            }
        }
    }

    pub async fn update(pg_pool: &PgPool, post_id: i32, _user_id: i32, title: Option<String>, text: Option<String>) -> Result<()> {
        match title {
            Some(updated_title) => {
                match sqlx::query_as!(
                    Self,
                    r#"
                    UPDATE
                        posts
                    SET
                        title = $1
                    WHERE
                        id = $2
                    "#,
                    updated_title,
                    post_id
                )
                .execute(pg_pool)
                .await
                {
                    Ok(_post) => {},
                    Err(error) => {
                        println!("{}", error.to_string());
                        return Err(Error::from(
                            "An error occured while updating the post title in the database.",
                        ));
                    }
                }
            },
            None => {}
        }

        match text {
            Some(updated_text) => {
                match sqlx::query_as!(
                    Self,
                    r#"
                    UPDATE
                        posts
                    SET
                        text = $1
                    WHERE
                        id = $2
                    "#,
                    updated_text,
                    post_id
                )
                .execute(pg_pool)
                .await
                {
                    Ok(_post) => {},
                    Err(error) => {
                        println!("{}", error.to_string());
                        return Err(Error::from(
                            "An error occured while updating the post text in the database.",
                        ));
                    }
                }
            },
            None => {}
        }
        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewPost<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub text: &'a str,
    pub summary: &'a str,
    pub created_by: i32,
}

impl<'a> NewPost<'a> {
    pub fn new(slug: &'a str, title: &'a str, text: &'a str, summary: &'a str, created_by: i32) -> Result<Self> {
        Ok(Self { slug, title, text, summary, created_by })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<Post> {
        match sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts
                (slug, title, text, summary, created_by)
            VALUES
                ($1, $2, $3, $4, $5)
            RETURNING
                id,
                slug,
                title,
                text,
                summary,
                created_at
            "#,
            &self.slug,
            &self.title,
            &self.text,
            &self.summary,
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
