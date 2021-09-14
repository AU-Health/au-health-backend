mod gql;
mod helpers;
use cynic::{Operation, QueryBuilder};
use gql::gql_schema::queries::HealthCheck;
use helpers::spawn_app;
pub use helpers::*;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let query: Operation<HealthCheck> = HealthCheck::build(());

    let response = app.send_graphql_request(query).await;

    assert!(response.health_check);
}
