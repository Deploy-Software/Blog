use async_graphql::{Error, Result, SimpleObject};
use bcrypt::{hash, DEFAULT_COST, verify};
use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod session;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleUser {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub date: DateTime<chrono::Utc>,
}

impl<'a> SimpleUser {
    pub async fn from_email(pg_pool: &PgPool, email: &'a str) -> Result<Self> {
        match sqlx::query_as!(
            Self,
            "SELECT id, email, password, date FROM users WHERE email = $1",
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
        password_to_test: &'a str,
    ) -> Result<bool> {
        match verify(password_to_test, &self.password) {
            Ok(matches) => Ok(matches),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "We were unable compare the password with our saved password.",
                ))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: String,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str, password: &'a str) -> Result<Self> {
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

        Ok(NewUser { email, password: hashed_password })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleUser> {
        match sqlx::query_as!(
            SimpleUser,
            "INSERT INTO users(email, password) VALUES($1, $2) RETURNING id, email, password, date",
            &self.email,
            &self.password
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
