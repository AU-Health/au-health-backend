mod gql;
mod helpers;
use cynic::{Operation, QueryBuilder};
use gql::gql_schema::queries::HealthCheck;
pub use helpers::*;

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::new().await;

    let query: Operation<HealthCheck> = HealthCheck::build(());

    let response = app.send_graphql_request(query).await;

    assert!(response.health_check);
}
