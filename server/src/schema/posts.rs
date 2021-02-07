use crate::records::posts::{Post, NewPost};
use crate::records::users::SimpleUser;
use crate::AuthToken;
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<Vec<Post>> {
    let pg_pool = ctx.data::<PgPool>()?;
    Post::all(&pg_pool).await
}

pub async fn new<'a>(ctx: &'a Context<'_>, title: &'a str, text: &'a str) -> Result<Post> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_post = NewPost::new(title, text, user.id)?;
    new_post.insert(&pg_pool).await
}