mod auth;
mod context;
mod dto;
mod handlers;
mod iam;
mod middleware;
mod routes;

use axum::{middleware::from_fn, middleware::from_fn_with_state, routing::get, Router};
use sdkwork_database_ops_http::{attach_ops_routes, BearerTokenOpsAuth, DatabaseOpsHttpState};
use sdkwork_prm_service_host::{default_seed_locale, default_seed_profile, PromptsServiceHost};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub service_host: Arc<PromptsServiceHost>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting SDKWork Prompts API Server...");

    if iam::iam_enabled() {
        sdkwork_iam_web_adapter::prime_signing_master_secret();
        tracing::info!("IAM session resolution enabled");
    }

    let service_host = Arc::new(PromptsServiceHost::new().await);
    let state = AppState {
        service_host: service_host.clone(),
    };

    let ops_auth = Arc::new(BearerTokenOpsAuth::from_env("SDKWORK_ACCESS_TOKEN"));
    let ops_state = DatabaseOpsHttpState::new(
        service_host.database_pool(),
        service_host.database_module(),
        default_seed_locale(),
        default_seed_profile(),
        ops_auth,
    );

    let app = attach_ops_routes(
        Router::new()
            .route("/health", get(handlers::health))
            .merge(routes::build_prm_routes())
            .layer(from_fn(middleware::require_dual_token_auth))
            .layer(from_fn_with_state(state.clone(), iam::resolve_iam_context))
            .with_state(state),
        ops_state,
    )
    .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:8080";
    tracing::info!("Prompts API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
