use async_graphql::{Context, Error, Object, SimpleObject};
use futures::FutureExt;
use sqlx::Connection;

use crate::gql::context::ParsedContext;

#[derive(Default)]
pub struct SystemQuery;

#[derive(SimpleObject)]
pub struct HealthCheck {
    pub database: bool,
    pub redis: bool,
    pub api: bool,
}

#[Object]
impl SystemQuery {
    async fn health_check(&self, raw_ctx: &Context<'_>) -> Result<HealthCheck, Error> {
        let ctx = ParsedContext::new(raw_ctx);

        let db_ping = ctx.db_pool.acquire().await?.detach().ping().await?;

        // let store = ctx.session_manager.store;

        let health_check = HealthCheck {
            database: true,
            redis: true,
            api: true,
        };
        Ok(health_check)
    }
}
