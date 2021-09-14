use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::Error;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::user::{LoginUser, NewUser, User};

impl User {
    pub async fn query_by_id(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Self, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, username, password, created_at, updated_at
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

pub async fn login_user(
    pool: &Pool<Postgres>,
    argon2: &Argon2<'_>,
    login_user: LoginUser,
) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, username, password, created_at, updated_at
    FROM users
    WHERE username = $1
    LIMIT 1;",
        login_user.username
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    let valid = user
        .verify_password(argon2, login_user.password)
        .map_err(|e| {
            tracing::info!("Failed to verify password");
            e
        })?;

    match valid {
        false => Err(Error::new("Invalid password")),
        true => Ok(user),
    }
}

pub async fn register_user(
    pool: &Pool<Postgres>,
    argon2: &Argon2<'_>,
    new_user: NewUser,
) -> Result<User, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon2
        .hash_password(new_user.password.as_bytes(), &salt)?
        .to_string();

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, username, password, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5, $6) 
    RETURNING *;",
        Uuid::new_v4(),
        new_user.email,
        new_user.username,
        hashed_password,
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(user)
}
