use async_graphql::{Error, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(SimpleObject)]
pub struct User {
    id: Uuid,
    username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub async fn login(username: String, _password: String) -> Result<User, Error> {
    Ok(User {
        id: Uuid::new_v4(),
        username,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
