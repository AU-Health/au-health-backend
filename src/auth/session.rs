use crate::domain::user::User;
use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use headers::Cookie;
use http::header::SET_COOKIE;
use uuid::Uuid;

pub const AUTH_COOKIE_NAME: &str = "auth";
pub const USER_ID_SESSION_KEY: &str = "user_id";

pub struct AuthSessionCookie {
    pub cookie_value: String,
}

#[async_trait]
impl<B> FromRequest<B> for AuthSessionCookie
where
    B: Send,
{
    type Rejection = String;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie_headers = TypedHeader::<Cookie>::from_request(req).await.ok();

        match cookie_headers {
            None => Err("Cookie header not set".to_string()),
            Some(cookies) => cookies
                .get(AUTH_COOKIE_NAME)
                .map(|s| s.to_string())
                .ok_or_else(|| "Auth cookie not set".to_string())
                .map(|s| Self { cookie_value: s }),
        }
    }
}

impl AuthSessionCookie {
    /// Creates user session.
    pub async fn create_session(
        user: &User,
        session_store: &RedisSessionStore,
    ) -> Result<Self, Error> {
        let mut session = Session::new();
        session
            .insert(USER_ID_SESSION_KEY, user.id)
            .map_err(|err| Error::new(format!("Unable to create user session: {:?}", err)))?;

        let cookie_value = session_store
            .store_session(session)
            .await?
            .ok_or("Cookie value empty")?;

        Ok(Self { cookie_value })
    }

    /// Uses GQL Context to set session cookie on the browser.
    pub async fn create_cookie(&self, ctx: &Context<'_>) -> Result<(), Error> {
        ctx.append_http_header(
            SET_COOKIE,
            format!("{}={}; SameSite=Lax", AUTH_COOKIE_NAME, self.cookie_value),
        );

        Ok(())
    }

    /// Load actual session from Redis/Session Store.
    pub async fn load_session(
        &self,
        session_store: &RedisSessionStore,
    ) -> Result<Option<Session>, async_session::Error> {
        session_store.load_session(self.cookie_value.clone()).await
    }
}

#[async_trait]
/// Just to add easy get_user_id method to Session.
pub trait GetUserId {
    async fn get_user_id(&self) -> Result<Uuid, Error>;
}

#[async_trait]
impl GetUserId for Session {
    async fn get_user_id(&self) -> Result<Uuid, Error> {
        let user_id = self
            .get::<Uuid>("user_id")
            .ok_or("User ID not set in Session")?;

        Ok(user_id)
    }
}
