use argon2::Argon2;
use async_graphql::{Context, Error, Object};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    auth,
    domain::user::{LoginUser, NewUser, User},
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, _ctx: &Context<'_>) -> Result<User, Error> {
        Ok(User {
            id: Uuid::new_v4(),
            username: "test".to_owned(),
            password: "1234".to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Login User credentials")] login_user: LoginUser,
    ) -> Result<User, Error> {
        let db_pool = ctx.data::<Pool<Postgres>>().unwrap();
        let argon2 = ctx.data::<Argon2>().unwrap();

        auth::login_user(db_pool, argon2, login_user).await
    }

    async fn register(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "New User information")] new_user: NewUser,
    ) -> Result<User, Error> {
        let db_pool = ctx.data::<Pool<Postgres>>().unwrap();
        let argon2 = ctx.data::<Argon2>().unwrap();

        auth::register_user(db_pool, argon2, new_user).await
    }
}
