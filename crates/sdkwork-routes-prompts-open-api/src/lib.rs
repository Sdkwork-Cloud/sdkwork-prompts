mod catalog;
mod http_route_manifest;

use axum::Router;
use sdkwork_prompts_web_context::AppState;

pub use http_route_manifest::open_route_manifest;

pub fn routes() -> Router<AppState> {
    catalog::router()
}

pub fn gateway_mount(state: AppState) -> Router {
    routes().with_state(state)
}
