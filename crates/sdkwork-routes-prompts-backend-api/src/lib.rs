mod http_route_manifest;
mod prompts;

use axum::Router;
use sdkwork_prompts_web_context::AppState;

pub use http_route_manifest::backend_route_manifest;

pub fn routes() -> Router<AppState> {
    prompts::router()
}

pub fn gateway_mount(state: AppState) -> Router {
    routes().with_state(state)
}
