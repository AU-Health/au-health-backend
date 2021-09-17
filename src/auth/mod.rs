mod session;

use async_graphql::Error;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
pub use session::*;
use uuid::Uuid;

use crate::domain::user::{Role, User};

pub struct Authorizer<'a> {
    session_store: &'a RedisSessionStore,
}

const NO_COOKIE_PRESENT_ERROR: Error = Error::new("No cookie present");

impl<'a> Authorizer<'a> {
    pub fn new(session_store: &'a RedisSessionStore) -> Self {
        Self { session_store }
    }

    async fn load_session(&self, auth_session_cookie: &SessionCookie) -> Result<Session, Error> {
        self.session_store
            .load_session(auth_session_cookie.cookie_value.clone())
            .await?
            .ok_or_else(|| Error::new("Session not found"))
    }

    pub async fn user_id(&self, auth_session_cookie: &SessionCookie) -> Result<Uuid, Error> {
        let session = self.load_session(auth_session_cookie).await?;

        session
            .get::<Uuid>(USER_ID_SESSION_KEY)
            .ok_or_else(|| Error::new("User ID not set in Session"))
    }

    /// Check if user is logged in. Returns `Ok(true)` if found, `Ok(false)` if not.
    /// Errors if the cookie is present, but session is not found, or the User ID did not get set in the Session.
    pub async fn logged_in(
        &self,
        auth_session_cookie: &Option<SessionCookie>,
    ) -> Result<bool, Error> {
        match auth_session_cookie {
            None => Ok(false),
            Some(cookie) => self.user_id(cookie).await.and(Ok(true)),
        }
    }

    pub async fn create_session(&self, user: &User) -> Result<SessionCookie, Error> {
        let mut session = Session::new();
        session
            .insert(USER_ID_SESSION_KEY, user.id)
            .map_err(|err| Error::new(format!("Unable to create user session: {:?}", err)))?;

        let cookie_value = self
            .session_store
            .store_session(session)
            .await?
            .ok_or("Cookie value empty")?;

        Ok(SessionCookie { cookie_value })
    }

    // pub async fn authorized(
    //     roles: Vec<Role>,
    //     auth_session_cookie: &Option<AuthSessionCookie>,
    // ) -> Result<bool, Error> {
    // }
}
