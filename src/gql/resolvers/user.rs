use std::convert::TryInto;

use async_graphql::{Context, Error, Object};
use http::header::SET_COOKIE;

use crate::{
    domain::user::{LoginUser, NewUser, User, VerifiedNewUser},
    gql::context::ParsedContext,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Checks for the currently logged in user by parsing the cookie.
    async fn me(&self, raw_ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        let cookie = ctx.get_cookie();

        match cookie {
            Err(_) => Ok(None),
            Ok(cookie) => {
                let user_id = ctx.session_manager.user_id(cookie).await?;
                let user = User::query_by_id(ctx.db_pool, user_id).await?;

                Ok(Some(user))
            }
        }
    }
}

fn logged_in_err() -> Error {
    Error::new("Already logged in")
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Logs a user in. Sets cookie on their client.
    async fn login(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "Login User credentials")] login_user: LoginUser,
    ) -> Result<User, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        if ctx.get_cookie().is_ok() {
            return Err(logged_in_err());
        }

        let user = login_user.login_user(ctx.db_pool, ctx.argon2).await?;

        ctx.session_manager
            .create_session(&user)
            .await?
            .set_cookie(raw_ctx)
            .await?;

        Ok(user)
    }

    /// Registers a new user. Sets a cookie on their client, as if they logged in.
    async fn register(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "New User information")] new_user: NewUser,
    ) -> Result<User, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        if ctx.get_cookie().is_ok() {
            return Err(logged_in_err());
        }

        let verified_user: VerifiedNewUser = new_user.try_into()?;

        let user = verified_user.register_user(ctx.db_pool, ctx.argon2).await?;

        ctx.session_manager
            .create_session(&user)
            .await?
            .set_cookie(raw_ctx)
            .await?;

        Ok(user)
    }

    // When the user logs out, thje cookie will take note and remove the user's saved credentials
    // From its queries
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(SET_COOKIE, "auth=deleted; Max-Age=-1");
        Ok(true)
    }
}
