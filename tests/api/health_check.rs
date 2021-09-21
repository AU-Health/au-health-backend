use claim::assert_ok;
use cynic::{Operation, QueryBuilder};

use crate::{gql::gql_schema::queries::HealthCheckQuery, helpers::TestApp};

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::new().await;

    let query: Operation<HealthCheckQuery> = HealthCheckQuery::build(());

    let response = app.send_graphql_request(query).await;

    assert_ok!(&response);

    let health_check = &response.unwrap().health_check;

    assert!(health_check.api);
    assert!(health_check.database);
    assert!(health_check.redis);
}
