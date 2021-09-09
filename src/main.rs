//! Backend GraphQL server for AU Health Dept Wellness App.
//! Developed by AU Capstone group.

#![warn(missing_docs)]

use std::{future::Future, time::Duration};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use au_health_backend::{
    configuration::{self, get_configuration},
    gql::{GqlSchema, QueryRoot},
};
use axum::{
    extract::Extension,
    handler::get,
    http::StatusCode,
    response::{Html, IntoResponse},
    AddExtensionLayer, Json, Router,
};
use sqlx::postgres::PgPoolOptions;

/// initalize GraphQL Playground UI for testing.
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn forbidden_response() -> impl IntoResponse {
    StatusCode::FORBIDDEN
}

/// Processes GraphQL requests.
async fn graphql_handler(schema: Extension<GqlSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

/// Entry point to server.
#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to load configuration");

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(connection_pool)
        .finish();

    let router = Router::new()
        .route(
            &configuration.application.graphql_path,
            get(graphql_playground).post(graphql_handler),
        )
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
