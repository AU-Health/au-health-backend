use async_graphql::{Context, Error, Object};

use crate::{domain::survey::NewSurvey, gql::context::ParsedContext};

#[derive(Default)]
pub struct SurveyMutation;

#[Object]
impl SurveyMutation {
    pub async fn create_survey(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "Survey submission")] survey: NewSurvey,
    ) -> Result<bool, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        // get the cookie or error out
        let cookie = ctx.get_cookie()?;

        let user_id = ctx.session_manager.user_id(cookie).await?;

        survey.create_survey(ctx.db_pool, user_id).await?;

        Ok(true)
    }
}
