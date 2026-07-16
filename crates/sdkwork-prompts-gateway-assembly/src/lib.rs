//! Business-only gateway assembly for SDKWork Prompts.

use sdkwork_prompts_service_host::PromptsServiceHost;
use sdkwork_prompts_standalone_gateway::build_prompts_business_router;
use std::sync::Arc;

pub struct ApplicationAssembly {
    pub router: axum::Router,
    pub service_host: Arc<PromptsServiceHost>,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let service_host = Arc::new(PromptsServiceHost::try_new().await?);
    let router = build_prompts_business_router(service_host.clone());
    Ok(ApplicationAssembly {
        router,
        service_host,
    })
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}

pub fn assembly_route_count() -> usize {
    0
}
