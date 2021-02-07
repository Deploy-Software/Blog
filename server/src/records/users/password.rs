use async_graphql::{Error, Result};
use bcrypt::{hash, DEFAULT_COST};
use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};

#[derive(sqlx::FromRow)]
pub struct UserPassword {
    pub id: i32,
    pub password: String,
    pub date: DateTime<chrono::Utc>,
}

impl<'a> UserPassword {
    pub fn get_password_hash(&self) -> String {
        self.password.clone()
    }

    pub async fn from(pg_pool: &PgPool, user_id: i32) -> Result<Option<Self>> {
        match sqlx::query_as!(
            Self,
            "SELECT id, password, date FROM user_password WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pg_pool)
        .await
        {
            Ok(user_password) => Ok(user_password),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "An error occured while retrieving the password from the database.",
                ))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewPassword {
    pub hashed_password: String,
}

impl<'a> NewPassword {
    pub fn new(password: &'a str) -> Result<Self> {
        // TODO Implement better password validation
        let re = match Regex::new(r"(^[a-zA-Z0-9]{8,}$)") {
            Ok(re) => re,
            Err(error) => {
                println!("{}", error.to_string());
                return Err(Error::from("Password regex could not be compiled."));
            }
        };

        if !re.is_match(password) {
            return Err(Error::from("Password is not secure enough."));
        }

        let hashed_password = match hash(&password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(error) => {
                println!("{}", error.to_string());
                return Err(Error::from("Could not hash password."));
            }
        };

        Ok(NewPassword { hashed_password })
    }

    pub async fn insert(&self, pg_pool: &PgPool, user_id: i32) -> Result<PgQueryResult> {
        match sqlx::query!(
            "INSERT INTO user_password(user_id, password) VALUES($1, $2)",
            user_id,
            &self.hashed_password
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert user password in database."))
            }
        }
    }
}