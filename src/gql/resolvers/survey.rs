// If you're confused about async_graphql, check the readme markdown file. There is a link to the library's
// information page there. 
use async_graphql::{Context, Error, Object};

use crate::{domain::survey::NewSurveyResponse, gql::context::ParsedContext};

#[derive(Default)]
pub struct SurveyMutation;

// a survey response contains a vector of answers. this saves those to the db under the user's id (which is logged via cookie)

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

        survey_response.save_to_db(ctx.db_pool, user_id).await?;

        Ok(true)
    }
}
