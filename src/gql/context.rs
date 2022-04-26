use argon2::Argon2;
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use sqlx::{Pool, Postgres};

use crate::session::{SessionCookie, SessionManager};

pub struct ParsedContext<'a> {
    pub db_pool: &'a Pool<Postgres>,
    pub argon2: &'a Argon2<'a>,
    session_cookie: &'a Option<SessionCookie>,
    pub session_manager: SessionManager<'a>,
}

fn not_logged_in_error() -> Error {
    Error::new("Not logged in")
}


//this whole file is basically dedicated to creating the cookie that tracks whether someone is logged in or not.
//the context is used throughout method calls to track where things came from (which user)

impl<'a> ParsedContext<'a> {
    pub fn new(ctx: &Context<'a>) -> Self {
        let db_pool = ctx
            .data::<Pool<Postgres>>()
            .expect("DB Pool not found in Context");
        let store = ctx
            .data::<RedisSessionStore>()
            .expect("Session store not found in Context");
        let argon2 = ctx.data::<Argon2>().expect("Argon2 not found in Context");
        let auth_session_cookie = ctx
            .data::<Option<SessionCookie>>()
            .expect("Auth Cookie Option not found in Context");

        let authorizer = SessionManager::new(store);

        Self {
            db_pool,
            argon2,
            session_cookie: auth_session_cookie,
            session_manager: authorizer,
        }
    }

    /// Gets the SessionCookie or errors if no cookie is found.
    pub fn get_cookie(&self) -> Result<&SessionCookie, Error> {
        self.session_cookie.as_ref().ok_or_else(not_logged_in_error)
    }
}
