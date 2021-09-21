use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

use super::resolvers::{
    question::QuestionMutation,
    survey::SurveyMutation,
    system::SystemQuery,
    user::{UserMutation, UserQuery},
};

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

/// Root for all GraphQL Queries.
#[derive(MergedObject, Default)]
pub struct Query(UserQuery, SystemQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, SurveyMutation, QuestionMutation);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
