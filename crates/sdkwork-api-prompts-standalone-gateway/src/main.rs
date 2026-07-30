use sdkwork_api_prompts_assembly::assemble_api_router;
use sdkwork_utils_rust::optional::default_if_blank;
use sdkwork_web_bootstrap::ComposedApiAssembly;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let assembly = assemble_api_router()
        .await
        .expect("prompts API assembly bootstrap failed");
    let manifest = assembly.route_manifest.clone();
    let resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await;
    let framework =
        sdkwork_iam_web_adapter::build_web_framework_builder(resolver, manifest, Vec::new());
    let app = ComposedApiAssembly::try_compose("SDKWork Prompts API", vec![assembly])
        .expect("compose prompts API contribution")
        .into_hosted(framework)
        .router;

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
