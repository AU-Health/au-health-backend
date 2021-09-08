//! Backend GraphQL server for AU Health Dept Wellness App.
//! Developed by AU Capstone group.

#![warn(missing_docs)]
mod auth;
mod gql;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::Extension,
    handler::get,
    response::{Html, IntoResponse},
    AddExtensionLayer, Json, Router,
};
use gql::{GqlSchema, QueryRoot};

/// initalize GraphQL Playground UI for testing.
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

/// Processes GraphQL requests.
async fn graphql_handler(schema: Extension<GqlSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

/// Entry point to server.
#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let router = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
