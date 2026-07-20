use sdkwork_api_prompts_assembly::assemble_api_router;
use sdkwork_utils_rust::optional::default_if_blank;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let business = assemble_api_router()
        .await
        .expect("prompts API assembly bootstrap failed")
        .router;
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());

    let addr = default_if_blank(
        std::env::var("SDKWORK_PROMPTS_APPLICATION_PUBLIC_INGRESS_BIND")
            .ok()
            .as_deref(),
        "127.0.0.1:8080",
    );
    tracing::info!("sdkwork-api-prompts-standalone-gateway listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
