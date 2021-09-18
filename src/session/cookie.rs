use async_graphql::{Context, Error};
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use headers::Cookie;
use http::header::SET_COOKIE;

use super::AUTH_COOKIE_NAME;

pub struct SessionCookie {
    pub value: String,
}

#[async_trait]
impl<B> FromRequest<B> for SessionCookie
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
                .map(|s| Self { value: s }),
        }
    }
}

impl SessionCookie {
    /// Uses GQL Context to set session cookie on the browser.
    pub async fn set_cookie(&self, ctx: &Context<'_>) -> Result<(), Error> {
        ctx.append_http_header(
            SET_COOKIE,
            format!("{}={}; SameSite=Lax", AUTH_COOKIE_NAME, self.value),
        );

        Ok(())
    }

    /// Load actual session from Redis/Session Store.
    pub async fn load_session(&self, session_store: &RedisSessionStore) -> Result<Session, Error> {
        Ok(session_store
            .load_session(self.value.clone())
            .await
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("Session present but not found on Redis"))?)
    }
}
