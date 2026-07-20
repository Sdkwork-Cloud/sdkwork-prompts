mod ai_app_agent_templates;
mod ai_app_prompts;
mod ai_prompts;
mod open;

use axum::Router;

use super::AppState;

pub fn build_prompts_routes() -> Router<AppState> {
    Router::new()
        .merge(ai_prompts::router())
        .merge(ai_app_prompts::router())
        .merge(ai_app_agent_templates::router())
        .merge(open::router())
}
