use super::{SessionCookie, USER_ID_SESSION_KEY};
use crate::domain::user::User;
use async_graphql::Error;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use uuid::Uuid;

pub struct SessionManager<'a> {
    store: &'a RedisSessionStore,
}

impl<'a> SessionManager<'a> {
    /// TODO: Stub function atm. Waiting on https://github.com/jbr/async-redis-session/pull/15
    pub fn ping(&self) -> Result<(), Error> {
        // let store = self.store.connection();

        Ok(())
    }

    /// Creates new `SessionManager` using a `RedisSessionStore`.
    pub fn new(store: &'a RedisSessionStore) -> Self {
        Self { store }
    }

    async fn load_session(&self, session_cookie: &SessionCookie) -> Result<Session, Error> {
        self.store
            .load_session(session_cookie.value.clone())
            .await?
            .ok_or_else(|| Error::new("Session not found"))
    }

    /// Get user's id from the Session.
    pub async fn user_id(&self, session_cookie: &SessionCookie) -> Result<Uuid, Error> {
        let session = self.load_session(session_cookie).await?;

        session
            .get::<Uuid>(USER_ID_SESSION_KEY)
            .ok_or_else(|| Error::new("User ID not set in Session"))
    }

    /// Check if user is logged in. Returns `Ok(true)` if found, `Ok(false)` if not.
    /// Errors if the cookie is present, but session is not found, or the User ID did not get set in the Session.
    pub async fn logged_in(&self, session_cookie: &Option<SessionCookie>) -> Result<bool, Error> {
        match session_cookie {
            None => Ok(false),
            Some(cookie) => self.user_id(cookie).await.and(Ok(true)),
        }
    }

    /// Creates a new user session and returns the cookie to apply to client.
    pub async fn create_session(&self, user: &User) -> Result<SessionCookie, Error> {
        let mut session = Session::new();
        session
            .insert(USER_ID_SESSION_KEY, user.id)
            .map_err(|err| Error::new(format!("Unable to create user session: {:?}", err)))?;

        let value = self
            .store
            .store_session(session)
            .await?
            .ok_or("Cookie value empty")?;

        Ok(SessionCookie { value })
    }
}
