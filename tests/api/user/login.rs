use std::convert::TryFrom;

use argon2::Argon2;
use au_health_backend::domain::{self, user::VerifiedNewUser};
use claim::assert_ok;
use cynic::{MutationBuilder, Operation};

use crate::{
    gql::gql_schema::queries::{Login, LoginArguments, LoginUser},
    helpers::TestApp,
};

#[tokio::test]
async fn login_works() {
    let app = TestApp::new().await;

    let user = domain::user::NewUser {
        email: "mw3915a@student.american.edu".to_string(),
        password: "hunter2".to_string(),
    };

    let verified_user =
        VerifiedNewUser::try_from(user.clone()).expect("Unable to create verified user");

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

    assert_ok!(&response);

    assert_eq!(response.unwrap().login.email, login_user.email);

    assert!(app.auth_cookie_present());
}
