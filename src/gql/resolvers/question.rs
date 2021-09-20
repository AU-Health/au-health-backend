use async_graphql::{Context, Error, Object};

use super::super::context::ParsedContext;
use crate::domain::{
    question::NewQuestion,
    user::{Role, User},
};

#[derive(Default)]
pub struct QuestionQuery;

#[Object]
impl QuestionQuery {
    pub async fn create_question(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "New Question to add")] _question: NewQuestion,
    ) -> Result<bool, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        // get the cookie or error out
        let cookie = ctx.get_cookie()?;

        let user_id = ctx.session_manager.user_id(cookie).await?;

        let user = User::query_by_id(ctx.db_pool, user_id).await?;

        user.role.authorized(vec![Role::Admin])?;

        Ok(true)
    }
}
