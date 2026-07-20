mod auth;
mod context;
pub mod iam;
pub mod infra_router;
mod middleware;
mod response;
mod routes;

use axum::{middleware::from_fn, middleware::from_fn_with_state, Router};
use sdkwork_prompts_service_host::PromptsServiceHost;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub service_host: Arc<PromptsServiceHost>,
}

pub fn build_prompts_business_router(service_host: Arc<PromptsServiceHost>) -> Router {
    let state = AppState { service_host };
    Router::new()
        .merge(routes::build_prompts_routes())
        .layer(from_fn(middleware::require_dual_token_auth))
        .layer(from_fn_with_state(state.clone(), iam::resolve_iam_context))
        .with_state(state)
}
