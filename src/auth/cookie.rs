use crate::domain::user::User;
use async_graphql::Error;
use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use headers::Cookie;

pub struct AuthCookies {
    pub cookie_value: Option<String>,
}

#[async_trait]
impl<B> FromRequest<B> for AuthCookies
where
    B: Send,
{
    type Rejection = String;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie_headers = TypedHeader::<Cookie>::from_request(req).await.ok();

        match cookie_headers {
            None => Ok(AuthCookies { cookie_value: None }),
            Some(cookies) => {
                let cookie_value = cookies.get("auth").map(|s| s.to_string().into());
                Ok(AuthCookies { cookie_value })
            }
        }
    }
}

/// Creates user session, and returns the cookie value to append.
pub async fn create_user_session(
    user: &User,
    session_store: &RedisSessionStore,
) -> Result<String, Error> {
    let mut session = Session::new();
    session.insert("user_id", user.id)?;

    let cookie_value = session_store
        .store_session(session)
        .await
        .expect("Session unable to be stored")
        .expect("Cookie value empty");

    Ok(cookie_value)
}
