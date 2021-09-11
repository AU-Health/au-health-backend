use std::net::TcpListener;

use argon2::Argon2;
use sqlx::{Pool, Postgres};

use crate::{
    configuration::GraphQLSettings, gql::schema::build_schema, routes::build_graphql_router,
};

pub async fn run(
    listener: TcpListener,
    connection_pool: Pool<Postgres>,
    graphql_configuration: &GraphQLSettings,
) -> Result<(), hyper::Error> {
    let schema = build_schema()
        .data(connection_pool)
        .data(Argon2::default())
        .finish();

    let graphql_router = build_graphql_router(graphql_configuration, schema);

    let server = axum::Server::from_tcp(listener)
        .expect("Failed to bind server to listener")
        .serve(graphql_router.into_make_service());

    server.await
}
