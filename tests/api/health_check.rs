use claim::assert_ok;
use cynic::{Operation, QueryBuilder};

use crate::{gql::gql_schema::queries::HealthCheck, helpers::TestApp};

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::new().await;

    let query: Operation<HealthCheck> = HealthCheck::build(());

    let response = app.send_graphql_request(query).await;

    assert_ok!(&response);

    assert!(&response.unwrap().health_check);
}
