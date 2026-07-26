mod agent_templates;
mod http_route_manifest;
mod prompts;

use axum::Router;
use sdkwork_prompts_web_context::AppState;

pub use http_route_manifest::app_route_manifest;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(prompts::router())
        .merge(agent_templates::router())
}

pub fn gateway_mount(state: AppState) -> Router {
    routes().with_state(state)
}
