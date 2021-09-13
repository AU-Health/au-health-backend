//! Backend GraphQL server for AU Health Dept Wellness App.
//! Developed by AU Capstone group.

#![warn(missing_docs)]

use std::{net::TcpListener, time::Duration};

use async_redis_session::RedisSessionStore;
use au_health_backend::{configuration::get_configuration, startup::run};

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

    let store = RedisSessionStore::new("redis://127.0.0.1/").expect("Failed to connect to Redis");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address.clone())
        .unwrap_or_else(|_| panic!("Failed to bind listener on addr {}", address));

    print_init_messages(&address, &configuration.application.graphql.path);

    run(
        listener,
        connection_pool,
        &configuration.application.graphql,
        store,
    )
    .await
    .unwrap();
}

fn print_init_messages(address: &str, graphql_path: &str) {
    let nice_link = address.replace("0.0.0.0", "localhost");

    println!("Server running on {}", address);
    println!("GraphQL link: http://{}{}", nice_link, graphql_path)
}
