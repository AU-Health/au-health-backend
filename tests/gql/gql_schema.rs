#[cynic::schema_for_derives(file = r#"tests/gql/schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RegisterArguments {
        pub user: NewUser,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct LoginArguments {
        pub user: LoginUser,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct Me {
        pub me: Option<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct HealthCheck {
        pub health_check: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "RegisterArguments")]
    pub struct Register {
        #[arguments(new_user = &args.user)]
        pub register: User,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Logout {
        pub logout: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", argument_struct = "LoginArguments")]
    pub struct Login {
        #[arguments(login_user = &args.user)]
        pub login: User,
    }

    #[derive(cynic::QueryFragment, Debug, Clone)]
    pub struct User {
        pub created_at: DateTime,
        pub id: Uuid,
        pub updated_at: DateTime,
        pub username: String,
    }

    #[derive(cynic::InputObject, Debug, Clone)]
    pub struct NewUser {
        pub email: String,
        pub username: String,
        pub password: String,
    }

    #[derive(cynic::InputObject, Debug, Clone)]
    pub struct LoginUser {
        pub username: String,
        pub password: String,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTime(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

mod schema {
    cynic::use_schema!(r#"tests/gql/schema.graphql"#);
}
