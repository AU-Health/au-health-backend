use async_graphql::{Context, Error, Object};

#[derive(Default)]
pub struct QuestionQuery;

#[Object]
impl QuestionQuery {
    pub async fn create_question(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "New Question to add")] question: NewQuestion,
    ) -> Result<bool, Error> {
        let context_data = ContextData::new(ctx);

        Ok(true)
    }
}
