use au_health_backend::domain::{self, user::Role};
use claim::assert_ok;
use cynic::{MutationBuilder, Operation};

use crate::{
    gql::gql_schema::queries,
    gql::gql_schema::queries::{CreateSurveyResponse, CreateSurveyResponseArguments},
    helpers::TestApp,
};

#[tokio::test]
async fn can_create_survey_response() {
    let app = TestApp::new().await;

    let _user = app.create_and_login_test_user(Role::User).await.unwrap();

    let new_question = domain::question::NewQuestion {
        question: "What is your name?".to_string(),
        category: "Basic Information".to_string(),
        response_type: "text".to_string(),
        responses: None,
    };

    let question = new_question.save_to_db(&app.db_pool).await.unwrap();

    let new_answer = queries::NewAnswer {
        question_id: queries::Uuid(question.id.to_string()),
        answer: "Matt".to_string(),
    };

    let new_survey_response = queries::NewSurveyResponse {
        answers: vec![new_answer],
    };

    let query: Operation<CreateSurveyResponse> =
        CreateSurveyResponse::build(&CreateSurveyResponseArguments {
            survey_response: new_survey_response,
        });

    let response = app.send_graphql_request(query).await;

    assert_ok!(&response);
}
