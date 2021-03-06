//! Backend GraphQL server for AU Health Dept Wellness App.
//! Developed by AU Capstone group.

#![warn(missing_docs)]

use std::{convert::TryFrom, net::TcpListener, time::Duration};
use argon2::Argon2;
use async_redis_session::RedisSessionStore;

use au_health_backend::{
    configuration::get_configuration,
    domain::user::{NewUser, Role, User, VerifiedNewUser},
    startup::run,
};

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// Entry point to server.
/// 
/// starts up all the databases and everything, makes sure everything is in order to run the app
#[tokio::main]
async fn main() {
    dotenv().ok();

    let configuration = get_configuration().expect("Failed to load configuration");

    let db_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_with(configuration.database.postgres.with_db())
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations. TODO: only do this on dev environment
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    let store = RedisSessionStore::new(configuration.database.redis.with_port())
        .expect("Failed to connect to Redis");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address.clone())
        .unwrap_or_else(|_| panic!("Failed to bind listener on addr {}", address));

    print_init_messages(&address, &configuration.application.graphql.path);

    //function described below
    check_for_root_user(
        &db_pool,
        configuration.application.root_email,
        configuration.application.root_password,
    )
    .await;

    run(listener, db_pool, configuration.application.graphql, store)
        .await
        .unwrap();
}


//shows you in the terminal where the server will be running

fn print_init_messages(address: &str, graphql_path: &str) {
    let nice_link = address.replace("0.0.0.0", "localhost");

    println!("Server running on {}", address);
    println!("GraphQL link: http://{}{}", nice_link, graphql_path)
}

// checks to see if there is a root user (there should be)
//if there is not, it creates one with the given credentials which are taken from the .env file

async fn check_for_root_user(db_pool: &Pool<Postgres>, root_email: String, root_password: String) {
    let user_result = User::query_by_email(db_pool, &root_email).await;

    match user_result {
        Ok(user) => println!("Root user {} already created", user.email),
        Err(_) => {
            let new_user = NewUser {
                email: root_email,
                password: root_password,
            };

            let verified_user = VerifiedNewUser::try_from(new_user)
                .expect("Failed to validate root user information");

            let argon = Argon2::default();

            let user = verified_user
                .register_user(db_pool, &argon)
                .await
                .expect("Failed to create root user")
                .change_role(db_pool, Role::Admin)
                .await
                .expect("Failed to change root user role");

            println!(
                "Root user {} created with environment variable credentials",
                user.email
            )
        }
    }
}
