use crate::records::posts::{Post, NewPost};
use async_graphql::{Context, Result};
use sqlx::PgPool;

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<Vec<Post>> {
    let pg_pool = ctx.data::<PgPool>()?;
    Post::all(&pg_pool).await
}

pub async fn new<'a>(ctx: &'a Context<'_>, title: &'a str, text: &'a str) -> Result<Post> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_post = NewPost::new(title, text, 1)?;
    new_post.insert(&pg_pool).await
}