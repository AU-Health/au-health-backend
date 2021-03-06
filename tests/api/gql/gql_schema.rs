#[cynic::schema_for_derives(file = r#"./tests/api/gql/schema.graphql"#, module = "schema")]
pub mod queries {
    use super::schema;

    //schemas are used in GraphQL to show what can be queried or mutated.
    //queries simply retrieve data, while mutations change, add, or delete it.

    //each of these structs relate to an object that the scheme can call or needs in order to make a mutation
    //for example, this first struct is used as an argument to register a new user,
    //and the struct contains a user object.
    //further down, you can find that the user object holds an email, id, and times.

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct RegisterArguments {
        pub user: NewUser,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct LoginArguments {
        pub user: LoginUser,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NewSurveyResponse {
        pub answers: Vec<NewAnswer>,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NewAnswer {
        pub answer: String,
        pub question_id: Uuid,
    }

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct CreateSurveyResponseArguments {
        pub survey_response: NewSurveyResponse,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Mutation",
        argument_struct = "CreateSurveyResponseArguments"
    )]
    pub struct CreateSurveyResponse {
        #[arguments(survey_response = &args.survey_response)]
        pub create_survey_response: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct Me {
        pub me: Option<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct HealthCheckQuery {
        pub health_check: HealthCheck,
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

    #[derive(cynic::QueryFragment, Debug)]
    pub struct HealthCheck {
        pub api: bool,
        pub database: bool,
        pub redis: bool,
    }

    #[derive(cynic::QueryFragment, Debug, Clone)]
    pub struct User {
        pub email: String,
        pub created_at: DateTime,
        pub id: Uuid,
        pub updated_at: DateTime,
    }

    #[derive(cynic::InputObject, Debug, Clone)]
    pub struct NewUser {
        pub email: String,
        pub password: String,
    }

    #[derive(cynic::InputObject, Debug, Clone)]
    pub struct LoginUser {
        pub email: String,
        pub password: String,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct DateTime(pub String);

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Uuid(pub String);
}

mod schema {
    cynic::use_schema!(r#"./tests/api/gql/schema.graphql"#);
}
