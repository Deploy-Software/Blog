use async_graphql::{Error, Result};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewSession {
    pub token: String,
}

impl<'a> NewSession {
    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    fn generate_token() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect()
    }

    pub fn make() -> Self {
        NewSession {
            token: Self::generate_token(),
        }
    }

    pub async fn insert(&self, pg_pool: &PgPool, user_id: i32) -> Result<PgQueryResult> {
        match sqlx::query!(
            "INSERT INTO user_sessions (user_id, token) VALUES ($1, $2)",
            user_id,
            &self.token
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert user session in database."))
            }
        }
    }
}