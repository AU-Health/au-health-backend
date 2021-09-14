use argon2::{Argon2, PasswordHash};
use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use argon2::PasswordVerifier;

#[derive(InputObject, Clone)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(SimpleObject, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn verify_password(
        &self,
        argon2: &Argon2,
        password: String,
    ) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(&self.password)?;

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
