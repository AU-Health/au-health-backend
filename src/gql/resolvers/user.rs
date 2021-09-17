use std::convert::TryInto;

use async_graphql::{Context, Error, Object};
use http::header::SET_COOKIE;

use crate::{
    auth::AuthSessionCookie,
    domain::{
        user::{LoginUser, User},
        user_registration::{NewUser, VerifiedNewUser},
    },
    gql::context::ContextData,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Checks for the currently logged in user by parsing the cookie.
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let context_data = ContextData::new(ctx);

        match context_data.auth_session_cookie {
            None => Ok(None),
            Some(auth_session_cookie) => {
                let user_id = auth_session_cookie
                    .get_user_id(context_data.session_store)
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
    /// Logs a user in. Sets cookie on their client.
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
            false => {
                login_user
                    .login_user(context_data.db_pool, context_data.argon2)
                    .await
            }
        }?;

        let auth_session = AuthSessionCookie::new(&user, context_data.session_store).await?;

        auth_session.create_cookie(ctx).await?;

        Ok(user)
    }

    /// Registers a new user. Sets a cookie on their client, as if they logged in.
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
            false => {
                let verified_user: VerifiedNewUser = new_user.try_into()?;

                verified_user
                    .register_user(context_data.db_pool, context_data.argon2)
                    .await
            }
        }?;

        let auth_session = AuthSessionCookie::new(&user, context_data.session_store).await?;

        auth_session.create_cookie(ctx).await?;

        Ok(user)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(SET_COOKIE, "auth=deleted; Max-Age=-1");
        Ok(true)
    }
}
