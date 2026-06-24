mod ai_prompts;
mod app;
mod backend;
mod open;

use axum::Router;
use super::AppState;

pub fn build_prm_routes() -> Router<AppState> {
    Router::new()
        .merge(ai_prompts::router())
        .merge(app::router())
        .merge(backend::router())
        .merge(open::router())
}
