#[cynic::schema_for_derives(file = r#"tests/gql/schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct HealthCheck {
        pub health_check: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct Me {
        pub me: Option<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Register {
        #[arguments(new_user = NewUser { email: "".to_string(), password: "".to_string(), username: "".to_string() })]
        pub register: User,
    }

    #[derive(cynic::FragmentArguments)]
    pub struct RegisterArguments {
        pub user: NewUser,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Logout {
        pub logout: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Login {
        #[arguments(login_user = LoginUser { password: "".to_string(), username: "".to_string() })]
        pub login: User,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub created_at: DateTime,
        pub id: Uuid,
        pub updated_at: DateTime,
        pub username: String,
    }

    #[derive(cynic::InputObject, Clone, Debug)]
    pub struct NewUser {
        pub email: String,
        pub password: String,
        pub username: String,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct LoginUser {
        pub password: String,
        pub username: String,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTime(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

mod schema {
    cynic::use_schema!(r#"tests/gql/schema.graphql"#);
}
