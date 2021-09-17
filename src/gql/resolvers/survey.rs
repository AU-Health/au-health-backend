use async_graphql::{Context, Error, Object};

use crate::{domain::survey::NewSurvey, gql::context::ContextData};

#[derive(Default)]
pub struct SurveyQuery;

#[Object]
impl SurveyQuery {
    async fn create_survey(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Survey submission")] survey: NewSurvey,
    ) -> Result<bool, Error> {
        let context_data = ContextData::new(ctx);

        match context_data.auth_session_cookie {
            None => Err(Error::new("Not logged in")),
            Some(cookie) => {
                let user_id = cookie.get_user_id(context_data.session_store).await?;

                survey.create_survey(context_data.db_pool, user_id).await?;

                Ok(true)
            }
        }
    }
}
