use sdkwork_database_ops_http::{attach_ops_routes, default_ops_auth, DatabaseOpsHttpState};
use sdkwork_prompts_service_host::{default_seed_locale, default_seed_profile, PromptsServiceHost};
use sdkwork_prompts_standalone_gateway::{build_prompts_business_router, iam, infra_router};
use sdkwork_web_bootstrap::ServiceRouterConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting SDKWork Prompts API Server...");

    if iam::iam_enabled() {
        tracing::info!("IAM session resolution enabled");
    }

    let service_host = Arc::new(PromptsServiceHost::new().await);
    let ops_auth = default_ops_auth();
    let ops_state = DatabaseOpsHttpState::new(
        service_host.database_pool(),
        service_host.database_module(),
        default_seed_locale(),
        default_seed_profile(),
        ops_auth,
    );

    let app = infra_router::mount_service_routes(
        attach_ops_routes(
            build_prompts_business_router(service_host.clone()),
            ops_state,
        )
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_PROMPTS_ENVIRONMENT"],
            &[
                "SDKWORK_PROMPTS_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        )),
        ServiceRouterConfig::default().with_always_ready(),
    );

    let addr = "0.0.0.0:8080";
    tracing::info!("Prompts API server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
