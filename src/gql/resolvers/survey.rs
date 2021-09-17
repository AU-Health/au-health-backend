use async_graphql::{Context, Error, Object};

use crate::{domain::survey::NewSurvey, gql::context::ContextData};

#[derive(Default)]
pub struct SurveyQuery;

#[Object]
impl SurveyQuery {
    pub async fn create_survey(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Survey submission")] survey: NewSurvey,
    ) -> Result<bool, Error> {
        let context = ContextData::new(ctx);

        let cookie = context.session_cookie

        let user_id = context.authorizer.user_id(auth_session_cookie)

        

        match context.session_cookie {
            None => Err(Error::new("Not logged in")),
            Some(cookie) => {
                let user_id = cookie.get_user_id(context.session_store).await?;

                survey.create_survey(context.db_pool, user_id).await?;

                Ok(true)
            }
        }
    }
}
