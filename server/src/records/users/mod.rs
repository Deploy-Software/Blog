use async_graphql::{Error, Result, SimpleObject};
use bcrypt::verify;
use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod password;
pub mod session;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleUser {
    pub id: i32,
    pub email: String,
    pub date: DateTime<chrono::Utc>,
}

impl<'a> SimpleUser {
    pub async fn from_email(pg_pool: &PgPool, email: &'a str) -> Result<Self> {
        match sqlx::query_as!(
            Self,
            "SELECT id, email, date FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pg_pool)
        .await
        {
            Ok(maybe_user) => match maybe_user {
                Some(user) => Ok(user),
                None => Err(Error::from("The email and password combination failed.")),
            },
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "An error occured while retrieving the user from the database.",
                ))
            }
        }
    }

    pub async fn password_matches(
        &self,
        pg_pool: &PgPool,
        password_to_test: &'a str,
    ) -> Result<bool> {
        let user_password = password::UserPassword::from(pg_pool, self.id).await?;
        match user_password {
            Some(user_pass) => match verify(password_to_test, &user_pass.get_password_hash()) {
                Ok(matches) => Ok(matches),
                Err(error) => {
                    println!("{}", error.to_string());
                    Err(Error::from(
                        "We were unable compare the password with our saved password.",
                    ))
                }
            },
            None => Err(Error::from("You don't have a password.")),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str) -> Result<Self> {
        let re = match Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)") {
            Ok(re) => re,
            Err(error) => {
                println!("{}", error.to_string());
                return Err(Error::from("Email regex could not be compiled."));
            }
        };

        if !re.is_match(email) {
            return Err(Error::from("Email is not valid."));
        }

        Ok(NewUser { email })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleUser> {
        match sqlx::query_as!(
            SimpleUser,
            "INSERT INTO users(email) VALUES($1) RETURNING id, email, date",
            &self.email
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert user in database."))
            }
        }
    }
}
