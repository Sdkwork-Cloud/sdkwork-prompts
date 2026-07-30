//! API assembly bootstrap for sdkwork-prompts.

use std::sync::Arc;

use axum::Router;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_prompts_service_host::PromptsServiceHost;
use sdkwork_prompts_web_context::{AppState, PromptsRequestContext, ResolvedPromptsContext};
pub use sdkwork_web_bootstrap::ApiAssemblyContribution;
use sdkwork_web_bootstrap::{ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::{DomainContextInjector, HttpRouteManifest, WebRequestContext};

pub type ApiAssembly = ApiAssemblyContribution;

#[derive(Clone)]
struct PromptsReadiness {
    pool: DatabasePool,
}

impl ReadinessCheck for PromptsReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        let pool = self.pool.clone();
        Box::pin(async move {
            match pool.test_connection().await {
                Ok(true) => Ok(()),
                Ok(false) => Err("prompts database readiness query returned no row".to_owned()),
                Err(error) => Err(format!("prompts database readiness check failed: {error}")),
            }
        })
    }
}

#[derive(Clone, Default)]
struct PromptsContextInjector;

impl DomainContextInjector for PromptsContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        let Some(principal) = context.principal.as_ref() else {
            return;
        };
        let (Ok(tenant_id), Ok(user_id)) =
            (principal.tenant_id().parse(), principal.user_id().parse())
        else {
            return;
        };
        let organization_id = principal
            .organization_id()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        request
            .extensions_mut()
            .insert(ResolvedPromptsContext(PromptsRequestContext::new(
                tenant_id,
                organization_id,
                user_id,
            )));
    }
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let service_host = PromptsServiceHost::try_new().await?;
    assemble_owner_api_contribution_with_host(service_host)
}

pub async fn assemble_api_router_with_pool(pool: DatabasePool) -> Result<ApiAssembly, String> {
    let service_host = PromptsServiceHost::from_pool(pool).await?;
    assemble_owner_api_contribution_with_host(service_host)
}

fn assemble_owner_api_contribution_with_host(
    service_host: PromptsServiceHost,
) -> Result<ApiAssembly, String> {
    let readiness_check = Arc::new(PromptsReadiness {
        pool: service_host.database_pool(),
    });
    let state = AppState::new(
        service_host.ai_repository(),
        service_host.iam_pool().cloned(),
    );

    let router = Router::new()
        .merge(sdkwork_routes_prompts_app_api::gateway_mount(state.clone()))
        .merge(sdkwork_routes_prompts_backend_api::gateway_mount(
            state.clone(),
        ))
        .merge(sdkwork_routes_prompts_open_api::gateway_mount(state));
    let routes = sdkwork_routes_prompts_app_api::app_route_manifest()
        .routes()
        .iter()
        .chain(sdkwork_routes_prompts_backend_api::backend_route_manifest().routes())
        .chain(sdkwork_routes_prompts_open_api::open_route_manifest().routes())
        .cloned()
        .collect();
    let route_manifest = HttpRouteManifest::from_owned_routes(routes);
    ApiAssemblyContribution::from_manifest(
        "sdkwork-prompts",
        "SDKWork Prompts API",
        router,
        route_manifest,
        vec![Arc::new(PromptsContextInjector)],
        readiness_check,
    )
}

/// Builds the raw Prompts App API for a gateway-owned Web Framework layer.
pub async fn assemble_app_api_contribution() -> Result<ApiAssemblyContribution, String> {
    let service_host = PromptsServiceHost::try_new().await?;
    assemble_app_api_contribution_with_host(service_host)
}

pub async fn assemble_app_api_contribution_with_pool(
    pool: DatabasePool,
) -> Result<ApiAssemblyContribution, String> {
    let service_host = PromptsServiceHost::from_pool(pool).await?;
    assemble_app_api_contribution_with_host(service_host)
}

fn assemble_app_api_contribution_with_host(
    service_host: PromptsServiceHost,
) -> Result<ApiAssemblyContribution, String> {
    let readiness_check = Arc::new(PromptsReadiness {
        pool: service_host.database_pool(),
    });
    let state = AppState::new(
        service_host.ai_repository(),
        service_host.iam_pool().cloned(),
    );
    let route_manifest = sdkwork_routes_prompts_app_api::app_route_manifest();
    let router = sdkwork_routes_prompts_app_api::gateway_mount(state);
    ApiAssemblyContribution::from_manifest(
        "sdkwork-prompts",
        "SDKWork Prompts App API",
        router,
        route_manifest,
        vec![Arc::new(PromptsContextInjector)],
        readiness_check,
    )
}
