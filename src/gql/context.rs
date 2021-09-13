use argon2::Argon2;
use async_graphql::Context;
use async_redis_session::RedisSessionStore;
use sqlx::{Pool, Postgres};

use crate::auth::AuthSessionCookie;

pub struct ContextData<'a> {
    pub db_pool: &'a Pool<Postgres>,
    pub argon2: &'a Argon2<'a>,
    pub auth_session_cookie: &'a Option<AuthSessionCookie>,
    pub session_store: &'a RedisSessionStore,
}

impl<'a> ContextData<'a> {
    pub fn new(ctx: &Context<'a>) -> Self {
        let db_pool = ctx
            .data::<Pool<Postgres>>()
            .expect("DB Pool not found in Context");
        let session_store = ctx
            .data::<RedisSessionStore>()
            .expect("Session store not found in Context");
        let argon2 = ctx.data::<Argon2>().expect("Argon2 not found in Context");
        let auth_session_cookie = ctx
            .data::<Option<AuthSessionCookie>>()
            .expect("Auth Cookie Option not found in Context");

        Self {
            db_pool,
            argon2,
            auth_session_cookie,
            session_store,
        }
    }
}
