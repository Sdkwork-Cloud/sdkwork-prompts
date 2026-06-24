pub mod schema;
pub mod repo_impl;
pub mod bootstrap;

use std::sync::Arc;

use sdkwork_intelligence_prm_service::PromptsServiceError;
use sdkwork_id_core::{IdGenerator, SnowflakeIdGenerator};
use sqlx::PgPool;

#[derive(Clone)]
pub struct SqlxPromptsRepository {
    pool: PgPool,
    id_gen: Arc<dyn IdGenerator>,
}

impl SqlxPromptsRepository {
    pub fn new(pool: PgPool) -> Self {
        let node_id = std::env::var("SDKWORK_PROMPTS_SNOWFLAKE_NODE_ID")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        let gen = SnowflakeIdGenerator::new(node_id)
            .expect("invalid SDKWORK_PROMPTS_SNOWFLAKE_NODE_ID");
        Self {
            pool,
            id_gen: Arc::new(gen),
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub(crate) fn next_id(&self) -> Result<i64, PromptsServiceError> {
        let id_str = self
            .id_gen
            .next_id()
            .map_err(|e| PromptsServiceError::internal(e.to_string()))?;
        id_str
            .parse::<i64>()
            .map_err(|_| PromptsServiceError::internal("invalid snowflake id"))
    }

    pub fn new_placeholder() -> Self {
        Self::new(
            PgPool::connect_lazy("postgres://localhost:5432/forum")
                .expect("Failed to create placeholder pool"),
        )
    }
}
