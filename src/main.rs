//! Backend GraphQL server for AU Health Dept Wellness App.
//! Developed by AU Capstone group.

#![warn(missing_docs)]

use std::time::Duration;

use au_health_backend::{
    configuration::get_configuration, gql::build_schema, routes::build_router,
};

use sqlx::postgres::PgPoolOptions;

/// Entry point to server.
#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to load configuration");

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    let schema = build_schema().data(connection_pool).finish();

    let router = build_router(&configuration, schema);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    print_init_messages(&address, &configuration.application.graphql_path);

    axum::Server::bind(&address.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}

fn print_init_messages(address: &String, graphql_path: &String) {
    let nice_link = address.clone().replace("0.0.0.0", "localhost");

    println!("Server running on {}", address);
    println!("GraphQL link: http://{}{}", nice_link, graphql_path)
}
