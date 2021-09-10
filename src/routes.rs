use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::Extension,
    handler::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::BoxRoute,
    AddExtensionLayer, Json, Router,
};

use crate::{
    configuration::Settings,
    gql::{GqlSchema, QueryRoot},
};

/// initalize GraphQL Playground UI for testing.
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn forbidden_response() -> impl IntoResponse {
    StatusCode::FORBIDDEN
}

/// Processes GraphQL requests.
async fn graphql_handler(schema: Extension<GqlSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub fn build_router(
    configuration: &Settings,
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
) -> Router<BoxRoute> {
    let schema_router = Router::new()
        .route(
            &configuration.application.graphql_path,
            post(graphql_handler),
        )
        .layer(AddExtensionLayer::new(schema));

    if configuration.application.playground_enabled {
        schema_router
            .route(
                &configuration.application.graphql_path,
                get(graphql_playground),
            )
            .boxed()
    } else {
        schema_router
            .route(
                &configuration.application.graphql_path,
                get(forbidden_response),
            )
            .boxed()
    }
}
