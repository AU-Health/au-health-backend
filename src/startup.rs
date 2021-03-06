use std::net::TcpListener;

use argon2::Argon2;
use async_redis_session::RedisSessionStore;
use sqlx::{Pool, Postgres};

use crate::{
    configuration::GraphQlSettings, gql::schema::build_schema, routes::build_graphql_router,
};

/// Builds GraphQL Schema and runs Axum server
pub async fn run(
    listener: TcpListener,
    connection_pool: Pool<Postgres>,
    graphql_configuration: GraphQlSettings,
    session_store: RedisSessionStore,
) -> Result<(), hyper::Error> {
    let schema = build_schema()
        .data(connection_pool)
        .data(session_store)
        .data(Argon2::default())
        .finish();

    let graphql_router = build_graphql_router(graphql_configuration, schema);

    // TCP connection with axum server
    let server = axum::Server::from_tcp(listener)
        .expect("Failed to bind server to listener")
        .serve(graphql_router.into_make_service());

    server.await
}
