use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{
    extract::Extension,
    handler::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use headers::{AccessControlAllowOrigin, HeaderMapExt};
use http::Response;
use hyper::Body;

use crate::{configuration::GraphQlSettings, gql::schema::GqlSchema, session::SessionCookie};

/// initalize GraphQL Playground UI for testing.
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

/// Processes GraphQL requests.
async fn graphql_handler(
    schema: Extension<GqlSchema>,
    graphql_req: GraphQLRequest,
    auth_cookies: Option<SessionCookie>,
) -> Response<Body> {
    let gql_resp: GraphQLResponse = schema
        .execute(graphql_req.into_inner().data(auth_cookies))
        .await
        .into();

    let mut response = gql_resp.into_response();

    response
        .headers_mut()
        .typed_insert(AccessControlAllowOrigin::ANY);

    response
}

async fn forbidden_response() -> impl IntoResponse {
    StatusCode::FORBIDDEN
}

pub fn build_graphql_router(configuration: GraphQlSettings, schema: GqlSchema) -> Router<BoxRoute> {
    let schema_router = Router::new()
        .route(&configuration.path, post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    if configuration.playground_enabled {
        schema_router
            .route(&configuration.path, get(graphql_playground))
            .boxed()
    } else {
        schema_router
            .route(&configuration.path, get(forbidden_response))
            .boxed()
    }
}
