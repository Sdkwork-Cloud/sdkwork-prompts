mod catalog;

use axum::Router;
use sdkwork_prompts_web_context::AppState;

pub fn routes() -> Router<AppState> {
    catalog::router()
}

pub fn gateway_mount(state: AppState) -> Router {
    routes().with_state(state)
}
