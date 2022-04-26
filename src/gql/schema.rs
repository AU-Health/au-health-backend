use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

use super::resolvers::{
    question::{QuestionMutation, QuestionQuery}, // -- will need this once it is initialized (I think? who really knows)
    survey::SurveyMutation,
    system::SystemQuery,
    user::{UserMutation, UserQuery},
};

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}

/// Root for all GraphQL Queries.
#[derive(MergedObject, Default)]
pub struct Query(UserQuery, SystemQuery, QuestionQuery); //Added questionQuery so we can select questions from the db
                                                         //not done yet though, might still need some work. everything else ~should~ be fine...

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, SurveyMutation, QuestionMutation);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
