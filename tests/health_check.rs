use std::net::TcpListener;

use async_redis_session::RedisSessionStore;
use au_health_backend::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

mod gql;
use cynic::{http::ReqwestExt, MutationBuilder, Operation, QueryBuilder};
use gql::schema::queries::{HealthCheck, NewUser, Register};

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://localhost:{}", port);

    let mut config = get_configuration().expect("Failed to get configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&config.database).await;

    let session_store =
        RedisSessionStore::new("redis://127.0.0.1/").expect("Failed to connect to Redis");

    let server = run(
        listener,
        db_pool.clone(),
        config.application.graphql,
        session_store,
    );

    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let query: Operation<HealthCheck> = HealthCheck::build(());

    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/graphql", &app.address))
        .run_graphql(query)
        .await
        .expect("Failed to send request");

    assert!(response.data.is_some());

    let data = response.data.unwrap();

    assert!(data.health_check);
}

#[tokio::test]
async fn register_works() {
    let app = spawn_app().await;

    let query: Operation<Register> = Register::build(&NewUser {
        email: "mawefwefwefwef7@gmail.com".to_string(),
        username: "zireael".to_string(),
        password: "hunter2".to_string(),
    });

    let client = reqwest::Client::new();
}
