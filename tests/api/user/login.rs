#[tokio::test]
async fn login_works() {
    let app = TestApp::new().await;

    let user = domain::user::NewUser {
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
