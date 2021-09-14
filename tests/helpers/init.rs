use std::net::TcpListener;

use async_redis_session::RedisSessionStore;
use au_health_backend::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};
use reqwest::{cookie::Jar, Client};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Arc;
use uuid::Uuid;

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
    pub cookie_jar: Arc<Jar>,
    pub client: Client,
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
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

    let cookie_jar = Arc::new(Jar::default());

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to create reqwest client");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool,
        client,
        cookie_jar,
    }
}
