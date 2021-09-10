use async_graphql::{
    Context, EmptySubscription, Error, InputObject, Object, Schema, SchemaBuilder,
};
use sqlx::{Pool, Postgres};

use crate::auth;

pub fn build_schema() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}

#[derive(InputObject)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// Root for all GraphQL Queries.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self, _ctx: &Context<'_>) -> Result<bool, Error> {
        Ok(true)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Login to application.
    async fn login(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Username")] username: String,
        #[graphql(desc = "Password")] password: String,
    ) -> Result<auth::User, Error> {
        auth::login_user(username, password).await
    }

    async fn register(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "New User information")] new_user: NewUser,
    ) -> Result<auth::User, Error> {
        let db_pool = ctx.data::<Pool<Postgres>>().unwrap();

        auth::register_user(db_pool, new_user).await
    }
}

pub type GqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
