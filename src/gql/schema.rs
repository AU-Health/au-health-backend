use async_graphql::{
    Context, EmptySubscription, Error, MergedObject, Object, Schema, SchemaBuilder,
};

use super::resolvers::user::{UserMutation, UserQuery};

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

#[derive(Default)]
pub struct HealthCheckQuery;

#[Object]
impl HealthCheckQuery {
    async fn health_check(&self, _ctx: &Context<'_>) -> Result<bool, Error> {
        Ok(true)
    }
}
/// Root for all GraphQL Queries.
#[derive(MergedObject, Default)]
pub struct Query(UserQuery, HealthCheckQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
