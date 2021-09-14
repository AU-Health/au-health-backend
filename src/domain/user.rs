use argon2::{Argon2, PasswordHash};
use async_graphql::{Error, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use argon2::PasswordVerifier;

#[derive(InputObject)]
/// Input from GQL, consume with login_user() to get a User.
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl LoginUser {
    pub async fn login_user(
        self,
        pool: &Pool<Postgres>,
        argon2: &Argon2<'_>,
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password, created_at, updated_at
        FROM users
        WHERE email = $1
        LIMIT 1;",
            self.email
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        let valid = user.verify_password(argon2, self.password).map_err(|e| {
            tracing::info!("Failed to verify password");
            e
        })?;

        match valid {
            false => Err(Error::new("Invalid password")),
            true => Ok(user),
        }
    }
}

#[derive(SimpleObject, Debug)]
/// User with all fields attached. Returned from DB.
pub struct User {
    pub id: Uuid,
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

    pub async fn query_by_id(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password, created_at, updated_at
        FROM users
        WHERE id = $1
        LIMIT 1;",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

        Ok(user)
    }
}
