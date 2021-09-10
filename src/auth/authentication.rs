use async_graphql::{Error, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::gql::NewUser;

#[derive(SimpleObject, Debug)]
pub struct User {
    id: Uuid,
    username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub async fn login_user(username: String, _password: String) -> Result<User, Error> {
    Ok(User {
        id: Uuid::new_v4(),
        username,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

pub async fn register_user(pool: &Pool<Postgres>, new_user: NewUser) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, password, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5) RETURNING id, username, created_at, updated_at;",
        Uuid::new_v4(),
        new_user.username,
        new_user.password,
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
