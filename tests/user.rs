mod gql;
mod helpers;
use std::convert::TryInto;

use argon2::Argon2;
use au_health_backend::domain::{self, user::User, user_registration::VerifiedNewUser};
use cynic::{MutationBuilder, Operation};
use gql::gql_schema::queries::{
    Login, LoginArguments, LoginUser, NewUser, Register, RegisterArguments,
};
use helpers::spawn_app;
pub use helpers::*;
use uuid::Uuid;

use crate::gql::gql_schema::queries::Logout;

#[tokio::test]
async fn register_works() {
    let app = spawn_app().await;

    let user = NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let query: Operation<Register> = Register::build(&RegisterArguments { user: user.clone() });

    let response = app.send_graphql_request(query).await;

    assert_eq!(response.register.email.clone(), user.email.clone());

    assert!(app.auth_cookie_present());

    let user_id = Uuid::parse_str(&response.register.id.0).expect("unable to parse uuid");

    let db_user = sqlx::query_as!(
        User,
        "SELECT id, email, password, created_at, updated_at
     FROM users 
     WHERE id = $1 LIMIT 1",
        user_id
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failure getting user from db");

    assert_eq!(user.email, db_user.email);
}

#[tokio::test]
async fn login_works() {
    let app = spawn_app().await;

    let user = domain::user_registration::NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let verified_user: VerifiedNewUser = user.clone().try_into().expect("cannot verify user");

    verified_user
        .register_user(&app.db_pool, &Argon2::default())
        .await
        .expect("unable to register user");

    let login_user = LoginUser {
        email: user.email,
        password: user.password,
    };

    let query: Operation<Login> = Login::build(&LoginArguments {
        user: login_user.clone(),
    });

    let response = app.send_graphql_request(query).await;

    assert_eq!(response.login.email, login_user.email);

    assert!(app.auth_cookie_present());
}

#[tokio::test]
async fn logout_works() {
    let app = spawn_app().await;

    let user = NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let register_query: Operation<Register> =
        Register::build(&RegisterArguments { user: user.clone() });

    let _response = app.send_graphql_request(register_query).await;

    assert!(app.auth_cookie_present());

    let query: Operation<Logout> = Logout::build(());

    let response = app.send_graphql_request(query).await;

    assert!(response.logout);

    assert!(!app.auth_cookie_present());
}
