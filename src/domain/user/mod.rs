mod registration;
pub use registration::*;
mod role;
use argon2::{Argon2, PasswordHash};
use async_graphql::{Error, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
pub use role::*;
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
            r#"SELECT id, email, password, created_at, updated_at, role as "role: _"
        FROM user_account
        WHERE email = $1
        LIMIT 1;"#,
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
#[derive(SimpleObject, Debug, Clone)]
/// User with all fields attached. Returned from DB.
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: Role,
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

    pub async fn change_role(&self, pool: &Pool<Postgres>, new_role: Role) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            r#"UPDATE user_account SET role = $1 WHERE id = $2 RETURNING id, email, password, created_at, updated_at, role as "role: _";"#,
            new_role as Role,
            self.id,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn query_by_id(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, email, password, created_at, updated_at, role as "role: _"
        FROM user_account
        WHERE id = $1
        LIMIT 1;"#,
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

    pub async fn query_by_email(pool: &Pool<Postgres>, email: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, email, password, created_at, updated_at, role as "role: _"
        FROM user_account
        WHERE email = $1
        LIMIT 1;"#,
            email
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
