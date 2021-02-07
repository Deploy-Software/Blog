use crate::records::posts::Post;
use async_graphql::{Context, Result};
use sqlx::PgPool;

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<Vec<Post>> {
    let pg_pool = ctx.data::<PgPool>()?;
    Post::all(&pg_pool).await
}
