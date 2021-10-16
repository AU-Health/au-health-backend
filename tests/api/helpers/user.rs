//! Helpers for tests that need created or logged in users.

use std::convert::TryFrom;

use crate::gql::gql_schema::queries::{Login, LoginArguments, LoginUser};
use argon2::Argon2;
use au_health_backend::domain::{
    self,
    user::{Role, User, VerifiedNewUser},
};
use cynic::{GraphQlError, MutationBuilder, Operation};

use super::TestApp;

impl TestApp {
    pub async fn create_test_user(&self, role: Role, password: String) -> User {
        let user = domain::user::NewUser {
            email: "mw3915a@student.american.edu".to_string(),
            password,
        };

        let verified_user =
            VerifiedNewUser::try_from(user.clone()).expect("Unable to create verified user");

        let user = verified_user
            .register_user(&self.db_pool, &Argon2::default())
            .await
            .expect("unable to register user");

        let updated_user = user.change_role(&self.db_pool, role).await.unwrap();

        updated_user
    }

    pub async fn login_user(
        &self,
        user: User,
        password: String,
    ) -> Result<Login, Vec<GraphQlError>> {
        let login_user = LoginUser {
            email: user.email,
            password,
        };

        let query: Operation<Login> = Login::build(&LoginArguments {
            user: login_user.clone(),
        });

        self.send_graphql_request(query).await
    }

    pub async fn create_and_login_test_user(&self, role: Role) -> Result<User, Vec<GraphQlError>> {
        let password = "hunter2";

        let user = self.create_test_user(role, password.to_string()).await;

        self.login_user(user.clone(), password.to_string()).await?;

        Ok(user)
    }
}
