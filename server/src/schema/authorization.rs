use crate::records::authorities::Authorities;
use crate::records::users::{session::NewSession, NewUser, SimpleUser};
use crate::AuthToken;
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn get<'a>(ctx: &'a Context<'_>) -> Result<Option<Authorities>> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = ctx.data_opt::<AuthToken>();
    match token {
        Some(token) => Authorities::get(&pg_pool, &token.0).await,
        None => Ok(None),
    }
}

pub async fn sign_up<'a>(
    ctx: &'a Context<'_>,
    email: String,
    name: String,
    password: String,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_user = NewUser::new(&email, &name, &password)?;
    new_user.insert(&pg_pool).await?;
    Ok("OK")
}

pub async fn sign_in<'a>(ctx: &'a Context<'_>, email: String, password: String) -> Result<String> {
    let pg_pool = ctx.data::<PgPool>()?;
    let user = SimpleUser::from_email(&pg_pool, &email).await?;
    if !user.password_matches(&password).await? {
        return Err(Error::from("The email and password combination failed."));
    }
    let user_session = NewSession::make();
    user_session.insert(&pg_pool, user.id).await?;
    Ok(user_session.get_token())
}
