use cynic::{MutationBuilder, Operation};

use crate::{
    gql::gql_schema::queries::{Logout, NewUser, Register, RegisterArguments},
    helpers::TestApp,
};

#[tokio::test]
async fn logout_works() {
    let app = TestApp::new().await;

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
