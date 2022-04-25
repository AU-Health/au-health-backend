use async_graphql::{Context, Error, Object};

use super::super::context::ParsedContext;
use crate::domain::{
    question::{NewQuestion, Question},
    user::{Role, User},
};

#[derive(Default)]
pub struct QuestionMutation;


//added this to be able to query questions
#[derive(Default)] 
pub struct QuestionQuery;


/// this currently is unreachable. we need to be able to query the questions from the database so that 
/// the front end can serve them to the user, and we can recieve the answers from them.
#[Object]
impl QuestionQuery{
    pub async fn return_questions(
        &self
    ) -> &str{
        print!("TEST");



        return "test"
    }

}


//this lets you add a question, and adds it to the db
#[Object]
impl QuestionMutation {
    pub async fn create_question(
        &self,
        raw_ctx: &Context<'_>,
        #[graphql(desc = "New Question to add")] question: NewQuestion,
    ) -> Result<Question, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        // get the cookie or error out
        let cookie = ctx.get_cookie()?;

        let user_id = ctx.session_manager.user_id(cookie).await?;

        let user = User::query_by_id(ctx.db_pool, user_id).await?;

        user.role.authorized(vec![Role::Admin])?;

        question.save_to_db(ctx.db_pool).await
    }
}
