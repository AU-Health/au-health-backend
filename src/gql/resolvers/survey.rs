use async_graphql::{Context, Error, Object};

use crate::{domain::survey::NewSurveyResponse, gql::context::ParsedContext};

#[derive(Default)]
pub struct SurveyMutation;

#[Object]
impl SurveyMutation {
    pub async fn create_survey_response(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "Survey submission")] survey_response: NewSurveyResponse,
    ) -> Result<bool, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        // get the cookie or error out
        let cookie = ctx.get_cookie()?;

        let user_id = ctx.session_manager.user_id(cookie).await?;

        survey_response
            .create_survey_response(ctx.db_pool, user_id)
            .await?;

        Ok(true)
    }
}
