use async_graphql::{Context, Error, Object};
use chrono::Utc;
use http::header::SET_COOKIE;
use uuid::Uuid;

use crate::{
    auth::{self, create_user_session},
    domain::user::{LoginUser, NewUser, User},
    gql::context::get_data_from_ctx,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me(&self, _ctx: &Context<'_>) -> Result<User, Error> {
        Ok(User {
            id: Uuid::new_v4(),
            username: "test".to_owned(),
            password: "1234".to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[derive(Default)]
pub struct UserMutation;

fn logged_in_error() -> Error {
    Error::new("Already logged in!")
}

#[Object]
impl UserMutation {
    async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Login User credentials")] login_user: LoginUser,
    ) -> Result<User, Error> {
        let context_data = get_data_from_ctx(ctx).await;

        let user = match context_data.auth_cookie.cookie_value.is_some() {
            true => Err(logged_in_error()),
            false => auth::login_user(context_data.db_pool, context_data.argon2, login_user).await,
        }?;

        let cookie_value = create_user_session(&user, context_data.session_store).await?;

        ctx.append_http_header(
            SET_COOKIE,
            format!("auth={}; SameSite=Lax", cookie_value).to_string(),
        );

        Ok(user)
    }

    async fn register(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "New User information")] new_user: NewUser,
    ) -> Result<User, Error> {
        let context_data = get_data_from_ctx(ctx).await;

        let user = match context_data.auth_cookie.cookie_value.is_some() {
            true => Err(logged_in_error()),
            false => auth::register_user(context_data.db_pool, context_data.argon2, new_user).await,
        }?;

        let cookie_value = create_user_session(&user, context_data.session_store).await?;

        ctx.append_http_header(
            SET_COOKIE,
            format!("auth={}; SameSite=Lax", cookie_value).to_string(),
        );

        Ok(user)
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(SET_COOKIE, "auth=deleted; Max-Age=-1");
        Ok(true)
    }
}
