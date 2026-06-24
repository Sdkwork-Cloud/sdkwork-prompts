mod agent_template_store;
mod prompt_ai_store;
mod stable_id;

use sqlx::PgPool;

#[derive(Clone)]
pub struct SqlxPromptAiRepository {
    pool: PgPool,
}

impl SqlxPromptAiRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
