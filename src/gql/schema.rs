use async_graphql::{Context, EmptyMutation, EmptySubscription, Error, Object, Schema};

use crate::auth::{login, User};

/// Root for all GraphQL Queries.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Login to application.
    async fn login(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Username")] username: String,
        #[graphql(desc = "Password")] password: String,
    ) -> Result<User, Error> {
        login(username, password).await
    }
}

pub type GqlSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
