use async_graphql::{Context, Error, Object};
use http::header::SET_COOKIE;

use crate::{
    auth::{self, AuthSessionCookie, GetUserId},
    domain::user::{LoginUser, NewUser, User},
    gql::context::ContextData,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let context_data = ContextData::new(ctx);

        match context_data.auth_session_cookie {
            None => Ok(None),
            Some(auth_session_cookie) => {
                let user_id = auth_session_cookie
                    .load_session(context_data.session_store)
                    .await?
                    .ok_or("Session present but not found on Redis")?
                    .get_user_id()
                    .await?;
                let user = User::query_by_id(context_data.db_pool, user_id).await?;

                Ok(Some(user))
            }
        }
    }
}

fn logged_in_error() -> Error {
    Error::new("Already logged in!")
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
        let context_data = ContextData::new(ctx);

        let is_logged_in = context_data
            .auth_session_cookie
            .as_ref()
            .map_or_else(|| false, |_auth| true);

        let user = match is_logged_in {
            true => Err(logged_in_error()),
            false => auth::login_user(context_data.db_pool, context_data.argon2, login_user).await,
        }?;

        let auth_session =
            AuthSessionCookie::create_session(&user, context_data.session_store).await?;

        auth_session.create_cookie(ctx).await?;

        Ok(user)
    }

    async fn register(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "New User information")] new_user: NewUser,
    ) -> Result<User, Error> {
        let context_data = ContextData::new(ctx);

        let is_logged_in = context_data
            .auth_session_cookie
            .as_ref()
            .map_or_else(|| false, |_auth| true);

        let user = match is_logged_in {
            true => Err(logged_in_error()),
            false => auth::register_user(context_data.db_pool, context_data.argon2, new_user).await,
        }?;

        let auth_session =
            AuthSessionCookie::create_session(&user, context_data.session_store).await?;

        auth_session.create_cookie(ctx).await?;

        Ok(user)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(SET_COOKIE, "auth=deleted; Max-Age=-1");
        Ok(true)
    }
}
