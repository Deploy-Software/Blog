use {
    crate::{
        records::{
            posts::{NewPost, Post},
            users::SimpleUser,
        },
        AuthToken,
    },
    async_graphql::{Context, Error, Result},
    sqlx::PgPool,
};

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<Vec<Post>> {
    let pg_pool = ctx.data::<PgPool>()?;
    Post::all(&pg_pool).await
}

pub async fn get<'a>(ctx: &'a Context<'_>, post_id: i32) -> Result<Option<Post>> {
    let pg_pool = ctx.data::<PgPool>()?;
    Post::get(&pg_pool, post_id).await
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

pub async fn update<'a>(ctx: &'a Context<'_>, post_id: i32, title: Option<String>, text: Option<String>) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    Post::update(&pg_pool, post_id, user.id, title, text).await?;
    Ok("Updated!")
}