use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Response,
};
use axum::{
    extract::Extension,
    handler::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::BoxRoute,
    AddExtensionLayer, Json, Router,
};

use crate::{configuration::GraphQLSettings, gql::schema::GqlSchema};

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

pub fn build_graphql_router(
    configuration: &GraphQLSettings,
    schema: GqlSchema,
) -> Router<BoxRoute> {
    let schema_router = Router::new()
        .route(&configuration.path, post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    if configuration.playground_enabled {
        schema_router
            .route(&configuration.path, get(graphql_playground))
            .boxed()
    } else {
        schema_router
            .route(&configuration.path, get(forbidden_response))
            .boxed()
    }
}
