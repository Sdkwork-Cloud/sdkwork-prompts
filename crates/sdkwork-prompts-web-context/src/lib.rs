mod auth;
mod context;
mod iam;
mod middleware;
mod response;

use std::sync::Arc;

use sdkwork_intelligence_prompts_ai_contract::PromptAiRepository;
use sqlx::PgPool;

pub use context::{PromptsCtx, PromptsRequestContext, ResolvedPromptsContext};
pub use iam::{iam_enabled, resolve_iam_context};
pub use middleware::require_dual_token_auth;
pub use response::{
    anonymous_ok_json, anonymous_prompt_error, anonymous_trace_id, created_json,
    cursor_page_info, map_prompt_error, offset_page_info, ok_json, page_data, resource_data,
    status_problem,
};

#[derive(Clone)]
pub struct AppState {
    repository: Arc<dyn PromptAiRepository>,
    iam_pool: Option<PgPool>,
}

impl AppState {
    pub fn new(repository: Arc<dyn PromptAiRepository>, iam_pool: Option<PgPool>) -> Self {
        Self {
            repository,
            iam_pool,
        }
    }

    pub fn ai_repository(&self) -> &dyn PromptAiRepository {
        self.repository.as_ref()
    }

    pub fn iam_pool(&self) -> Option<&PgPool> {
        self.iam_pool.as_ref()
    }
}
