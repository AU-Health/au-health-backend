use argon2::Argon2;
use async_graphql::Context;
use async_redis_session::RedisSessionStore;
use sqlx::{Pool, Postgres};

use crate::auth::{Authorizer, SessionCookie};

pub struct ContextData<'a> {
    pub db_pool: &'a Pool<Postgres>,
    pub argon2: &'a Argon2<'a>,
    pub session_cookie: &'a Option<SessionCookie>,
    pub authorizer: Authorizer<'a>,
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
            .data::<Option<SessionCookie>>()
            .expect("Auth Cookie Option not found in Context");

        let authorizer = Authorizer { session_store };

        Self {
            db_pool,
            argon2,
            session_cookie: auth_session_cookie,
            authorizer,
        }
    }
}
