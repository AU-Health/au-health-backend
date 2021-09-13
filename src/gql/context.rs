use argon2::Argon2;
use async_graphql::Context;
use async_redis_session::RedisSessionStore;
use sqlx::{Pool, Postgres};

use crate::auth::AuthCookies;

pub struct ContextData<'a> {
    pub db_pool: &'a Pool<Postgres>,
    pub argon2: &'a Argon2<'a>,
    pub auth_cookie: &'a AuthCookies,
    pub session_store: &'a RedisSessionStore,
}

pub async fn get_data_from_ctx<'a>(ctx: &Context<'a>) -> ContextData<'a> {
    let db_pool = ctx
        .data::<Pool<Postgres>>()
        .expect("DB Pool not found in Context");
    let session_store = ctx
        .data::<RedisSessionStore>()
        .expect("Session store not found in Context");
    let argon2 = ctx.data::<Argon2>().expect("Argon2 not found in Context");
    let auth_cookie = ctx
        .data::<AuthCookies>()
        .expect("Auth Cookie Option not found in Context");

    ContextData {
        db_pool,
        argon2,
        auth_cookie,
        session_store,
    }
}
