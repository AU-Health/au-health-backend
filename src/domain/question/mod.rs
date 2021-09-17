use async_graphql::InputObject;

#[derive(InputObject)]
pub struct NewQuestion {
    pub question: String,
    pub category: String,
    pub response_type: String,
    pub responses: Option<Vec<String>>,
}
