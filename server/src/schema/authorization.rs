use crate::records::users::{password::NewPassword, session::NewSession, NewUser, SimpleUser};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn sign_up<'a>(ctx: &'a Context<'_>, email: String, password: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_user = NewUser::new(&email)?;
    let user = new_user.insert(&pg_pool).await?;
    let new_password = NewPassword::new(&password)?;
    new_password.insert(&pg_pool, user.id).await?;
    Ok("OK")
}

pub async fn sign_in<'a>(ctx: &'a Context<'_>, email: String, password: String) -> Result<String> {
    let pg_pool = ctx.data::<PgPool>()?;
    let user = SimpleUser::from_email(&pg_pool, &email).await?;
    if !user.password_matches(&pg_pool, &password).await? {
        return Err(Error::from("The email and password combination failed."));
    }
    let user_session = NewSession::make();
    user_session.insert(&pg_pool, user.id).await?;
    Ok(user_session.get_token())
}
