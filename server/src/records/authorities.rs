use crate::records::users::SimpleUser;
use async_graphql::{Result, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct Authorities {
    valid_token: bool,
}

impl Authorities {
    pub async fn get(pg_pool: &PgPool, session_token: &str) -> Result<Option<Self>> {
        let user = SimpleUser::from_session_token(pg_pool, session_token).await?;
        Ok(Some(Self {
            valid_token: user.is_some(),
        }))
    }
}
